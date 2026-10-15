"""Isolated deterministic simulator: no process, network, model or canonical DB."""
import copy
from dataclasses import dataclass
from enum import Enum
import hashlib
import json
import sqlite3
from typing import Protocol
import validate as v

WORKER = 'actor_01ARZ3NDEKTSV4RRFFQ69G5FAW'
VERIFIER = 'actor_01ARZ3NDEKTSV4RRFFQ69G5FAV'
CONTROLLER = 'actor_01ARZ3NDEKTSV4RRFFQ69G5FAX'

class RefusalCode(str, Enum):
    INVALID = 'invalid'
    DENIED = 'denied'
    UNSUPPORTED = 'unsupported'
    CONFLICT = 'conflict'
    UNKNOWN = 'unknown'

class Refusal(ValueError):
    def __init__(self, code):
        self.code = code
        super().__init__(code.value)

class AckLost(Exception):
    pass

class Phase(str, Enum):
    PREPARED = 'PREPARED'
    DISPATCHING = 'DISPATCHING'
    RUNNING = 'RUNNING'
    STOP_UNKNOWN = 'STOP_UNKNOWN'
    STOPPED = 'STOPPED'
    FINALIZED = 'FINALIZED'
    RELEASED = 'RELEASED'

@dataclass(frozen=True)
class Prepared:
    run_id: str
    handle_sha256: str

@dataclass(frozen=True)
class PreparedContext:
    run_id: str
    handle_sha256: str
    contract_sha256: str
    source_sha256: str
    profile_sha256: str
    contract_json: str
    def decoded_contract(self):
        return json.loads(self.contract_json) # each caller gets its own copy

@dataclass(frozen=True)
class Candidate:
    path: str
    data: bytes
    claimed_sha256: str

@dataclass(frozen=True)
class CheckEvidence:
    criterion_id: str
    check_id: str
    check_version: int
    passed: bool

@dataclass(frozen=True)
class GateReceipt:
    contract_sha256: str
    artifact_sha256: str
    verifier_actor_id: str
    checks: tuple[CheckEvidence, ...]

class WorkerAdapter(Protocol):
    def describe(self) -> dict: ...
    def prepare(self, context: PreparedContext) -> None: ...
    def start(self, context: PreparedContext) -> None: ...
    def observe(self, handle: Prepared) -> str | None: ...
    def cancel(self, handle: Prepared) -> bool: ...
    def collect(self, handle: Prepared) -> Candidate: ...
    def release(self, handle: Prepared) -> None: ...

class Journal:
    """Real SQLite persistence of fixture records; not Rikuri canonical state."""
    def __init__(self, path):
        self.db = sqlite3.connect(path)
        self.db.execute('PRAGMA synchronous=FULL')
        self.db.executescript("""
            CREATE TABLE IF NOT EXISTS fixture_runs(id TEXT PRIMARY KEY, data TEXT NOT NULL);
            CREATE TABLE IF NOT EXISTS fixture_events(run TEXT, seq INTEGER, data TEXT,
                PRIMARY KEY(run,seq));
            CREATE TABLE IF NOT EXISTS fixture_runtime(id TEXT PRIMARY KEY, token TEXT,
                phase TEXT, bytes BLOB, claimed TEXT);
        """)
    def close(self):
        self.db.close()
    def load(self, run_id):
        row = self.db.execute('SELECT data FROM fixture_runs WHERE id=?',(run_id,)).fetchone()
        return json.loads(row[0]) if row else None
    def save(self, state, kind, tick):
        row = self.db.execute('SELECT COALESCE(MAX(seq),0)+1 FROM fixture_events WHERE run=?',
                              (state['run_id'],)).fetchone()
        event = {'schema_version':1,'run_id':state['run_id'],'sequence':row[0],
                 'logical_tick':tick,'kind':kind,'producer_actor_id':CONTROLLER,
                 'contract_sha256':state['contract_sha256']}
        v.validate_shape(event,'adapter-event.schema.json')
        v.validate_shape(self.view(state),'adapter-lifecycle.schema.json')
        with self.db:
            self.db.execute('INSERT INTO fixture_runs VALUES(?,?) ON CONFLICT(id) DO UPDATE SET data=excluded.data',
                            (state['run_id'],json.dumps(state,sort_keys=True)))
            self.db.execute('INSERT INTO fixture_events VALUES(?,?,?)',
                            (state['run_id'],row[0],json.dumps(event,sort_keys=True)))
    @staticmethod
    def view(state):
        keys=('schema_version','task_id','run_id','handle_sha256','contract_sha256',
              'source_sha256','phase','execution','acceptance','application','terminal','cancel_requested')
        return {key:state[key] for key in keys}
    def events(self, run_id, cursor=0, limit=128):
        if type(cursor) is not int or cursor < 0 or type(limit) is not int or not 1 <= limit <= 128:
            raise Refusal(RefusalCode.INVALID)
        last=self.db.execute('SELECT COALESCE(MAX(seq),0) FROM fixture_events WHERE run=?',(run_id,)).fetchone()[0]
        if cursor > last: raise Refusal(RefusalCode.CONFLICT)
        rows=self.db.execute('SELECT data FROM fixture_events WHERE run=? AND seq>? ORDER BY seq LIMIT ?',
                             (run_id,cursor,limit)).fetchall()
        return tuple(json.loads(row[0]) for row in rows)

