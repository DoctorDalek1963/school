#!/usr/bin/env python
"""A simple script to convert a decimal number to binary."""

import sys
from os import path


def dec_to_binary(n: int) -> str:
    """Convert a decimal number to binary."""
    binary = ''

    while n != 0:
        binary = str(n % 2) + binary
        n //= 2

    return binary


def main() -> None:
    """Get a decimal input from the command line."""
    try:
        print(dec_to_binary(int(sys.argv[1])))
    except (IndexError, ValueError):
        print(path.split(sys.argv[0])[-1] + ' must be called with a base-10 integer argument')


if __name__ == "__main__":
    main()
