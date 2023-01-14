import os

n = 10
time = 0
for _ in range(n):
    run = os.popen('cargo run --release')
    info = run.read()
    time += float(''.join(list(info.split()[0])[:-1]))

print(time / n)