class FixtureAuthority:
    """Runner-owned authority/virtual clock; NOT authenticated production grants."""
    def __init__(self, source):
        self.source=source
        self.now=0
        self.grants={}
        self.release_permits=set()
    def issue(self, task):
        v.validate_contract(task)
        token=v.digest({'grant':len(self.grants)+1,'contract':v.digest(task)})
        self.grants[token]={'digest':v.digest(task),'expires':self.now+task['limits']['wall_seconds'],
                            'revoked':False}
        return token
    def revoke(self, token):
        self.grants[token]['revoked']=True
    def check(self, token, task):
        grant=self.grants.get(token)
        if not grant or grant['digest']!=v.digest(task) or grant['revoked'] or self.now>=grant['expires']:
            raise Refusal(RefusalCode.DENIED)
        if hashlib.sha256(self.source).hexdigest()!=task['source']['revision']:
            raise Refusal(RefusalCode.CONFLICT)
        caps=task['requested_capabilities']
        if (caps['network'] or caps['paid_inference'] or task['privacy']=='local_only'
            or task['limits']['max_cost_microusd']!=0 or task['limits']['max_attempts']!=1
            or set(caps['read_paths'])!={'mean.py'} or set(caps['write_paths'])!={'mean.py'}
            or set(caps['execute_checks'])!=set(v.CHECKS)
            or task['independent_gate']['gate_id']!='mean-acceptance-v1'
            or task['independent_gate']['verifier_actor_id']!=VERIFIER):
            raise Refusal(RefusalCode.UNSUPPORTED)
    def permit_release(self, run_id):
        token=v.digest({'release':run_id})
        self.release_permits.add((run_id,token))
        return token

