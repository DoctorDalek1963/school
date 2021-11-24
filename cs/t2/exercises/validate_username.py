#!/usr/bin/env python
"""Validate usernames according to certain rules."""

import re


def validate_username(username: str) -> bool:
    """Validate a username according to certain rules.

    A name must: be between 4 and 25 characters, start with a letter, only
    contain letters, numbers, and underscores, and it cannot end with an underscore.
    """
    return bool(re.match(r'^[a-z][a-z0-9_]{2,23}[a-z0-9]$', username, flags=re.IGNORECASE))


def main() -> None:
    """Take user input and validate the name."""
    if validate_username(input('Please input a username: ')):
        print('Valid')
    else:
        print('Invalid')


if __name__ == "__main__":
    main()
