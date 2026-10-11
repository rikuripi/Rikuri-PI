#!/bin/sh
# E1 evidence: planted defects N1, N2, N5, N6, N8, N9 (verifier round 4), X2-X4 (round 5), R1, R3, S1 (round 6)
# through the real daemon and CLI, each on a fresh scratch home, with and
# without the check. Run inside a sandbox with no network and no /run.
# Usage: drive-scenarios.sh <bin-dir> <scratch-dir>
B=$1
H=$2
D=$(dirname "$0")/drive-daemon.py
mkdir -p "$H"
i=0
run() {
    i=$((i + 1))
    python3 -I "$D" "$B" "$H/s$i" "$@"
    echo
}
run foreign-run
for s in foreign-run foreign-run-created foreign-complete-running foreign-complete-created; do
    run "$s" RIKURI_E1_N1=1
    run "$s" RIKURI_E1_N1=1 RIKURI_E1_NOCHECK=1
done
run shared-deletion-unit RIKURI_E1_N5=1
run shared-deletion-unit RIKURI_E1_N5=1 RIKURI_E1_NOCHECK=1
run no-idempotency-record RIKURI_E1_N6=1
run no-idempotency-record RIKURI_E1_N6=1 RIKURI_E1_NOCHECK=1
run wrong-idempotency-command RIKURI_E1_N9=1
run wrong-idempotency-command RIKURI_E1_N9=1 RIKURI_E1_NOCHECK=1
run init-trust
run init-trust RIKURI_E1_N8=1
run init-trust RIKURI_E1_N8=1 RIKURI_E1_NOCHECK=1
# verifier round 5: the idempotency record
run no-idempotency-record RIKURI_E1_X4=1
run no-idempotency-record RIKURI_E1_X4=1 RIKURI_E1_NOCHECK=1
run stored-response
run stored-response RIKURI_E1_X2=1
run stored-response RIKURI_E1_X2=1 RIKURI_E1_NOCHECK=1
run extra-record
run extra-record RIKURI_E1_X3=1
run extra-record RIKURI_E1_X3=1 RIKURI_E1_NOCHECK=1
# verifier round 6: the answer, and the event schema version
run create-answer
run create-answer RIKURI_E1_R1=1
run create-answer RIKURI_E1_R1=1 RIKURI_E1_NOCHECK=1
run init-answer
run init-answer RIKURI_E1_R3=1
run init-answer RIKURI_E1_R3=1 RIKURI_E1_NOCHECK=1
run event-schema
run event-schema RIKURI_E1_S1=1
run event-schema RIKURI_E1_S1=1 RIKURI_E1_NOCHECK=1
