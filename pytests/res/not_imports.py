NOT_IMPORTS = [
    {
        'source' :'''
        """
        import os
        from os import path
        """
        ''',
        # ----------------
        'expect' : []
    },
    {
        'source' :"""
        '''
        import os
        from os import path
        '''
        """,
        # ----------------
        'expect' : []
    },
    # Honestly need to think about if I need to detect if the below are valid python or not too.
    {
        'source' :"""
        import (os, sys)
        """,
        # ----------------
        'expect' : []
    },
    {
        'source' :"""
        importos
        """,
        # ----------------
        'expect' : []
    },
    {
        'source' :"""
        fromosimportpath
        """,
        # ----------------
        'expect' : []
    },
    # Currently failing need to patch grammar
    # {
    #     'source' :"""
    #     import os,
    #         sys
    #     """,
    #     # ----------------
    #     'expect' : []
    # },
    {
        'source' :"""
        from os import (
            
        )
        """,
        # ----------------
        'expect' : []
    },
]