import os
import sys
import inspect
import logging

THIS_DIR = os.path.abspath(os.path.dirname(__file__))
RES_DIR = os.path.abspath(
    os.path.join(THIS_DIR, 'res')
)
sys.path.append(RES_DIR)

from test_packages.imports import *
from fast_dep import GraphBuilder

def test_module_dep():
    builder = GraphBuilder()
    print(inspect.getsource(import_module_dep))
    graph = builder.build(
        inspect.getsource(import_module_dep),
        package='test_packages'
    )

    assert graph.size() == 4

    expected_nodes = [
        {
            'name': 'test_packages.module_dep.file',
            'dependencies': 0,
            'dependents': {'test_packages.module_dep'},
            'depth': 2
        },
        {
            'name': 'test_packages.module_dep',
            'dependencies': 1,
            'dependents': {'<terminal>'},
            'depth': 1
        },
        {
            'name': 'test_packages',
            'dependencies': 0,
            'dependents': {'<terminal>'},
            'depth': 1
        },
        {
            'name': '<terminal>',
            'dependencies': 3,
            'dependents': set(),
            'depth': 0
        }
    ]

    for expected in expected_nodes:
        actual = graph.get(expected['name'])
        print(expected['name'])
        assert actual.dependencies == expected['dependencies']
        assert actual.dependents == expected['dependents']
        assert actual.depth == expected['depth']

