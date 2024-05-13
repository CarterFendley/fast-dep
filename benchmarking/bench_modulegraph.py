#!/usr/bin/env python
import sys
from util import Timer
from importlib.util import find_spec

from modulegraph.find_modules import find_modules

module = sys.argv[1]
spec = find_spec(module)
target = spec.origin

timer = Timer()
with timer:
    # Do it!
    g = find_modules(scripts=[target])

print(f"Traced module '{module}': {timer.latest_cumtime}")