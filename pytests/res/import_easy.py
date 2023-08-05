IMPORTS = [
    {
        'source' :"""
        import os
        """,
        # ----------------
        'expect' : [{
            'type': 'import',
            'names': [
                { 'name': 'os' },
            ]
        }]
    },
    {
        'source' :"""
        import os,sys
        """,
        # ----------------
        'expect' : [{
            'type': 'import',
            'names': [
                { 'name': 'os' },
                { 'name': 'sys'},
            ]
        }]
    },
    {
        'source' :"""
        import os, sys
        """,
        # ----------------
        'expect' : [{
            'type': 'import',
            'names': [
                { 'name': 'os' },
                { 'name': 'sys'},
            ]
        }]
    },
    {
        'source' :"""
        import os as a, sys as b
        """,
        # ----------------
        'expect' : [{
            'type': 'import',
            'names': [
                { 'name': 'os', 'asname': 'a' },
                { 'name': 'sys', 'asname': 'b' },
            ]
        }]
    },
    {
        'source' :"""
        from os import path
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 0,
            'module': 'os',
            'names': [
                { 'name': 'path' },
            ]
        }]
    },
    {
        'source' :"""
        from os import path, getcwd
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 0,
            'module': 'os',
            'names': [
                { 'name': 'path' },
                { 'name': 'getcwd' },
            ]
        }]
    },
    {
        'source' :"""
        from os import path as p, getcwd as whereami
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 0,
            'module': 'os',
            'names': [
                { 'name': 'path', 'asname': 'p' },
                { 'name': 'getcwd', 'asname': 'whereami' },
            ]
        }]
    },
    {
        'source' :"""
        from . import blah
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 1,
            'module': '',
            'names': [
                { 'name': 'blah' },
            ]
        }]
    },
    {
        'source' :"""
        from ...module import blah
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 3,
            'module': 'module',
            'names': [
                { 'name': 'blah' },
            ]
        }]
    }
]