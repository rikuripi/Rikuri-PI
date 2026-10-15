"""Proposed guest-only admission probe followed by the original stdlib suite."""
import os
from pathlib import Path
import unittest

assert os.getuid() != 0, "Refused: tests would run as root"
assert os.getgroups() == [], "Refused: supplementary groups retained"
status = dict(line.split(":", 1) for line in Path("/proc/self/status").read_text().splitlines() if ":" in line)
for key in ["CapEff", "CapPrm", "CapBnd", "CapAmb"]:
    assert int(status[key].strip(), 16) == 0, "Refused: capabilities retained"
assert status["NoNewPrivs"].strip() == "1"
for path in ["/media", "/mnt", "/run", "/home", "/root", "/var/tmp"]:
    assert list(Path(path).iterdir()) == [], "Refused: masked directory is visible: " + path
route_lines = [line for line in Path("/proc/net/route").read_text().splitlines() if line.strip()]
assert not route_lines or (len(route_lines) == 1 and route_lines[0].split()[0] == "Iface"), "Refused: route or unrecognized metadata exists"
import errno
import socket
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as connection:
    connection.settimeout(1)
    try:
        connection.connect(("192.0.2.1", 9))
    except OSError as exc:
        assert exc.errno == errno.ENETUNREACH, "Refused: no-network outcome not proven"
    else:
        raise RuntimeError("Refused: external connection succeeded")
print("PASS: external synthetic IPv4 connection refused with ENETUNREACH", flush=True)
try:
    Path("/tmp/contract/.write-probe").write_text("should fail")
except PermissionError:
    pass
except OSError as exc:
    import errno
    assert exc.errno == errno.EROFS, "Refused: unexpected write-probe failure"
else:
    raise RuntimeError("Refused: snapshot writable")
print("PASS: privilege, mask, route and read-only-snapshot admission checks", flush=True)
suite = unittest.defaultTestLoader.discover("tests", pattern="test_contract.py")
result = unittest.TextTestRunner(verbosity=2).run(suite)
assert result.testsRun == 25 and not result.skipped, "Refused: expected original suite coverage missing"
raise SystemExit(0 if result.wasSuccessful() else 1)