class DeterministicAdapter:
    """Durable fake runtime effects only. Never evaluates candidate bytes."""
    def __init__(self,journal,*,lost_ack=None,query=True,cancellation=True):
        self.journal=journal
        self.lost_ack=lost_ack
        self.query=query
        self.cancellation=cancellation
        self.start_calls=0
    def describe(self):
        return {'version':1,'scope':'synthetic_only','query':self.query,'cancel':self.cancellation}
    def prepare(self,context):
        if context.decoded_contract()['execution_profile']['runtime']!='in-memory-python':
            raise Refusal(RefusalCode.UNSUPPORTED)
    def start(self,context):
        self.start_calls+=1
        if self.lost_ack=='before': raise AckLost()
        run=context.run_id;token=context.handle_sha256
        row=self.journal.db.execute('SELECT token FROM fixture_runtime WHERE id=?',(run,)).fetchone()
        if row and row[0]!=token: raise Refusal(RefusalCode.CONFLICT)
        if not row:
            with self.journal.db:
                self.journal.db.execute('INSERT INTO fixture_runtime VALUES(?,?,?,?,?)',
                                        (run,token,'RUNNING',None,None))
        if self.lost_ack=='after': raise AckLost()
    def _runtime(self,handle):
        row=self.journal.db.execute('SELECT token,phase,bytes,claimed FROM fixture_runtime WHERE id=?',(handle.run_id,)).fetchone()
        if row and row[0]!=handle.handle_sha256:raise Refusal(RefusalCode.CONFLICT)
        return row
    def observe(self,handle):
        if not self.query:raise Refusal(RefusalCode.UNSUPPORTED)
        row=self._runtime(handle)
        return row[1] if row else None
    def simulate_finish(self,run_id,*,incorrect=False,unknown_bytes=False,bad_manifest=False):
        data=b'not a supported fixture' if unknown_bytes else (v.BAD_SOURCE if incorrect else v.GOOD_SOURCE).encode()
        hashed='0'*64 if bad_manifest else hashlib.sha256(data).hexdigest()
        with self.journal.db:
            changed=self.journal.db.execute("UPDATE fixture_runtime SET phase='COMPLETED',bytes=?,claimed=? WHERE id=? AND phase='RUNNING'",
                                            (data,hashed,run_id)).rowcount
        if changed!=1: raise Refusal(RefusalCode.CONFLICT)
    def cancel(self,handle):
        if not self.cancellation:return False
        self._runtime(handle)
        with self.journal.db:
            changed=self.journal.db.execute("UPDATE fixture_runtime SET phase='CANCELLED' WHERE id=? AND token=? AND phase='RUNNING'",(handle.run_id,handle.handle_sha256)).rowcount
        return bool(changed)
    def collect(self,handle):
        row=self._runtime(handle)
        if not row or row[1]!='COMPLETED':raise Refusal(RefusalCode.UNKNOWN)
        return Candidate('mean.py',bytes(row[2]),row[3])
    def release(self,handle):
        self._runtime(handle)
        # Preserve all artifacts/receipts. No deletion; logical release only.
        return None

class IndependentFixtureGate:
    """Trusted paired-source checker; not arbitrary generated-code execution."""
    def __init__(self,*,missing=False):
        self.missing=missing
    def verify(self,task,candidate):
        function={v.GOOD_SOURCE.encode():v.good_mean,v.BAD_SOURCE.encode():v.bad_mean}.get(candidate.data)
        if function is None:raise Refusal(RefusalCode.UNSUPPORTED)
        checks=tuple(CheckEvidence(item['id'],item['check_id'],1,v.CHECKS[item['check_id']](function))
                     for item in task['acceptance'])
        if self.missing:checks=checks[:-1]
        return GateReceipt(v.digest(task),hashlib.sha256(candidate.data).hexdigest(),VERIFIER,checks)

