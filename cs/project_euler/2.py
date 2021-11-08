#!/usr/bin/env python

from sys import argv

def fib():
    a = 0
    b = 1
    while True:
        a, b = a + b, a
        yield a


f = fib()
n = 0
t = 0

while n < int(argv[1]):
    n = next(f)
    if n % 2 == 0:
        t += n

print(t)
