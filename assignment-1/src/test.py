# Please don't read my ugly python

import os
import re

n = 10
time = 0
unit = None

os.system('rustc -C opt-level=3 main.rs')
for i in range(n):
    print(f'\r{(i/n)*100:.0f}% Complete', end="")
    info = os.popen('./main').read()
    exec_time = info.split()[0]
    unit = re.sub('\d', '', exec_time)[1:]
    time += float(re.sub('[^0-9|\.]', '', exec_time))

print('\r100% Complete!')
print(f'Average time in {n} computation{"s" if n > 1 else ""}: {(time/n):.2f}{unit}')
