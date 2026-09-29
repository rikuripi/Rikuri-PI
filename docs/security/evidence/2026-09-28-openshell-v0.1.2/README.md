# OpenShell containment evaluation — 2026-09-28

Status: **PASS for ten bounded smoke outcomes; production qualification Not yet proven.** This is an archived candidate experiment, not a Rikuri Stage Gate Record. Date uses America/Los_Angeles; raw UTC timestamps can read September 29.

The test ran earlier in the same September 28 session. Archiving this receipt did not rerun containment or install a runtime in Rikuri.

## Environment and scope

This tested OpenShell v0.1.2 on Linux x86_64, kernel 6.8.0-138-generic, Docker Engine 29.8.1, with Landlock and AppArmor enabled. Release archives were verified against their GitHub release SHA-256 digests before execution.

The synthetic Python image was based on `python@sha256:392307d22300de8b5986851a12d9176dfc0fc073e65bf6523ebd7dcbeb23564e`. The worker ran as UID/GID 1000. Both worker and supervisor containers had `Privileged=false` and the `docker-default` AppArmor profile.

The gateway listened on 127.0.0.1:28670 with mTLS authentication, separate temporary XDG directories, and `OPENSHELL_TELEMETRY_ENABLED=false`. Policy required Landlock (`hard_requirement`), permitted only listed paths, contained no network rules, and attached no credential providers. Inspection confirmed workload network mode `none`; its separate supervisor used host networking.

No model calls, user secrets, real personal files, or repository mounts were used. No existing repository, firewall, or service configuration was changed.

## Results

The identical probe first ran in an ordinary Docker container as the same unprivileged UID. All ten operations succeeded there. This established that Unix file permissions, missing fixtures, and an unavailable network server were not responsible for the later denials.

| Probe | Ordinary Docker baseline | OpenShell result | Assessment |
|---|---|---|---|
| Read the allowed fixture | Succeeded | Succeeded, expected content | PASS |
| Write in the allowed workspace | Succeeded | Succeeded; separately read back | PASS |
| Write the read-only fixture | Succeeded | Permission denied; original preserved | PASS |
| Read the forbidden fixture | Succeeded | Permission denied | PASS |
| Write in the forbidden directory | Succeeded | Permission denied | PASS |
| Read forbidden file through a workspace symlink | Succeeded | Permission denied | PASS |
| Child Python process reads forbidden file | Succeeded | Child received permission denied | PASS |
| Direct TCP connection to synthetic server | Succeeded | Permission denied; policy denial logged | PASS |
| HTTP request to synthetic server | Succeeded | Permission denied; policy denial logged | PASS |
| UDP echo exchange with synthetic server | Succeeded | Failed; no datagram reached server | PASS for observed denial |

The UDP syscall returned EDESTADDRREQ (errno 89). This demonstrates that this probe could not communicate, rather than a readable UDP policy-denial message. The synthetic server log contains only the successful baseline requests and no openshell-offline requests. TCP and HTTP denials also appear in OpenShell logs as transparent_tcp_policy_denied.

The first setup placed the dummy server on the host's Docker bridge address. The ordinary container timed out against it, so those failures were not counted as containment evidence. The server was moved into a separate ordinary container, where every baseline operation succeeded. No host firewall settings were changed. The initial unsuccessful baseline is retained for transparency.

## Evidence

- [Host details](evidence/host.json)
- [Selected release metadata and archive digests](evidence/release.json)
- [Synthetic image definition](Dockerfile)
- [Probe source](probe.py)
- [Final dummy server source](mock_container.py)
- [Strict offline policy](offline.yaml)
- [Baseline results](evidence/baseline.json)
- [OpenShell results](evidence/offline.json)
- [Independent file read-back](evidence/readback.json)
- [OpenShell policy admission and status](evidence/sandbox)
- [Actual container boundaries and image IDs](evidence/container-boundaries.json)
- [Enforcement logs](evidence/sandbox-log)
- [Dummy server requests](evidence/mock-log)
- [Cleanup confirmation](evidence/cleanup.json)
- [Initial invalid network baseline](evidence/initial-baseline-host-endpoint.json)
- [Isolated command environment](harness.py) and [gateway configuration](gateway.toml)
- [Artifact integrity manifest](SHA256SUMS)

