#!/usr/bin/env python
import sys
from util import Timer
from importlib.util import find_spec

from fast_dep import GraphBuilder

module = sys.argv[1]
spec = find_spec(module)
target = spec.origin

timer = Timer()
with timer:
    # Do it!
    b = GraphBuilder()
    with open(spec.origin) as f:
        source = f.read()
    graph = b.build(
        source=source
    )

print(f"Traced module '{module}': {timer.latest_cumtime}")
print(f'\tGraph Size: {graph.size()}')