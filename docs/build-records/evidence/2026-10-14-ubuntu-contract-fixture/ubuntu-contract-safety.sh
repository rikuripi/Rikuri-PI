#!/bin/bash
# PROPOSED, NOT YET EXECUTED. Contract fixture tests only; not a general sandbox.
set -euo pipefail
export PATH=/usr/sbin:/usr/bin:/sbin:/bin
unset BASH_ENV ENV
self=$(/usr/bin/realpath -- "$0")
[[ $EUID == 0 ]] || { echo "Refused: reviewed namespace setup must start as guest root." >&2; exit 2; }
# Only a guest-local, root-owned, non-writable staging tree may be used.
[[ "$self" =~ ^/var/tmp/rikuri-contract\.[A-Za-z0-9]+/launcher.sh$ ]] || { echo "Refused: launcher is not in protected guest staging." >&2; exit 2; }
stage=$(/usr/bin/dirname -- "$self")
[[ $(/usr/bin/stat -c %u -- "$stage") == 0 ]] || exit 2
[[ -z $(/usr/bin/find "$stage" \( ! -user root -o -perm /022 -o -type l \) -print -quit) ]] || { echo "Refused: staging owner/mode/symlink check." >&2; exit 2; }

if [[ ${1:-} != --inside ]]; then
    [[ $# == 0 ]] || { echo "Refused: no arbitrary commands/arguments." >&2; exit 2; }
    uid=$(/usr/bin/id -u henry); gid=$(/usr/bin/id -g henry)
    [[ "$uid" != 0 ]] || exit 2
    /usr/bin/test -d "$stage/contract/tests"
    # There is no user namespace here: the root-only mount setup is the explicit
    # proposed exception. Global userns/AppArmor settings are never changed.
    exec /usr/bin/unshare --mount --net --pid --fork --mount-proc \
        /usr/bin/env -i PATH="$PATH" /bin/bash "$self" --inside "$uid" "$gid" \
        "$(/usr/bin/readlink /proc/self/ns/mnt)" "$(/usr/bin/readlink /proc/self/ns/net)" "$(/usr/bin/readlink /proc/self/ns/pid)"
fi

[[ $# == 6 && $$ == 1 ]] || { echo "Refused: inner setup is not private PID-namespace init." >&2; exit 2; }
uid=$2; gid=$3
[[ "$uid" =~ ^[0-9]+$ && "$gid" =~ ^[0-9]+$ && "$uid" != 0 ]] || exit 2
[[ "$(/usr/bin/readlink /proc/self/ns/mnt)" != "$4" ]] || exit 2
[[ "$(/usr/bin/readlink /proc/self/ns/net)" != "$5" ]] || exit 2
[[ "$(/usr/bin/readlink /proc/self/ns/pid)" != "$6" ]] || exit 2

/usr/bin/mount -n --make-rprivate /
/usr/bin/mount -n -t tmpfs -o mode=1777,size=64m tmpfs /tmp
/usr/bin/mkdir /tmp/contract /tmp/home
/usr/bin/mount -n --bind "$stage/contract" /tmp/contract
/usr/bin/mount -n -o remount,bind,ro,nosuid,nodev /tmp/contract
/usr/bin/chown "$uid:$gid" /tmp/home
# Hide service sockets, host shares, guest homes, and other temporary data.
/usr/bin/mount -n -t tmpfs -o mode=755,size=1m tmpfs /run
/usr/bin/mount -n -t tmpfs -o mode=755,size=1m tmpfs /media
/usr/bin/mount -n -t tmpfs -o mode=755,size=1m tmpfs /mnt
/usr/bin/mount -n -t tmpfs -o mode=755,size=1m tmpfs /home
/usr/bin/mount -n -t tmpfs -o mode=755,size=1m tmpfs /root
/usr/bin/mount -n -t tmpfs -o mode=1777,size=16m tmpfs /var/tmp
cd /tmp/contract
# No supplementary groups, capabilities, inherited environment or privilege gain.
exec /usr/bin/timeout --signal=TERM --kill-after=5 60 \
    /usr/bin/setpriv --reuid="$uid" --regid="$gid" --clear-groups \
    --bounding-set=-all --inh-caps=-all --ambient-caps=-all --no-new-privs \
    /usr/bin/env -i HOME=/tmp/home TMPDIR=/tmp PATH=/usr/bin:/bin \
    PYTHONNOUSERSITE=1 PYTHONDONTWRITEBYTECODE=1 \
    /usr/bin/python3 -B safety_probe_and_tests.py
