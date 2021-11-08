#!/usr/bin/env python
"""A simple script to find the sum of even Fibonacci numbers less than the argument."""

from sys import argv


def fib():
    """Generate Fibonacci numbers forever."""
    a = 0
    b = 1
    while True:
        a, b = a + b, a
        yield a


def main() -> None:
    """Find the sum of even Fibonacci numbers less than argv[1]."""
    f = fib()
    n = 0
    t = 0

    while n < int(argv[1]):
        n = next(f)
        if n % 2 == 0:
            t += n

    print(t)


if __name__ == "__main__":
    main()
