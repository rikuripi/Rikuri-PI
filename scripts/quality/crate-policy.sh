#!/usr/bin/env bash
# Crate policy check: every crate in crates/ and apps/ opts into the
# workspace lints (`[lints] workspace = true`) and overrides none of them.
# Cargo applies workspace lints only to crates that opt in, so without this
# check a new crate could use `unsafe`, `unwrap()` or `println!` and pass the
# gate (PR #7 review). Usage: crate-policy.sh [root] (default: repository root)
set -euo pipefail
root="${1:-$(git rev-parse --show-toplevel)}"

python3 - "$root" <<'PY'
import glob, os, sys, tomllib
root = sys.argv[1]
manifests = sorted(glob.glob(os.path.join(root, "crates", "*", "Cargo.toml"))
                   + glob.glob(os.path.join(root, "apps", "*", "Cargo.toml")))
failed = False
for path in manifests:
    name = os.path.relpath(os.path.dirname(path), root)
    try:
        with open(path, "rb") as f:
            manifest = tomllib.load(f)
    except (OSError, tomllib.TOMLDecodeError) as e:
        print(f"POLICY {name}: cannot read Cargo.toml: {e}")
        failed = True
        continue
    lints = manifest.get("lints")
    if lints != {"workspace": True}:
        print(f"POLICY {name}: [lints] must be exactly `workspace = true` (found {lints!r})")
        failed = True
print(f"POLICY checked {len(manifests)} crate(s)")
if not manifests:
    print("POLICY no crates found")
    failed = True
sys.exit(1 if failed else 0)
PY
