#!/usr/bin/env python

"""A simple module containing only maximise_toys(prices: list[float], budget: float),
a function to maximise the number of toys that can be bought."""


def maximise_toys(prices: list[float], budget: float) -> tuple[int, float, list[float]]:
    """Determine the maximum number of toys that can be bought."""
    l: list[float] = []
    total = 0.0

    for p in sorted(prices):
        if total + p < budget:
            total += p
            l.append(p)
        else:
            break

    return len(l), total, l


if __name__ == "__main__":
    print(maximise_toys([1, 2, 3, 4], 7))
