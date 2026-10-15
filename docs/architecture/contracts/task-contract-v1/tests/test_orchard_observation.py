import copy
import unittest
from unittest.mock import patch
import orchard_bridge as b
from validate import ContractError

def observation():
    return b.inspect_row({"id":"12345678-1234-4234-9234-123456789abc","status":"DONE","application_status":None})

class ObservationTests(unittest.TestCase):
    def test_validation_is_mandatory_before_export(self):
        with patch.object(b,"validate_observation",side_effect=ContractError("refused")):
            with self.assertRaises(ContractError):
                observation()
    def test_metadata_binding_is_separate_from_shape(self):
        original=observation()
        for change in ({"metadata_sha256":"a"*64},{"reported_application":"APPLIED"},
                       {"legacy_task_id":"22345678-1234-4234-9234-123456789abc"}):
            value=copy.deepcopy(original); value.update(change)
            with self.assertRaises(ContractError):b.validate_observation(value)
    def test_authority_and_false_acceptance_refused(self):
        for change in ({"acceptance":"passed"},{"authority":"granted"},{"runtime_compatible":True},
                       {"execution":"completed"},{"application":"applied"},{"grant":True},
                       {"reported_execution":"unknown"},{"schema_version":2}):
            value=observation();value.update(change)
            with self.assertRaises(ContractError):b.validate_observation(value)
    def test_validation_does_not_mutate_observation(self):
        value=observation(); before=copy.deepcopy(value)
        self.assertIsNone(b.validate_observation(value))
        self.assertEqual(before,value)
