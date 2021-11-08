#!/usr/bin/env python
"""A simple script to print the sum of multiples of 3 or 5 less than the argument."""

from sys import argv

if __name__ == "__main__":
    try:
        print(sum([x for x in range(int(argv[1])) if x % 3 == 0 or x % 5 == 0]))
    except (IndexError, ValueError):
        print('Script must be called with an integer')
