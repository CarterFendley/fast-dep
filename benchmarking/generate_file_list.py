#!/usr/bin/env python
import sys
from importlib.util import find_spec

from fast_dep import GraphBuilder

module = sys.argv[1]
spec = find_spec(module)
target = spec.origin

# Do it!
b = GraphBuilder()
with open(spec.origin) as f:
    source = f.read()
graph = b.build(
    source=source
)

with open('.file_list.txt', 'w') as f:
    for i, origin in enumerate(graph.origins()):
        if origin.endswith('.py'):
            if i != 0:
                f.write('\n')
            f.write(origin)