#!/usr/bin/env python

"""A simple module to determine how many characters in a string must be deleted to remove all adjacencies.

Functions:
    find_deletion_number(string: str) -> int:
        Find how many characters must be deleted to remove all sequences of adjacent characters.

    main() -> None:
        Take a string as user input and print out how many characters need to be deleted.
"""

import re


def find_deletion_number(string: str) -> int:
    """Find how many characters must be deleted to remove all sequences of adjacent characters."""
    num = 0

    for char in set(string):
        num += len(''.join(re.findall(f'(?<={char}){char}+', string)))

    return num


def main() -> None:
    """Take a string as user input and print out how many characters need to be deleted."""
    num = find_deletion_number(input("Please enter a string: "))
    print(f'You will need to delete {num} characters from that string to remove adjacencies')


if __name__ == "__main__":
    main()
