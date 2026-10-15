import copy
import unittest
import standard
import validate as v
import orchard_bridge as b

class ObservationSchemaTests(unittest.TestCase):
    def setUp(self):
        self.schema=v.load(v.ROOT/"orchard-observation.schema.json")
        self.engine=standard.validator(self.schema)
        self.base=b.inspect_row({"id":"12345678-1234-4234-9234-123456789abc","status":"DONE","application_status":None})
    def check(self,value,expected):
        self.assertEqual(v.schema_matches(value,self.schema),expected)
        self.assertEqual(self.engine.is_valid(value),expected)
    def test_positive_states_and_null_application(self):
        for state in sorted(b.STATES):
            for app in [None,*sorted(b.APPLICATION_STATES)]:
                value=b.inspect_row({"id":self.base["legacy_task_id"],"status":state,"application_status":app})
                self.check(value,True)
    def test_closed_required_fields_and_invalid_claims(self):
        for field in self.base:
            value=copy.deepcopy(self.base);del value[field];self.check(value,False)
        for change in ({"acceptance":"passed"},{"application":"applied"},{"execution":"completed"},
                       {"authority":"granted"},{"grant":True},{"schema_version":True},
                       {"schema_version":2},{"reported_application":"DONE"},{"runtime_compatible":True},
                       {"missing_runtime_seams":[]},{"reported_status":"CANCELLED"},
                       {"metadata_sha256":"a"*64+"\n"},{"legacy_task_id":self.base["legacy_task_id"]+"\n"}):
            value=copy.deepcopy(self.base);value.update(change);self.check(value,False)
    def test_status_consistency_and_digest_authority_boundary(self):
        for status in sorted(b.STATES):
            for execution in ("completed","running","unknown"):
                value=copy.deepcopy(self.base);value.update(reported_status=status,reported_execution=execution)
                expected=execution=={"DONE":"completed","RUNNING":"running"}.get(status,"unknown")
                self.check(value,expected)
        # A correctly shaped digest is not a semantic binding or producer proof.
        value=copy.deepcopy(self.base);value["metadata_sha256"]="a"*64
        self.check(value,True)
        with self.assertRaises(v.ContractError):b.validate_observation(value)
