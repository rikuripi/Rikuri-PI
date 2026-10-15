import copy
from pathlib import Path
import unittest
import standard
import validate as v

class AdapterSchemaTests(unittest.TestCase):
    def test_three_schemas_use_standard_meta_validation(self):
        for name in ['adapter-lifecycle','adapter-event','adapter-gate-receipt']:
            standard.validator(v.load(v.ROOT/(name+'.schema.json')))
    def test_closed_fields_and_false_done_lifecycle(self):
        task=v.example()
        state={'schema_version':1,'task_id':task['task_id'],'run_id':task['run_id'],
               'handle_sha256':'a'*64,'contract_sha256':'b'*64,'source_sha256':'c'*64,
               'phase':'PREPARED','execution':'not_started','acceptance':'unverified',
               'application':'not_applied','terminal':'NONE','cancel_requested':False}
        schema=v.load(v.ROOT/'adapter-lifecycle.schema.json')
        self.assertTrue(standard.validator(schema).is_valid(state))
        self.assertTrue(v.schema_matches(state,schema))
        for change in [{'terminal':'DONE'},{'phase':'FINALIZED'},{'approve':True},
                       {'acceptance':'passed'},{'phase':'RELEASED','execution':'unknown'}]:
            bad=copy.deepcopy(state);bad.update(change)
            self.assertFalse(standard.validator(schema).is_valid(bad))
            self.assertFalse(v.schema_matches(bad,schema))

if __name__=='__main__':unittest.main()
