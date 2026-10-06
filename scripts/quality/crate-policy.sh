#!/usr/bin/env bash
# Crate policy check:
# 1. Every workspace member, wherever it lives, opts into the workspace
#    lints (`[lints] workspace = true`) and overrides none of them. Cargo
#    applies workspace lints only to crates that opt in, so without this a
#    crate could use `unsafe`, `unwrap()` or `println!` and pass the gate
#    (PR #7 reviews). Members come from Cargo itself (workspace-members.sh),
#    not from assumed directories.
# 2. Every local crate Cargo builds is a member. An excluded crate used as a
#    path dependency, or a path patch, compiles into the product outside
#    the lint policy, the contract check and `clippy --workspace`.
# 3. Nothing switches lints off in the Cargo config files Cargo reads when
#    run from the workspace root (the root, its parent directories,
#    CARGO_HOME) or in the environment: rustflags that cap, allow or
#    force-warn lints or read an @argfile, an alias that shadows a command
#    the gate runs, a rustc/rustdoc replacement or wrapper, [env] variables
#    that configure clippy or rustc (CLIPPY_ARGS with `force` alone switched
#    clippy off), or a target runner (it made failing tests pass). Each was
#    shown to let the gate pass code it refuses.
# 4. No tracked member source sets clippy's or rustc's environment through
#    `rustc-env` (a build script set CLIPPY_ARGS, and clippy passed).
# Threat model (owner, 2026-10-05): a tripwire for accidental or careless
# weakening, matching the common forms. Deliberate subversion by someone
# with commit access is for review and branch protection; the gate also
# trusts the toolchain on PATH.
# Usage: crate-policy.sh [root] (default: repository root)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "${1:-$(git rev-parse --show-toplevel)}" && pwd)"
# A failed listing fails the policy below, after the configuration checks,
# so a configuration that also breaks Cargo is still named.
listed=1
members="$("$here/workspace-members.sh" "$root")" || listed=0
outside="$("$here/workspace-members.sh" --outside "$root")" || listed=0

python3 - "$root" "$listed" "$members" "$outside" <<'PY'
import os, re, subprocess, sys, tomllib

root, listed = sys.argv[1], sys.argv[2] == "1"
members = [m for m in sys.argv[3].splitlines() if m]
outside = [m for m in sys.argv[4].splitlines() if m]
failed = False

def fail(message):
    global failed
    print(f"POLICY {message}")
    failed = True

# 1. Members opt into the workspace lints.
for name in members:
    path = os.path.join(root, name, "Cargo.toml")
    try:
        with open(path, "rb") as f:
            manifest = tomllib.load(f)
    except (OSError, tomllib.TOMLDecodeError) as e:
        fail(f"{name}: cannot read Cargo.toml: {e}")
        continue
    lints = manifest.get("lints")
    if lints != {"workspace": True}:
        fail(f"{name}: [lints] must be exactly `workspace = true` (found {lints!r})")
print(f"POLICY checked {len(members)} crate(s)")
if not listed:
    fail("cannot list the crates Cargo builds (see the MEMBERS line above)")
elif not members:
    fail("no workspace members found")

# 2. No local crate is built outside the workspace: path dependencies of
# members (from Cargo), and path patches (read here from the manifest and
# the Cargo config files below).
for name in outside:
    fail(f"{name}: a path dependency that is not a workspace member "
         "(excluded or outside the workspace); make it a member")
member_dirs = {os.path.normpath(os.path.join(root, m)) for m in members}

def path_patches(table, base):
    """(name, directory) for every path entry under [patch.*] and [replace]."""
    found = []
    specs = [s for s in (table.get("patch") or {}).values() if isinstance(s, dict)]
    specs.append(table.get("replace") or {})
    for source in specs:
        for name, spec in source.items():
            if isinstance(spec, dict) and isinstance(spec.get("path"), str):
                found.append((name, os.path.normpath(os.path.join(base, spec["path"]))))
    return found

def check_patches(table, base, shown):
    for name, directory in path_patches(table, base):
        if directory not in member_dirs:
            fail(f"{shown}: patch {name} builds {os.path.relpath(directory, root)}, "
                 "which is not a workspace member; make it a member")

try:
    with open(os.path.join(root, "Cargo.toml"), "rb") as f:
        check_patches(tomllib.load(f), root, "Cargo.toml")
except (OSError, tomllib.TOMLDecodeError) as e:
    fail(f"Cargo.toml: cannot read the workspace manifest: {e}")

# 3. Nothing in the build configuration switches lints off.
GATE_COMMANDS = ("build", "clippy", "deny", "fmt", "metadata", "test")
BUILD_TOOLS = ("rustc", "rustc-wrapper", "rustc-workspace-wrapper", "rustdoc")
TOOL_VARS = ("RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTDOC",
             "CARGO_BUILD_RUSTC", "CARGO_BUILD_RUSTC_WRAPPER",
             "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTDOC", "CLIPPY_ARGS")
FLAG_VARS = ("RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_RUSTDOCFLAGS")
ENCODED_FLAG_VARS = ("CARGO_ENCODED_RUSTFLAGS", "CARGO_ENCODED_RUSTDOCFLAGS")

def weakening(flags):
    """Compiler arguments that cap, allow or force-warn lints (`-D warnings`
    does not promote a force-warned lint), or read more arguments from an
    @file, which can hold any of them."""
    bad = []
    for flag in flags:
        if (flag == "--cap-lints" or flag.startswith("--cap-lints=")
                or flag == "-A" or (flag.startswith("-A") and len(flag) > 2)
                or flag == "--allow" or flag.startswith("--allow=")
                or flag == "--force-warn" or flag.startswith("--force-warn=")
                or flag.startswith("@")):
            bad.append(flag)
    return bad

