#!/usr/bin/env python3
"""Explicit offline dependency checks; receipt pins come from a trusted preparer.

This is a gate tripwire, not authentication against a malicious shell owner.
The preparer must fetch into an empty cache and pin its receipt out of band.
"""
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import subprocess
import sys
import time
import tomllib

MAX_AGE = 86400
INDEX = 'registry/index/index.crates.io-1949cf8c6b5b557f'
ARCHIVES = 'registry/cache/index.crates.io-1949cf8c6b5b557f'
ADVISORIES = 'advisory-dbs/advisory-db-3157b0e258782691'
ROOTS = (INDEX, ARCHIVES, ADVISORIES)
MANIFEST_ENV = 'RIKURI_OFFLINE_CACHE_MANIFEST'
PIN_ENV = 'RIKURI_OFFLINE_CACHE_SHA256'


class InvalidCache(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise InvalidCache(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, 'duplicate JSON key')
        result[key] = value
    return result


def regular(path):
    # Reject symlinks in every component, including the cache root.
    for parent in [path, *path.parents]:
        require(not parent.is_symlink(), 'symlink in input path')
    require(stat.S_ISREG(path.stat().st_mode), 'input is not a regular file')
    require(path.stat().st_size <= 33554432, 'input file too large')
    return path.read_bytes()


def fresh(timestamp, now):
    require(type(timestamp) is int, 'fetch time must be integer UTC seconds')
    require(0 <= now - timestamp < MAX_AGE, 'cache fetch time stale or future-dated')


def index_key(name):
    if len(name) == 1:
        return '1/' + name
    if len(name) == 2:
        return '2/' + name
    if len(name) == 3:
        return '3/' + name[0] + '/' + name
    return name[:2] + '/' + name[2:4] + '/' + name


def validate(repo, cargo, manifest_path, pin, now):
    require(re.fullmatch('[0-9a-f]{64}', pin) is not None, 'invalid receipt pin')
    raw = regular(manifest_path)
    require(digest(raw) == pin, 'receipt checksum mismatch')
    receipt = json.loads(raw, object_pairs_hook=unique_object)
    require(set(receipt) == {'schema_version', 'lock_sha256', 'policy_sha256',
                             'advisory', 'registry', 'files'}, 'invalid receipt fields')
    require(type(receipt['schema_version']) is int and receipt['schema_version'] == 1,
            'unsupported receipt version')
    lock_bytes = regular(repo / 'Cargo.lock')
    policy_bytes = regular(repo / 'deny.toml')
    require(receipt['lock_sha256'] == digest(lock_bytes), 'lockfile binding mismatch')
    require(receipt['policy_sha256'] == digest(policy_bytes), 'policy binding mismatch')
    policy = tomllib.loads(policy_bytes.decode())
    adv_policy = policy['advisories']
    require(adv_policy.get('maximum-db-staleness') == 'P1D', 'advisory staleness policy mismatch')
    require(adv_policy.get('yanked') == 'deny' and not adv_policy.get('disable-yank-checking', False),
            'yank checking must remain enabled and deny')
    advisory = receipt['advisory']
    registry = receipt['registry']
    require(set(advisory) == {'origin', 'commit', 'fetched_at'}, 'invalid advisory provenance')
    require(set(registry) == {'origin', 'fetched_at'}, 'invalid registry provenance')
    require(advisory['origin'] == 'https://github.com/RustSec/advisory-db', 'wrong advisory origin')
    require(registry['origin'] == 'https://index.crates.io/', 'wrong registry origin')
    require(re.fullmatch('[0-9a-f]{40}', advisory['commit']) is not None, 'invalid advisory commit')
    fresh(advisory['fetched_at'], now)
    fresh(registry['fetched_at'], now)
    require(not (cargo / 'config').exists() and not (cargo / 'config.toml').exists(),
            'prepared cargo home must not contain configuration overrides')
    files = receipt['files']
    require(isinstance(files, dict) and 0 < len(files) <= 20000, 'invalid cache inventory')
    total = 0
    for name, entry in files.items():
        relative = PurePosixPath(name)
        require(not relative.is_absolute() and '..' not in relative.parts and str(relative) == name,
                'unsafe cache path')
        require(any(name.startswith(root + '/') for root in ROOTS), 'unexpected cache tree')
        require(set(entry) == {'sha256', 'bytes'} and type(entry['bytes']) is int and entry['bytes'] >= 0,
                'invalid cache file metadata')
        total += entry['bytes']
        require(total <= 536870912, 'cache inventory too large')
        data = regular(cargo / name)
        require(len(data) == entry['bytes'] and digest(data) == entry['sha256'], 'cache file checksum mismatch')
    actual = set()
    for root in ROOTS:
        folder = cargo / root
        require(folder.is_dir() and not folder.is_symlink(), 'missing cache tree')
        for path in folder.rglob('*'):
            require(not path.is_symlink(), 'symlink in cache inventory')
            if path.is_dir():
                continue
            require(path.is_file(), 'special file in cache inventory')
            actual.add(path.relative_to(cargo).as_posix())
    require(actual == set(files), 'cache inventory has missing or extra files')
    # Bind the advisory claim to the actual checked-out ref without executing Git or hooks.
    head = regular(cargo / ADVISORIES / '.git/HEAD').decode().strip()
    if head == 'ref: refs/heads/main':
        head = regular(cargo / ADVISORIES / '.git/refs/heads/main').decode().strip()
    require(head == advisory['commit'], 'advisory checkout commit mismatch')
    require(any(n.startswith(ADVISORIES + '/crates/') and n.endswith('.md') for n in files),
            'advisory database content missing')
    config = json.loads(regular(cargo / INDEX / 'config.json'))
    require(config.get('dl') == 'https://static.crates.io/crates', 'wrong crate download origin')
    packages = tomllib.loads(lock_bytes.decode())['package']
    count = 0
    for package in packages:
        if 'source' not in package:
            continue
        require(package['source'] == 'registry+https://github.com/rust-lang/crates.io-index',
                'offline receipt supports only the official crates.io registry')
        name, version = package['name'], package['version']
        archive = ARCHIVES + '/' + name + '-' + version + '.crate'
        require(archive in files and files[archive]['sha256'] == package['checksum'],
                'locked crate archive missing or mismatched')
        index = INDEX + '/.cache/' + index_key(name)
        require(index in files, 'locked crate index entry missing')
        versions = []
        for part in regular(cargo / index).split(b'\0'):
            if part.startswith(b'{'):
                item = json.loads(part)
                if item.get('vers') == version:
                    versions.append(item)
        require(len(versions) == 1 and versions[0].get('name') == name
                and versions[0].get('cksum') == package['checksum']
                and type(versions[0].get('yanked')) is bool, 'locked crate index metadata invalid')
        count += 1
    return {'offline_cache': 'PASS', 'receipt_sha256': pin, 'locked_crates': count,
            'advisory_commit': advisory['commit'], 'advisory_fetched_at': advisory['fetched_at'],
            'registry_fetched_at': registry['fetched_at'], 'verified_at': now, 'maximum_age_seconds': MAX_AGE}


def main():
    repo = Path(__file__).resolve().parents[2]
    manifest = os.environ.get(MANIFEST_ENV)
    pin = os.environ.get(PIN_ENV)
    if MANIFEST_ENV not in os.environ and PIN_ENV not in os.environ:
        return subprocess.call(['cargo', 'deny', '--locked', 'check'], cwd=repo)
    try:
        require(bool(manifest) and bool(pin), 'offline mode requires receipt and trusted pin')
        require(bool(os.environ.get('CARGO_HOME')), 'offline mode requires explicit CARGO_HOME')
        cargo = Path(os.environ['CARGO_HOME'])
        require(cargo.is_absolute() and Path(manifest).is_absolute(), 'offline paths must be absolute')
        evidence = validate(repo, cargo, Path(manifest), pin, int(time.time()))
        print(json.dumps(evidence, sort_keys=True), flush=True)
    except (ValueError, OSError, KeyError, TypeError) as error:
        print('OFFLINE CACHE REFUSED: ' + str(error), file=sys.stderr)
        return 2
    return subprocess.call(['cargo', 'deny', '--locked', '--offline', 'check'], cwd=repo)


if __name__ == '__main__':
    sys.exit(main())
