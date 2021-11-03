#!/usr/bin/env python
"""A simple script to find the prime factors of a number passed as an argument."""

import sys
from math import ceil, sqrt
from os import path


def is_prime(n: int) -> bool:
    """Check if n is a prime number."""
    if n < 2:
        return False

    if n == 2:
        return True

    for i in range(2, ceil(sqrt(n)) + 1):
        if n % i == 0:
            return False

    return True


def find_prime_factors(n: int) -> list[int]:
    """Return a list of prime factors of n."""
    if is_prime(n):
        return [n]

    prime_factors = []
    while not is_prime(n):
        # Find the smallest prime factor, add it to list, and then divide n by it, and keep looping
        factor = [p for p in range(1, n) if is_prime(p) and n % p == 0][0]
        prime_factors.append(factor)

        # We're using // here rather than / to do integer division
        # We should never get a remainder anyway, but this keeps n as an int
        n //= factor

    # We have to remember to add n to the end of the list
    return prime_factors + [n]


def main() -> None:
    """Take input from the command line."""
    try:
        factors = find_prime_factors(int(sys.argv[1]))
        power_factors: list[tuple[int, int]] = []

        # Get each factor with how many times that factor appears
        # This allows us to simplify the factors with powers
        for f in factors:
            t = f, len([x for x in factors if x == f])
            if t not in power_factors:
                power_factors.append(t)

        print(' * '.join([
            str(base) + (('^' + str(exponent)) if exponent > 1 else '')
            for base, exponent in power_factors
        ]))

    except (IndexError, ValueError):
        print(path.split(sys.argv[0])[-1] + ' must be supplied with an integer argument')


if __name__ == "__main__":
    main()
