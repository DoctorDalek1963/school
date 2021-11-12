#!/usr/bin/env python
"""A simple script to find the difference between the square of the sum and the sum of the squares up to n."""

from sys import argv


def square_of_sum(n: int) -> int:
    """Return the square of the sum up to n."""
    return ((n * (n + 1)) // 2)**2


def sum_of_squares(n: int) -> int:
    """Return the sum of squares up to n."""
    return (n * (n + 1) * (2 * n + 1)) // 6


def main():
    """Find the difference between the two sums."""
    try:
        n = int(argv[1])
        print(square_of_sum(n) - sum_of_squares(n))
    except (IndexError, ValueError):
        print('Script needs one integer argument')


if __name__ == "__main__":
    main()
