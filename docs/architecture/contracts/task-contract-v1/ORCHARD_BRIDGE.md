# Read-only Orchard bridge

This standalone snapshot interpreter consumes an explicitly selected JSON row in current Orchard Crew.list() format. It never imports crew.py, queries a database, starts Codex, or writes canonical state. The production application does not call it.

Run: python3 -B orchard_bridge.py < explicitly-selected-synthetic-row.json

Only UUID-v4 task identity, legacy status and application_status are projected. Request/evidence/receipt are omitted, even when they assert approvals or successful checks. The metadata digest binds just those three observed fields; it is not authentication, provenance, or a freshness proof. Input is bounded to 256 KiB, duplicate keys and unknown states/fields are refused, errors withhold content.

DONE reports completed legacy execution, with actual execution, acceptance and application all unverified. BLOCKED can follow restart while execution remains uncertain. APPLIED/RESTORED are legacy claims, not verified application receipts. No fabricated run ID, grant, gate evidence or TaskResult is produced. Runtime compatibility remains false: split preparation/dispatch, acknowledgement bindings, cursor observations, confirmed cancellation, trusted capture and independent acceptance are still missing. This describes inspected source behavior, not a negotiated runtime capability test.

[Adapter design](ADAPTER_INTERFACE.md) · [Simulator](ADAPTER_SIMULATOR.md)

## Version 1 export validation

The [closed observation schema](orchard-observation.schema.json) fixes the existing bridge output shape. Every observation is validated before return/export, including status-to-reported-execution consistency. validate_observation additionally recomputes the selected-metadata digest; schema shape alone cannot check that binding. Existing TaskContract/TaskResult and adapter event schemas are unchanged.

The standard Draft 2020-12 engine qualifies this schema alongside the small custom checker. This remains a legacy snapshot wire contract, not a general runtime observation schema or production schema registry. A valid forged snapshot can still be forged; no authentication, freshness or operational capability is established.
