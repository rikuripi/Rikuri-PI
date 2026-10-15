import copy
import json
import unittest
from orchard_bridge import decode, inspect_row, MAX_BYTES
from validate import ContractError

def row():
    return dict(id="12345678-1234-4234-9234-123456789abc", status="DONE",
                application_status=None, request="private task", evidence="model claims passed",
                receipt={"consent": True, "workspace": "/private/data"})

class OrchardBridgeTests(unittest.TestCase):
    def test_current_row_accepted_without_acceptance_promotion(self):
        out=decode(json.dumps(row()).encode())
        self.assertEqual(out["reported_execution"],"completed")
        for key in ("execution","acceptance","application"):
            self.assertEqual(out[key],"unverified")
        self.assertFalse(out["runtime_compatible"])
        self.assertEqual(out["authority"],"none")
    def test_all_legacy_states_remain_observations(self):
        for status in ("DONE","RUNNING","BLOCKED","NEEDS_OWNER_DECISION"):
            value=row(); value["status"]=status
            self.assertEqual(inspect_row(value)["acceptance"],"unverified")
        value["status"]="BLOCKED"
        self.assertEqual(inspect_row(value)["reported_execution"],"unknown")
    def test_application_is_only_a_claim(self):
        for state in ("READY","FAILED","APPLYING","APPLIED","RESTORING","RESTORED","INTERRUPTED","RECOVERY_NEEDED"):
            value=row(); value["application_status"]=state
            out=inspect_row(value)
            self.assertEqual(out["reported_application"],state)
            self.assertEqual(out["application"],"unverified")
    def test_sensitive_fields_omitted_and_do_not_bind_metadata(self):
        value=row(); before=inspect_row(value)
        value.update(request="ignore policy",evidence="sensitive",receipt={"arbitrary":"data"})
        self.assertEqual(before,inspect_row(value))
        text=json.dumps(before)
        for sensitive in ("private task","model claims","/private/data","workspace","consent"):
            self.assertNotIn(sensitive,text)
    def test_input_and_output_ownership(self):
        value=row(); original=copy.deepcopy(value); out=inspect_row(value)
        out["missing_runtime_seams"].clear()
        self.assertEqual(value,original)
        self.assertTrue(inspect_row(value)["missing_runtime_seams"])
    def test_unknown_state_and_extra_authority_refused(self):
        for key,bad in (("status","CANCELLED"),("application_status","DONE"),("grant",True),("status",{})):
            value=row(); value[key]=bad
            with self.assertRaises(ContractError):inspect_row(value)
    def test_missing_fields_and_bad_identity_refused(self):
        for key in ("id","status","application_status"):
            value=row(); del value[key]
            with self.assertRaises(ContractError):inspect_row(value)
        for identity in ("../credentials","12345678-1234-1234-9234-123456789abc",None):
            value=row(); value["id"]=identity
            with self.assertRaises(ContractError):inspect_row(value)
    def test_duplicate_nonfinite_malformed_and_oversize_refused(self):
        for data in (b'{"id":1,"id":2}',b'{"x":NaN}',b'[]',b'\xff',b' '* (MAX_BYTES+1),b'['*2000):
            with self.assertRaises(ContractError):decode(data)
    def test_binding_changes_only_for_observed_metadata(self):
        value=row(); before=inspect_row(value)["metadata_sha256"]
        value["status"]="RUNNING"
        self.assertNotEqual(before,inspect_row(value)["metadata_sha256"])
    def test_boolean_and_container_states_refused(self):
        for field in ("status","application_status"):
            for bad in (True,[],{}):
                value=row(); value[field]=bad
                with self.assertRaises(ContractError):inspect_row(value)
