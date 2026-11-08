"""Check CLI behavior without changing the desktop input source."""

import subprocess
import sys

binary = sys.argv[1] if len(sys.argv) > 1 else "./target/release/im-switch"
cases = [
    ([], 0),
    (["--help"], 0),
    (["-h"], 0),
    (["--unknown"], 2),
    (["--english", "--status"], 2),
    (["ctrl+space"], 2),
    (["--request-access"], 2),
]
for args, expected in cases:
    result = subprocess.run([binary, *args], capture_output=True, text=True)
    assert result.returncode == expected, (args, result.returncode, result.stderr)
    output = result.stdout if expected == 0 else result.stderr
    assert "Usage: im-switch --english" in output, (args, output)
