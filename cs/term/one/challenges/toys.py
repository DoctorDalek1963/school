#!/usr/bin/env python

"""A simple module to maximise the amount of toys that can be bought within a budget, given a list of prices.

Functions:
    maximise_toys(budget: float, prices: list[float]) -> list[float]:
        Determine the maximum number of toys that can be bought.

    take_toy_input() > list[float]:
        Take user input to get a budget and a list of prices.

    main() -> None:
        Take input and print out how many toys can be bought.
"""


def maximise_toys(budget: float, prices: list[float]) -> list[float]:
    """Determine the maximum number of toys that can be bought."""
    l: list[float] = []

    for p in sorted(prices):
        if sum(l) + p < budget:
            l.append(p)
        else:
            break

    return l


def take_toy_input() -> list[float]:
    """Take user input to get a budget and a list of prices."""
    budget_string = input('Please enter a budget (without the £): ')

    while not budget_string.replace('.', '').isdigit():
        budget_string = input('Please try again: ')

    budget = float(budget_string)
    prices: list[float] = []

    while True:
        price_string = input('Please enter a price (blank to end): ')

        if price_string == '':
            break

        while not price_string.replace('.', '').isdigit():
            price_string = input('Please try again: ')

        prices.append(float(price_string))

    return maximise_toys(budget, prices)


def main() -> None:
    """Take input and print out how many toys can be bought."""
    final_prices = take_toy_input()
    print()
    print(f'You can buy {len(final_prices)} toys with that')
    print(f'They will cost £{sum(final_prices):.2f}')
    print(f'These prices are [{", ".join(["£" + str(f"{p:.2f}") for p in final_prices])}]')


if __name__ == "__main__":
    main()
