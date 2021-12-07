#!/usr/bin/env python
"""A simple script to find the smallest number that can be evenly divided by every number less than the argument."""

from functools import reduce
from sys import argv


def find_prime_factors(n: int) -> list[int]:
    """Return a list of all the prime factors of n."""
    factors = []
    divisor = 2

    while n > 1:
        while n % divisor == 0:
            factors.append(divisor)
            n //= divisor

        divisor += 1
        if divisor * divisor > n > 1:
            factors.append(n)
            break

    return factors


def missing_from_l1(l1: list[int], l2: list[int]) -> list[int]:
    """Return a list of all elements in l2 that are missing from l1. Accounts for repeats."""
    l: list[int] = []
    l1c = l1.copy()

    for v in l2:
        if v in l1c:
            del l1c[[a for a, b in enumerate(l1c) if b == v][-1]]
        else:
            l.append(v)

    return l


def smallest_number_divisible_by_all_lte(n: int) -> int:
    """Return the smallest number which is evenly divisible by all numbers less than n."""
    if n < 2:
        raise ValueError("n must be greater than 2")

    factors: list[int] = []

    for i in range(2, n + 1):
        factors += missing_from_l1(factors, find_prime_factors(i))

    return reduce(lambda a, b: a * b, factors)


if __name__ == "__main__":
    try:
        print(smallest_number_divisible_by_all_lte(int(argv[1])))
    except (IndexError, ValueError):
        print('Script takes integer argument')
