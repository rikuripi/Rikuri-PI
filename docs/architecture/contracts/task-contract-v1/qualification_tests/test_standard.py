import copy
import unittest
from unittest.mock import patch
from jsonschema import Draft202012Validator
from referencing.exceptions import Unresolvable
import validate as v
import standard as s

class StandardQualification(unittest.TestCase):
    def setUp(self):
        self.task=v.example()
        self.request=v.load(v.ROOT/'task-contract.schema.json')
        self.result=v.load(v.ROOT/'task-result.schema.json')

    def test_schemas_are_valid_draft202012(self):
        for schema in [self.request,self.result]:Draft202012Validator.check_schema(schema)

    def test_positive_request_and_all_receipt_shapes(self):
        s.validate_contract(self.task)
        for options in [{},{'fixture':'incorrect'},{'missing_evidence':True}]:
            receipt=v.FakeController().run(self.task,v.fixture_grant(self.task),**options)
            s.validate_result(self.task,receipt)
        s.validate_result(self.task,v.FakeController().run(self.task,None))

    def test_each_required_field_rejected_by_both_checkers(self):
        for key in self.task:
            task=copy.deepcopy(self.task);task.pop(key)
            self.assertFalse(s.validator(self.request).is_valid(task),key)
            self.assertFalse(v.schema_matches(task,self.request),key)

    def test_nested_shapes_enums_limits_and_extra_authority(self):
        mutations=[
            lambda t:t.update(approved=True),
            lambda t:t['limits'].pop('wall_seconds'),
            lambda t:t['limits'].update(wall_seconds=0),
            lambda t:t['limits'].update(max_workers=2),
            lambda t:t.update(schema_version=True),
            lambda t:t.update(schema_version=2),
            lambda t:t['stop'].update(automatic_next_task=True),
            lambda t:t['requested_capabilities'].update(write_paths=['../escape']),
            lambda t:t.update(task_id='task_00000000000000000000000000'),
            lambda t:t.update(privacy='invented'),
        ]
        for mutate in mutations:
            task=copy.deepcopy(self.task);mutate(task)
            self.assertFalse(s.validator(self.request).is_valid(task))
            self.assertFalse(v.schema_matches(task,self.request))

    def test_false_done_evidence_and_terminal_shape(self):
        base=v.FakeController().run(self.task,v.fixture_grant(self.task))
        for mutate in [
            lambda r:r.update(evidence=[]),
            lambda r:r.update(artifacts=[]),
            lambda r:r.update(acceptance='unverified'),
            lambda r:r.update(execution='not_started'),
            lambda r:r['evidence'][0].update(passed=False),
        ]:
            receipt=copy.deepcopy(base);mutate(receipt)
            self.assertFalse(s.validator(self.result).is_valid(receipt))
            self.assertFalse(v.schema_matches(receipt,self.result))

    def test_cross_field_approval_coverage_is_not_schema_authority(self):
        task=copy.deepcopy(self.task);task['requested_capabilities']['write_paths']=['other.py']
        self.assertTrue(s.validator(self.request).is_valid(task))
        with self.assertRaises(v.ContractError):s.validate_contract(task)
        result=v.FakeController().run(self.task,v.fixture_grant(self.task))
        result['contract_sha256']='b'*64
        self.assertTrue(s.validator(self.result).is_valid(result))
        with self.assertRaises(v.ContractError):s.validate_result(self.task,result)

    def test_integer_value_vs_canonical_wire_encoding_is_explicit(self):
        task=copy.deepcopy(self.task);task['limits']['wall_seconds']=30.0
        self.assertTrue(s.validator(self.request).is_valid(task))
        self.assertFalse(v.schema_matches(task,self.request))
        with self.assertRaises(v.ContractError):s.validate_contract(task)

    def test_remote_reference_retrieval_refused_without_network(self):
        attempts=[]
        original_denial=s.deny_remote
        def deny(uri):
            attempts.append(uri)
            return original_denial(uri)
        with patch.object(s,'deny_remote',side_effect=deny), patch('urllib.request.urlopen') as request:
            validator=s.validator({'$ref':'https://example.invalid/schema'})
            with self.assertRaises(Unresolvable):validator.validate({})
            request.assert_not_called()
        self.assertEqual(attempts,['https://example.invalid/schema'])

    def test_original_25_regressions_with_standard_shape_engine(self):
        suite=unittest.defaultTestLoader.discover(str(v.ROOT/'tests'),pattern='test_contract.py')
        result=unittest.TestResult()
        with patch.object(v,'validate_shape',s.validate_shape):suite.run(result)
        self.assertEqual(result.testsRun,25)
        self.assertEqual(result.errors,[])
        self.assertEqual(result.failures,[])
        self.assertEqual(result.skipped,[])

    def test_digest_rejects_trailing_newline(self):
        task=copy.deepcopy(self.task);task['source']['revision']+='\n'
        self.assertFalse(s.validator(self.request).is_valid(task))
        result=v.FakeController().run(self.task,v.fixture_grant(self.task))
        result['contract_sha256']+='\n'
        self.assertFalse(s.validator(self.result).is_valid(result))

    def test_check_identity_rejects_trailing_newline(self):
        task=copy.deepcopy(self.task)
        task['acceptance'][0]['check_id']+='\n'
        task['requested_capabilities']['execute_checks'][0]+='\n'
        self.assertFalse(s.validator(self.request).is_valid(task))

if __name__=='__main__':unittest.main()
