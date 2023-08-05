# Hacky until the following gets patched
# - https://github.com/PyO3/pyo3/issues/759
import fast_dep
parser = fast_dep.parser

from res.import_easy import IMPORTS as IMPORTS_EASY
from res.import_hard import IMPORTS as IMPORTS_HARD
from res.import_wtf import IMPORTS as IMPORTS_WTF
from res.not_imports import NOT_IMPORTS

def test_easy():
    for variant in IMPORTS_EASY:
        parsed = parser.parse(variant['source'])

        assert len(parsed) == len(variant['expect'])
        assert parsed == variant['expect']

def test_hard():
    for variant in IMPORTS_HARD:
        parsed = parser.parse(variant['source'])

        assert len(parsed) == len(variant['expect'])
        assert parsed == variant['expect']

def test_wtf():
    for variant in IMPORTS_WTF:
        parsed = parser.parse(variant['source'])

        assert len(parsed) == len(variant['expect'])
        assert parsed == variant['expect']

def test_wtf():
    for variant in NOT_IMPORTS:
        parsed = parser.parse(variant['source'])

        assert len(parsed) == len(variant['expect'])
        assert parsed == variant['expect']