def rustflags_in(config):
    """Every rustflags value in a parsed Cargo config, as a list of flags."""
    found = []
    def add(value):
        if isinstance(value, str):
            found.extend(value.split())
        elif isinstance(value, list):
            found.extend(str(v) for v in value)
    add(config.get("build", {}).get("rustflags"))
    add(config.get("build", {}).get("rustdocflags"))
    for target in config.get("target", {}).values():
        if isinstance(target, dict):
            add(target.get("rustflags"))
    return found

def toolchain_env(name):
    """[env] variables that configure clippy or rustc rather than the program."""
    return name.startswith(("CLIPPY_", "RUSTC", "RUSTDOC")) or name in (
        "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_ENCODED_RUSTDOCFLAGS")

configs = []
d = root
while True:
    for leaf in ("config", "config.toml"):
        configs.append(os.path.join(d, ".cargo", leaf))
    parent = os.path.dirname(d)
    if parent == d:
        break
    d = parent
cargo_home = os.environ.get("CARGO_HOME") or os.path.join(os.path.expanduser("~"), ".cargo")
for leaf in ("config", "config.toml"):
    configs.append(os.path.join(cargo_home, leaf))

for path in dict.fromkeys(configs):
    if not os.path.isfile(path):
        continue
    shown = os.path.relpath(path, root) if path.startswith(root + os.sep) else path
    try:
        with open(path, "rb") as f:
            config = tomllib.load(f)
    except (OSError, tomllib.TOMLDecodeError) as e:
        fail(f"{shown}: cannot read Cargo config: {e}")
        continue
    bad = weakening(rustflags_in(config))
    if bad:
        fail(f"{shown}: compiler flags weaken lints ({' '.join(bad)})")
    for name in config.get("alias", {}):
        if name in GATE_COMMANDS:
            fail(f"{shown}: [alias] {name} shadows `cargo {name}`, which the gate runs")
    for key in BUILD_TOOLS:
        if config.get("build", {}).get(key):
            fail(f"{shown}: build.{key} replaces or wraps the compiler the gate checks with")
    for name in config.get("env", {}):
        if toolchain_env(name):
            fail(f"{shown}: [env] {name} configures clippy or rustc")
    for name, target in config.get("target", {}).items():
        if isinstance(target, dict) and target.get("runner"):
            fail(f"{shown}: [target.{name}] runner replaces how the gate runs tests")
    # Paths in a config file are relative to the directory holding .cargo.
    check_patches(config, os.path.dirname(os.path.dirname(path)), shown)
    for override in config.get("paths", []):
        fail(f"{shown}: paths override {override} builds a crate outside the workspace")

for var, value in sorted(os.environ.items()):
    if var in TOOL_VARS:
        if value:
            fail(f"{var} replaces or wraps the compiler the gate checks with; unset it for the gate")
        continue
    if var.startswith("CARGO_ALIAS_") and var[len("CARGO_ALIAS_"):].lower() in GATE_COMMANDS:
        fail(f"{var} shadows a command the gate runs; unset it for the gate")
        continue
    if var.startswith("CARGO_TARGET_") and var.endswith("_RUNNER"):
        if value:
            fail(f"{var} replaces how the gate runs tests; unset it for the gate")
        continue
    if var in FLAG_VARS or (var.startswith("CARGO_TARGET_") and var.endswith("_RUSTFLAGS")):
        bad = weakening(value.split())
    elif var in ENCODED_FLAG_VARS:
        bad = weakening(value.split("\x1f"))
    else:
        continue
    if bad:
        fail(f"{var} weakens lints ({' '.join(bad)}); unset it for the gate")

# 4. No member source sets clippy's or rustc's environment: a build script
# printing `cargo::rustc-env=CLIPPY_ARGS=…` switched clippy off for its
# crate. Only names clippy or rustc read from their own environment count
# (CLIPPY_*, RUSTC_BOOTSTRAP); RUSTFLAGS-style names are read by Cargo, and
# RUSTC_VERSION-style names by nobody. A text search over tracked sources
# (what CI sees; every source when the root is not a git work tree),
# comments included: a name assembled at run time is not seen, and a
# comment quoting the exact assignment is refused.
RUSTC_ENV = re.compile(r"rustc-env\s*=\s*(?:CLIPPY_\w*|RUSTC_BOOTSTRAP)\s*=|__CLIPPY_HACKERY__")

def member_sources():
    """Relative paths of member .rs sources: tracked files in a git work
    tree, else every file under the member directories."""
    if not members:
        return []
    in_git = subprocess.run(["git", "-C", root, "rev-parse", "--is-inside-work-tree"],
                            capture_output=True).returncode == 0
    if in_git:
        listed = subprocess.run(["git", "-C", root, "ls-files", "-z", "--", *members],
                                capture_output=True)
        if listed.returncode != 0:
            fail("cannot list tracked member sources (git ls-files failed)")
            return []
        return sorted({p for p in listed.stdout.decode().split("\0") if p.endswith(".rs")})
    found = set()
    for name in members:
        for dirpath, dirnames, filenames in os.walk(os.path.join(root, name)):
            dirnames[:] = [d for d in dirnames if d not in ("target", ".git")]
            found.update(os.path.relpath(os.path.join(dirpath, leaf), root)
                         for leaf in filenames if leaf.endswith(".rs"))
    return sorted(found)

for rel in member_sources():
    with open(os.path.join(root, rel), encoding="utf-8", errors="replace") as f:
        if RUSTC_ENV.search(f.read()):
            fail(f"{os.path.normpath(rel)}: sets clippy's or rustc's environment through rustc-env")

sys.exit(1 if failed else 0)
PY
