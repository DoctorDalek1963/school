#!/usr/bin/env python
"""A script to find a, b, c where a^2 + b^2 = c^2, and a + b + c = 1000"""

from math import sqrt


def main() -> None:
    """Find a, b, c where a^2 + b^2 = c^2, and a + b + c = 1000"""
    for b in range(2, 999):
        for a in range(b):
            c = sqrt(a * a + b * b)
            if c == int(c) and a + b + c == 1000:
                print(a * b * int(c))
                return


if __name__ == "__main__":
    main()
