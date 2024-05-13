IMPORTS = [
    {
        'source' :"""
        from os import (
            path, # Lolz
        # Yo

                # Yo
            getcwd    ,
            getenv,
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
                { 'name': 'getenv' },
            ]
        }]
    },
    {
        'source' :"""
        import \
            os
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
        from\
            pathlib\
            import\
                Path
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 0,
            'module': 'pathlib',
            'names': [
                { 'name': 'Path' },
            ]
        }]
    },
    {
        'source' :"""
        from path\
            import\
                lib
        """,
        # ----------------
        'expect' : [{
            'type': 'import_from',
            'level': 0,
            'module': 'path',
            'names': [
                { 'name': 'lib' },
            ]
        }]
    },
]