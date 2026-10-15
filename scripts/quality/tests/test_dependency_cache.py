"""Refusal and command-routing coverage, run only in the admitted test layer."""
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'dependency-check.py'
spec = importlib.util.spec_from_file_location('dependency_check', SCRIPT)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class CacheTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.repo = self.root / 'repo'
        self.cargo = self.root / 'cargo'
        self.repo.mkdir()
        self.cargo.mkdir()
        self.now = 2000000000
        self.commit = 'a' * 40
        self.archive = module.ARCHIVES + '/demo-1.0.0.crate'
        checksum = module.digest(b'crate')
        (self.repo / 'Cargo.lock').write_text('version = 4\n[[package]]\nname = "demo"\nversion = "1.0.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "' + checksum + '"\n')
        (self.repo / 'deny.toml').write_text('[advisories]\nmaximum-db-staleness = "P1D"\nyanked = "deny"\n')
        self.inputs = {
            self.archive: b'crate',
            module.INDEX + '/config.json': json.dumps({'dl': 'https://static.crates.io/crates'}).encode(),
            module.INDEX + '/.cache/de/mo/demo': b'\x03\x02\0etag\0' + json.dumps({'name': 'demo', 'vers': '1.0.0', 'cksum': checksum, 'yanked': False}).encode() + b'\0',
            module.ADVISORIES + '/.git/HEAD': b'ref: refs/heads/main\n',
            module.ADVISORIES + '/.git/refs/heads/main': (self.commit + '\n').encode(),
            module.ADVISORIES + '/crates/demo/RUSTSEC-2000-0001.md': b'```toml\n[advisory]\n```\n# Fixture advisory\n',
        }
        for name, data in self.inputs.items():
            path = self.cargo / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        self.receipt = {'schema_version': 1,
                        'lock_sha256': module.digest((self.repo / 'Cargo.lock').read_bytes()),
                        'policy_sha256': module.digest((self.repo / 'deny.toml').read_bytes()),
                        'advisory': {'origin': 'https://github.com/RustSec/advisory-db', 'commit': self.commit, 'fetched_at': self.now - 1},
                        'registry': {'origin': 'https://index.crates.io/', 'fetched_at': self.now - 1},
                        'files': {name: {'bytes': len(data), 'sha256': module.digest(data)} for name, data in self.inputs.items()}}
        self.manifest = self.root / 'receipt.json'

    def seal(self):
        self.manifest.write_text(json.dumps(self.receipt))
        return module.digest(self.manifest.read_bytes())

    def validate(self):
        return module.validate(self.repo, self.cargo, self.manifest, self.seal(), self.now)

    def refuse(self, pattern):
        with self.assertRaisesRegex((ValueError, OSError), pattern):
            self.validate()

    def test_valid_and_age_boundary(self):
        for age in (0, 86399):
            for kind in ('registry', 'advisory'):
                self.receipt[kind]['fetched_at'] = self.now - age
            self.assertEqual(self.validate()['locked_crates'], 1)

    def test_stale_future_and_invalid_time(self):
        for kind in ('registry', 'advisory'):
            for age in (86400, 86401, -1):
                with self.subTest(kind=kind, age=age):
                    self.receipt[kind]['fetched_at'] = self.now - age
                    self.refuse('stale or future')
            self.receipt[kind]['fetched_at'] = True
            self.refuse('integer UTC')
            self.receipt[kind]['fetched_at'] = self.now - 1

    def test_pin_and_duplicate_keys(self):
        pin = self.seal()
        with self.assertRaisesRegex(ValueError, 'checksum mismatch'):
            module.validate(self.repo, self.cargo, self.manifest, 'b' * 64, self.now)
        self.manifest.write_text('{"schema_version":1,"schema_version":1}')
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            module.validate(self.repo, self.cargo, self.manifest, module.digest(self.manifest.read_bytes()), self.now)
        self.assertNotEqual(pin, module.digest(self.manifest.read_bytes()))

    def test_bindings_and_origin(self):
        original = copy.deepcopy(self.receipt)
        for key in ('lock_sha256', 'policy_sha256'):
            self.receipt[key] = 'f' * 64
            self.refuse('binding mismatch')
            self.receipt = copy.deepcopy(original)
        for kind in ('registry', 'advisory'):
            self.receipt[kind]['origin'] = 'https://example.invalid/'
            self.refuse('wrong .* origin')
            self.receipt = copy.deepcopy(original)
        self.receipt['advisory']['commit'] = 'b' * 40
        self.refuse('checkout commit mismatch')

    def test_changed_missing_extra_files(self):
        path = self.cargo / self.archive
        path.write_bytes(b'changed')
        self.refuse('checksum mismatch')
        path.unlink()
        self.refuse('No such file')
        path.write_bytes(b'crate')
        (path.parent / 'extra').write_bytes(b'extra')
        self.refuse('missing or extra')

    def test_unsafe_paths_and_symlinks(self):
        original = copy.deepcopy(self.receipt)
        for name in ('../escape', '/absolute', module.INDEX + '//bad', 'other/file'):
            self.receipt['files'][name] = {'bytes': 0, 'sha256': module.digest(b'')}
            self.refuse('unsafe cache path|unexpected cache tree')
            self.receipt = copy.deepcopy(original)
        path = self.cargo / self.archive
        path.unlink()
        path.symlink_to('/etc/passwd')
        self.refuse('symlink')
        path.unlink()
        path.write_bytes(b'crate')
        folder = self.cargo / module.ARCHIVES
        moved = self.root / 'moved'
        folder.rename(moved)
        folder.symlink_to(moved, target_is_directory=True)
        self.refuse('symlink')

    def test_inventory_omits_locked_data(self):
        path = self.cargo / self.archive
        path.unlink()
        del self.receipt['files'][self.archive]
        self.refuse('locked crate archive')

    def test_index_and_policy_fail_closed(self):
        index = module.INDEX + '/.cache/de/mo/demo'
        (self.cargo / index).write_bytes(b'no-record')
        self.receipt['files'][index] = {'bytes': 9, 'sha256': module.digest(b'no-record')}
        self.refuse('index metadata invalid')
        (self.cargo / 'config.toml').write_text('[source.crates-io]\n')
        self.refuse('configuration overrides')

    def test_missing_advisory_content(self):
        name = module.ADVISORIES + '/crates/demo/RUSTSEC-2000-0001.md'
        (self.cargo / name).unlink()
        del self.receipt['files'][name]
        self.refuse('advisory database content missing')

    def test_missing_index_metadata(self):
        index = module.INDEX + '/.cache/de/mo/demo'
        (self.cargo / index).unlink()
        del self.receipt['files'][index]
        self.refuse('index entry missing')

    def test_real_entrypoint_routes_and_propagates_failure(self):
        # Copy the real entrypoint into a disposable minimal repository. Only
        # cargo is synthetic; cache validation and subprocess routing are real.
        quality = self.repo / 'scripts/quality'
        quality.mkdir(parents=True)
        shutil.copyfile(SCRIPT, quality / SCRIPT.name)
        binary = self.root / 'bin'
        binary.mkdir()
        capture = self.root / 'called'
        stub = binary / 'cargo'
        stub.write_text('#!/bin/sh\nprintf "%s\\n" "$@" > "$CAPTURE"\nexit 7\n')
        stub.chmod(0o755)
        env = {'PATH': str(binary) + ':/usr/bin:/bin', 'CAPTURE': str(capture), 'CARGO_HOME': str(self.cargo), 'PYTHONDONTWRITEBYTECODE': '1'}
        def run():
            return subprocess.run([os.sys.executable, '-B', str(quality / SCRIPT.name)], env=env, capture_output=True, text=True, timeout=10)
        result = run()
        self.assertEqual(result.returncode, 7, result.stderr)
        self.assertEqual(capture.read_text().splitlines(), ['deny', '--locked', 'check'])
        capture.unlink()
        env[module.MANIFEST_ENV] = str(self.manifest)
        self.assertEqual(run().returncode, 2)
        self.assertFalse(capture.exists())
        # The production clock is used only for this end-to-end fresh receipt;
        # all exact age boundary tests above inject a deterministic clock.
        for kind in ('advisory', 'registry'):
            self.receipt[kind]['fetched_at'] = int(time.time())
        env[module.PIN_ENV] = self.seal()
        result = run()
        self.assertEqual(result.returncode, 7, result.stderr)
        self.assertEqual(capture.read_text().splitlines(), ['deny', '--locked', '--offline', 'check'])
        capture.unlink()
        (self.cargo / self.archive).write_bytes(b'tampered')
        self.assertEqual(run().returncode, 2)
        self.assertFalse(capture.exists())


if __name__ == '__main__':
    unittest.main(verbosity=2)
