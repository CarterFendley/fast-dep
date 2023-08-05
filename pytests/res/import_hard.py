IMPORTS = [
    {
        'source' :"""
        import _private_module
        """,
        # ----------------
        'expect' : [{
            'type': 'import',
            'names': [
                { 'name': '_private_module' },
            ]
        }]
    },
    {
        'source' :"""
        from _private_module import _private_name
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 0,
            'module': '_private_module',
            'names': [
                { 'name': '_private_name' },
            ]
        }]
    },
    {
        'source' :"""
        import      os
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
        from os import (
            path,
            getcwd,
        )
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
        from os import (
            path,
            getcwd
        )
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
        import os, \
            sys
        """,
        # ----------------
        'expect' : [{
            'type': 'import',
            'names': [
                { 'name': 'os' },
                { 'name': 'sys' },
            ]
        }]
    },
    {
        'source' :"""
        from os import (
            path,
            getcwd,
            getenv,)
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 0,
            'module': 'os',
            'names': [
                { 'name': 'path' },
                { 'name': 'getcwd' },
                { 'name': 'getenv' },
            ]
        }]
    },
]