#!/usr/bin/env python

"""A tiny module containing only find_deletion_number(s: str), a function to find how many
A and B characters in s must be deleted to remove all sequences of adjacent characters."""

import re


def find_deletion_number(s: str) -> int:
    """Find how many A and B characters must be deleted to remove all sequences of adjacent characters."""
    num_a = len(''.join(re.findall('(?<=A)A+', s)))
    num_b = len(''.join(re.findall('(?<=B)B+', s)))

    return num_a + num_b


if __name__ == "__main__":
    print(find_deletion_number('AAABBABBABBAA'))
