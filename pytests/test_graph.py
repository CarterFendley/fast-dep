import os
import sys
import inspect

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
        },
        {
            'name': 'test_packages.module_dep',
            'dependencies': 1,
            'dependents': {'<terminal>'}
        },
        {
            'name': 'test_packages',
            'dependencies': 0,
            'dependents': {'<terminal>'}
        },
        {
            'name': '<terminal>',
            'dependencies': 3,
            'dependents': set()
        }
    ]

    for expected in expected_nodes:
        actual = graph.get(expected['name'])
        print(expected['name'])
        assert actual.dependencies == expected['dependencies']
        assert actual.dependents == expected['dependents']

