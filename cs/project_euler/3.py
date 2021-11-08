#!/usr/bin/env python
"""A simple script to find the largest prime factor of a number."""

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
        if divisor * divisor > n:
            if n > 1:
                factors.append(n)
                break

    return factors


if __name__ == "__main__":
    try:
        print(max(find_prime_factors(int(argv[1]))))
    except IndexError:
        print('Script must be called with an integer')
