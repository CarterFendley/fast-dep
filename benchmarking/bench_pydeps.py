#!/usr/bin/env python
import sys
from util import Timer
from importlib.util import find_spec

import pydeps.cli
from pydeps.py2depgraph import py2dep
from pydeps.target import Target

module = sys.argv[1]
spec = find_spec(module)
target = spec.origin

def empty(args="", **kw):
    args = pydeps.cli.parse_args([
        'foo',
        '--no-config',
        '--pylib',
        # '--max-bacon', '0', # Zero is infinite
        '--noise-level', '10000',
        '--max-module-depth', '10000'
    ] + args.split())
    args.pop('fname')
    args.update(kw)
    return args

t = Target(target)
with t.chdir_work():
    timer = Timer()
    with timer:
        # Do it!
        res = py2dep(target=t, **empty())

        if False: # For debugging
            graph = {"%s -> %s" % (a.name, b.name) for a, b in res}
            print(graph)

    print(f"Traced module '{module}': {timer.latest_cumtime}")
    print(f'\tGraph Size: {len(list(iter(res)))}')