class Controller:
    def __init__(self,journal,authority,adapter,gate):
        self.journal=journal;self.authority=authority;self.adapter=adapter;self.gate=gate
    @staticmethod
    def _context(state):
        return PreparedContext(state['run_id'],state['handle_sha256'],state['contract_sha256'],
                               state['source_sha256'],v.digest(state['task']['execution_profile']),
                               json.dumps(state['task'],sort_keys=True))
    def _state(self,handle):
        state=self.journal.load(handle.run_id)
        if not state or state['handle_sha256']!=handle.handle_sha256:raise Refusal(RefusalCode.CONFLICT)
        return state
    def status(self,handle):
        return self.journal.view(self._state(handle))
    def prepare(self,task,grant):
        try:v.validate_contract(task)
        except v.ContractError:raise Refusal(RefusalCode.INVALID) from None
        self.authority.check(grant,task)
        frozen=json.loads(json.dumps(task))
        bound=v.digest({'task':frozen,'grant':grant})
        old=self.journal.load(task['run_id'])
        if old:
            if old['handle_sha256']!=bound:raise Refusal(RefusalCode.CONFLICT)
            return Prepared(old['run_id'],bound)
        state={'schema_version':1,'task_id':task['task_id'],'run_id':task['run_id'],
               'handle_sha256':bound,'contract_sha256':v.digest(frozen),'source_sha256':task['source']['revision'],
               'phase':'PREPARED','execution':'not_started','acceptance':'unverified',
               'application':'not_applied','terminal':'NONE','cancel_requested':False,'task':frozen,'grant':grant,'captured':None,'gate_receipt':None}
        self.adapter.prepare(self._context(state))
        self.journal.save(state,'prepared',self.authority.now)
        return Prepared(state['run_id'],bound)
    def start(self,handle):
        state=self._state(handle)
        if state['phase']!='PREPARED':return self.journal.view(state)
        try:self.authority.check(state['grant'],state['task'])
        except Refusal:
            state.update(phase='FINALIZED',terminal='BLOCKED')
            self.journal.save(state,'denied',self.authority.now)
            raise
        state.update(phase='DISPATCHING',execution='unknown')
        self.journal.save(state,'dispatch_intent',self.authority.now) # commit BEFORE effect
        try:self.adapter.start(self._context(state))
        except AckLost:return self.journal.view(state) # never resend
        state.update(phase='RUNNING',execution='running')
        self.journal.save(state,'started',self.authority.now)
        return self.journal.view(state)
    def reconcile(self,handle):
        state=self._state(handle)
        if state['phase'] not in ('DISPATCHING','RUNNING','STOP_UNKNOWN'):return self.journal.view(state)
        try:observed=self.adapter.observe(handle)
        except Refusal:observed=None
        if observed=='COMPLETED':
            state.update(phase='FINALIZED' if state['cancel_requested'] else 'STOPPED',execution='completed',
                         terminal='CANCELLED' if state['cancel_requested'] else 'NONE')
        elif observed=='CANCELLED':state.update(phase='FINALIZED',execution='cancelled',terminal='CANCELLED')
        elif observed=='RUNNING':
            state.update(phase='STOP_UNKNOWN' if state['cancel_requested'] else 'RUNNING',
                         execution='unknown' if state['cancel_requested'] else 'running')
        else:state.update(phase='STOP_UNKNOWN',execution='unknown')
        self.journal.save(state,'reconciled',self.authority.now)
        return self.journal.view(state)
    def observe(self,handle,cursor=0,limit=128):
        self._state(handle)
        return self.journal.events(handle.run_id,cursor,limit)
    def cancel(self,handle):
        state=self._state(handle)
        if state['phase'] in ('FINALIZED','RELEASED'):return self.journal.view(state)
        state['cancel_requested']=True
        self.journal.save(state,'cancel_requested',self.authority.now)
        if state['phase']=='PREPARED':state.update(phase='FINALIZED',terminal='CANCELLED')
        elif state['phase']=='STOPPED':state.update(phase='FINALIZED',terminal='CANCELLED')
        else:
            try:stopped=self.adapter.cancel(handle)
            except Refusal:stopped=False
            if stopped:state.update(phase='FINALIZED',execution='cancelled',terminal='CANCELLED')
            else:state.update(phase='STOP_UNKNOWN',execution='unknown')
        self.journal.save(state,'cancel_observed',self.authority.now)
        return self.journal.view(state)
    def collect(self,handle):
        state=self._state(handle)
        if state['phase']!='STOPPED':raise Refusal(RefusalCode.UNKNOWN)
        self.authority.check(state['grant'],state['task'])
        if state['captured'] is not None:
            captured=state['captured']
            return Candidate('mean.py',bytes.fromhex(captured['hex']),captured['hash'])
        candidate=self.adapter.collect(handle)
        hashed=hashlib.sha256(candidate.data).hexdigest()
        if candidate.path!='mean.py' or len(candidate.data)>65536 or hashed!=candidate.claimed_sha256:
            raise Refusal(RefusalCode.INVALID)
        state['captured']={'hex':candidate.data.hex(),'hash':hashed}
        self.journal.save(state,'captured',self.authority.now)
        return Candidate(candidate.path,candidate.data,hashed)
    def verify(self,handle):
        state=self._state(handle)
        if state['phase']=='FINALIZED':return self.journal.view(state)
        if state['phase']!='STOPPED':raise Refusal(RefusalCode.UNKNOWN)
        try:
            candidate=self.collect(handle)
            self.authority.check(state['grant'],state['task'])
            receipt=self.gate.verify(state['task'],candidate)
            payload={'schema_version':1,'contract_sha256':receipt.contract_sha256,
                     'artifact_sha256':receipt.artifact_sha256,'verifier_actor_id':receipt.verifier_actor_id,
                     'checks':[vars(check) for check in receipt.checks]}
            v.validate_shape(payload,'adapter-gate-receipt.schema.json')
            expected={(c['id'],c['check_id']) for c in state['task']['acceptance']}
            actual={(c.criterion_id,c.check_id) for c in receipt.checks}
            if (receipt.contract_sha256!=state['contract_sha256'] or receipt.artifact_sha256!=candidate.claimed_sha256
                or receipt.verifier_actor_id!=VERIFIER or actual!=expected or len(receipt.checks)!=len(expected)
                or any(c.check_version!=1 for c in receipt.checks)):
                raise Refusal(RefusalCode.INVALID)
            self.authority.check(state['grant'],state['task'])
            state['gate_receipt']=payload
            passed=all(c.passed for c in receipt.checks)
            state.update(acceptance='passed' if passed else 'failed',terminal='DONE' if passed else 'BLOCKED')
        except (Refusal,v.ContractError):
            state.update(acceptance='unverified',terminal='UNVERIFIED')
        # Reload captured bytes, preserving trusted capture when finalizing.
        captured=self.journal.load(state['run_id'])['captured']
        state.update(captured=captured,phase='FINALIZED')
        self.journal.save(state,'verified',self.authority.now)
        return self.journal.view(state)
    def evidence(self,handle):
        return copy.deepcopy(self._state(handle)['gate_receipt'])
    def release(self,handle,permit):
        state=self._state(handle)
        if (state['phase'] not in ('FINALIZED','RELEASED') or state['execution']=='unknown'
            or (handle.run_id,permit) not in self.authority.release_permits):
            raise Refusal(RefusalCode.DENIED)
        if state['phase']=='RELEASED':return self.journal.view(state)
        self.adapter.release(handle)
        state['phase']='RELEASED'
        self.journal.save(state,'released',self.authority.now)
        return self.journal.view(state)

