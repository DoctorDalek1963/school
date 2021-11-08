#!/usr/bin/env python
"""A simple script to find the largest palindrome that is the product of two 3-digit numbers."""


def find_palindrome_products() -> list[int]:
    """Find the largets palindrome which is a product of two 3-digit numbers."""
    l: list[int] = []

    for i in range(900, 1000):
        for j in range(i, 1000):
            p = i * j
            if str(p) == str(p)[::-1]:
                l.append(p)

    return l


if __name__ == "__main__":
    print(max(find_palindrome_products()))
