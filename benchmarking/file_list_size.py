#!/usr/bin/env python
import os
from util import Timer

timer = Timer()
tot_bytes = 0
with timer:
    # Do it!
    with open('.file_list.txt', 'r') as f:
        files = f.read().split('\n')

    for file in files:
        tot_bytes += os.path.getsize(file)

print(f'Total bytes across files {tot_bytes:,}')