def example():
    task=v.example()
    source=v.BAD_SOURCE.encode()
    task['source']['revision']=hashlib.sha256(source).hexdigest()
    return task,source

def demonstration():
    from pathlib import Path
    import tempfile
    outputs={}
    for scenario in ['success','gate_failed','lost_ack','cancel_unknown','denied']:
        with tempfile.TemporaryDirectory() as scratch:
            journal=Journal(Path(scratch)/'synthetic.sqlite')
            try:
                task,source=example()
                authority=FixtureAuthority(source)
                adapter=DeterministicAdapter(journal,lost_ack='after' if scenario=='lost_ack' else None,
                                             query=scenario!='lost_ack',cancellation=scenario!='cancel_unknown')
                controller=Controller(journal,authority,adapter,IndependentFixtureGate())
                grant=authority.issue(task)
                try:
                    handle=controller.prepare(task,'not-issued' if scenario=='denied' else grant)
                    controller.start(handle)
                    if scenario=='lost_ack':
                        controller.start(handle)
                        controller.reconcile(handle)
                    elif scenario=='cancel_unknown':
                        controller.cancel(handle)
                    else:
                        adapter.simulate_finish(handle.run_id,incorrect=scenario=='gate_failed')
                        controller.reconcile(handle)
                        controller.verify(handle)
                    outputs[scenario]={'status':controller.status(handle),'fake_start_calls':adapter.start_calls}
                except Refusal as refused:
                    outputs[scenario]={'refusal':refused.code.value,'fake_start_calls':adapter.start_calls}
            finally:journal.close()
    return {'synthetic_only':True,'scenarios':outputs}

if __name__=='__main__':
    print(json.dumps(demonstration(),indent=2))
