import os, pathlib, subprocess, sys
ROOT=pathlib.Path(__file__).resolve().parent
ENV={"PATH":str(ROOT/"bin")+":/usr/bin:/bin", "XDG_CONFIG_HOME":str(ROOT/"config"), "XDG_STATE_HOME":str(ROOT/"state"), "XDG_DATA_HOME":str(ROOT/"data"), "XDG_CACHE_HOME":str(ROOT/"cache"), "TMPDIR":str(ROOT/"tmp"), "OPENSHELL_TELEMETRY_ENABLED":"false", "OPENSHELL_SYSTEM_GATEWAY_DIR":str(ROOT/"system-config"), "OPENSHELL_COLOR":"never"}
def run(args, **kwargs):
    return subprocess.run(args,env=ENV,cwd=ROOT,**kwargs)
if __name__ == "__main__":
    raise SystemExit(run(sys.argv[1:]).returncode)
