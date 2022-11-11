#!/usr/bin/env python

"""A script to count the number of Sundays on the first of the month from 1901 to 2000."""

from datetime import datetime as dt


def is_sunday(date_string: str) -> bool:
    """Check if a given ISO date is a Sunday."""
    return dt.strftime(dt.strptime(date_string, '%Y-%m-%d'), '%A') == 'Sunday'


def main() -> None:
    """Count the number of Sundays on the first of the month from 1901 to 2000."""
    count: int = 0

    for year in range(1901, 2000 + 1):
        for month in range(1, 12 + 1):
            if is_sunday(f"{year}-{month}-01"):
                count += 1

    print(count)


if __name__ == '__main__':
    main()
