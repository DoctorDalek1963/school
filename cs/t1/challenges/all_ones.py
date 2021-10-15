#!/usr/bin/env python

"""A simple module to find the next multiple of a number that consists of only ones.

Functions:
    find_multiple(n: int) -> int:
        Find the next multiple of n that is only ones.

    main() -> None:
        Take user input for a number and print the next multiple of that number that is all ones.
"""


def find_multiple(n: int) -> int:
    """Find the next multiple of n that is only ones."""
    num = n
    while True:
        num += n
        if set(str(num)) == {'1'}:
            return num


def main() -> None:
    """Take user input for a number and print the next multiple of that number that is all ones."""
    num_string = input('Please enter a number: ')

    while not num_string.isdigit():
        num_string = input('Please try again: ')

    num = int(num_string)

    ones = find_multiple(num)
    print(f'The next multiple of {num} that is all ones is {num} * {ones // num} = {ones}')


if __name__ == "__main__":
    main()
