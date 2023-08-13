import os
import sys
import inspect
import logging

from pytest_unordered import unordered

THIS_DIR = os.path.abspath(os.path.dirname(__file__))
RES_DIR = os.path.abspath(
    os.path.join(THIS_DIR, '../res')
)
sys.path.append(RES_DIR)

import bunch_of_imports
from fast_dep import GraphBuilder

def test_bunch():
    builder = GraphBuilder()
    print(inspect.getsource(bunch_of_imports))
    graph = builder.build(
        inspect.getsource(bunch_of_imports),
        package=''
    )

    # Test caching
    graph = builder.build(
        inspect.getsource(bunch_of_imports),
        package=''
    )