"""Qualification-only standard shape validation; not production admission."""
from pathlib import Path
import json
from jsonschema import Draft202012Validator
from referencing import Registry
from referencing.exceptions import NoSuchResource
import validate as fixture

def deny_remote(uri):
    raise NoSuchResource(ref=uri)

def validator(schema):
    Draft202012Validator.check_schema(schema)
    return Draft202012Validator(schema, registry=Registry(retrieve=deny_remote))

def validate_shape(value, filename):
    schema=fixture.load(fixture.ROOT/filename)
    if not validator(schema).is_valid(value):
        raise fixture.ContractError('standard schema violation (content withheld)')

def validate_contract(value):
    validate_shape(value,'task-contract.schema.json')
    # Existing strict wire/cross-field refusals are additional to shape.
    fixture.validate_contract(value)
    return value

def validate_result(contract,result):
    validate_shape(result,'task-result.schema.json')
    fixture.validate_result(contract,result)
    return result

if __name__=='__main__':
    task=fixture.example()
    validate_contract(task)
    receipts={}
    for name,options in [('success',{}),('failure',{'fixture':'incorrect'}),('missing',{'missing_evidence':True})]:
        receipt=fixture.FakeController().run(task,fixture.fixture_grant(task),**options)
        validate_result(task,receipt)
        receipts[name]=receipt['terminal']
    receipt=fixture.FakeController().run(task,None)
    validate_result(task,receipt)
    receipts['denial']=receipt['terminal']
    print(json.dumps({'validator':'jsonschema Draft202012Validator','qualification_only':True,'outcomes':receipts},indent=2))