## Interpretation and remaining work

This demonstrates that this OpenShell release, driver, host kernel, image, and policy can perform useful file work while enforcing the tested restrictions. It supports proceeding with a bounded Rikuri backend evaluation.

It does not qualify OpenShell for production or close Rikuri F4/F9 or a Stage 03 gate. This run did not test permission revocation, supervisor crashes, live policy changes, credentials, DNS leakage, IPv6, resource exhaustion, malicious project hooks, broad syscall escape attempts, native macOS, or Sentry hardware. It also did not test an OpenShell policy permitting selected external destinations. The network positive control was the ordinary Docker baseline.

## Cleanup

The test sandbox, supervisor, dummy server, test volumes, temporary gateway process, and tagged synthetic image were removed or stopped. No system service was installed. Downloaded upstream images remain in Docker's cache. Downloaded binaries and temporary state remained in the original scratch directory. This repository contains only the selected synthetic source/configuration and evidence files.

No state/config credential directories or downloaded binaries are included. Release metadata was reduced to the two evaluated Linux archives and their source commit; disposable authentication annotations were removed from the sandbox inspection JSON. Other selected artifacts are byte-for-byte copies; no failure was converted to a success. The image references in the historical gateway configuration used version tags; actual container image IDs are recorded, but this does not satisfy future production immutable-digest pinning.

## Reviewing and repeating the experiment

Run `sha256sum -c SHA256SUMS` from this directory to check the archived artifacts. Inspect the baseline, offline result, file read-back and mock/enforcement logs together. `probe.py` records outcomes and returns success after collecting them even when operations fail; its process exit code is **not** a containment verdict. In particular, inspect the child's actual error and UDP/server evidence, rather than treating any exception as enforcement.

To repeat the runtime experiment, use a disposable test environment with the recorded release and a separately admitted Docker setup. These are lab fixtures, not a production policy or a turnkey installer:

1. Verify the two release archives against the recorded digests. Create fresh temporary XDG directories and test authentication; use `harness.py` to avoid inheriting user credentials and to disable optional telemetry. Start a temporary loopback gateway with `gateway.toml` using the pinned CLI's documented commands.
2. Build `Dockerfile` using a build context containing only that file and `probe.py`. Start `mock_container.py` in a separate ordinary test container; discover its bridge address and use port 8088. Do not weaken the host firewall if the baseline cannot reach it.
3. Run the probe in a fresh ordinary container, UID/GID 1000, with `PROBE_HOST`, `PROBE_PORT=8088` and `PROBE_MARKER=baseline`. All ten operations must succeed. No user directories, Docker socket or credentials are mounted in the worker.
4. Use the same image in an OpenShell sandbox with `offline.yaml`, no providers, the same synthetic endpoint and `PROBE_MARKER=openshell-offline`. Run `/test/probe.py` only inside that worker. Capture JSON and independently read `/readonly/visible.txt` and `/workspace/allowed.txt`; collect server/enforcement logs and inspect actual container boundaries. Interpret each result against the table above.
5. Remove only the named test sandbox, mock container, volumes and synthetic image; stop the temporary gateway. Record failures and cleanup. This archive does not provide an automated lifecycle runner; S00.13 must supply a repeatable reference-installation probe before admission.

The broader required suite is [Test Strategy §8.2](../../../../doc/Rikuri_Pi_Foundation0_Test_Replay_and_Acceptance_Strategy_v1.2.md#82-option-c-qualification-and-personal-data-acceptance). No claim of credential isolation, full offline control-plane behavior, escape-proof security or native macOS coverage follows from this experiment.
