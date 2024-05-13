#!/usr/bin/env python
import re
from util import Timer

timer = Timer()
with timer:
    # Do it!
    with open('.file_list.txt', 'r') as f:
        files = f.read().split('\n')

    for file in files:
        with open(file, 'r') as f:
            re.findall(r'import', f.read())

print(f"Read graph files: {timer.latest_cumtime}")
