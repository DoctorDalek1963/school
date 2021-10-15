#!/usr/bin/env python

"""A tiny module containing only find_deletion_number(s: str), a function to find how many
A and B characters in s must be deleted to remove all sequences of adjacent characters."""

import re


def find_deletion_number(s: str) -> int:
    """Find how many A and B characters must be deleted to remove all sequences of adjacent characters."""
    return len(''.join(re.findall('(?<=A)A+', s) + re.findall('(?<=B)B+', s)))


if __name__ == "__main__":
    print(find_deletion_number('AAABBABBABBAA'))
