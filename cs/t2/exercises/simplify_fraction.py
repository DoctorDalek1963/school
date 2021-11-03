#!/usr/bin/env python
"""A script to take a numerator and denominator and simplify their fraction."""

import math
import sys
from os import path


def main() -> None:
    """Take a numerator and denominator as command line arguments and simplify the fraction."""
    try:
        a, b = int(sys.argv[1]), int(sys.argv[2])
        gcd = math.gcd(a, b)
        print(a // gcd, b // gcd, sep='/')
    except (IndexError, ValueError):
        print(path.split(sys.argv[0])[-1] + ' must be called with two integer arguments')


if __name__ == "__main__":
    main()
