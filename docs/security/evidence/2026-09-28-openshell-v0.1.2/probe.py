import errno
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import urllib.request

results = []

def probe(name, fn):
    try:
        value = fn()
        results.append({"name": name, "outcome": "allowed", "value": value})
    except Exception as exc:
        results.append({"name": name, "outcome": "blocked_or_failed", "error": type(exc).__name__, "errno": getattr(exc, "errno", None), "detail": str(exc)})

probe("allowed_read", lambda: Path('/readonly/visible.txt').read_text())
probe("allowed_write", lambda: Path('/workspace/allowed.txt').write_text('synthetic allowed write\n'))
probe("readonly_write", lambda: Path('/readonly/visible.txt').write_text('synthetic attempted write\n'))
probe("forbidden_read", lambda: Path('/forbidden/not-for-agent.txt').read_text())
probe("forbidden_write", lambda: Path('/forbidden/attempt.txt').write_text('synthetic attempted write\n'))

def symlink_read():
    p = Path('/workspace/escape')
    p.unlink(missing_ok=True)
    p.symlink_to('/forbidden/not-for-agent.txt')
    return p.read_text()
probe("symlink_escape", symlink_read)

def child_read():
    p = subprocess.run([sys.executable, '-c', 'from pathlib import Path; print(Path("/forbidden/not-for-agent.txt").read_text())'], capture_output=True, text=True, timeout=5)
    if p.returncode:
        raise PermissionError(p.stderr.strip())
    return p.stdout.strip()
probe("child_forbidden_read", child_read)

host = os.environ['PROBE_HOST']
port = int(os.environ['PROBE_PORT'])
marker = os.environ.get('PROBE_MARKER', 'unspecified')

def raw_tcp():
    with socket.create_connection((host, port), timeout=3) as s:
        s.sendall(('GET /' + marker + '/tcp HTTP/1.0\r\nHost: synthetic\r\n\r\n').encode())
        data = s.recv(1024)
        if not data.startswith(b'HTTP/1.0 200'):
            raise RuntimeError('Expected synthetic HTTP response, got ' + repr(data[:80]))
        return 'synthetic server reached'
probe("raw_tcp", raw_tcp)

def http_request():
    with urllib.request.urlopen('http://' + host + ':' + str(port) + '/' + marker + '/http', timeout=3) as r:
        return r.read().decode()
probe("http_request", http_request)

def raw_udp():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
        s.settimeout(2)
        msg = ('synthetic-' + marker).encode()
        s.sendto(msg, (host, port))
        reply, _ = s.recvfrom(1024)
        if reply != msg:
            raise RuntimeError('Unexpected synthetic UDP response')
        return 'synthetic UDP server reached'
probe("raw_udp", raw_udp)

report = {"uid": os.getuid(), "gid": os.getgid(), "marker": marker, "results": results}
print(json.dumps(report, indent=2), flush=True)
Path('/workspace/results.json').write_text(json.dumps(report, indent=2))
