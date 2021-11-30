#!/usr/bin/env python
"""A script to determine if there exists exactly 3 question marks between every pair of numbers that adds to 10."""

import re


def validate_question_marks(string: str) -> bool:
    """Validate the input string to find if there exist exactly 3 question marks between every pair of numbers that adds to 10."""
    # We're using a capturing group in a lookahead to cheat and allow for overlapping matches
    matches = re.findall(r'(?=(\d+\?*\d+))', re.sub('[^?0-9]', '', string))

    for match in matches:
        sum_of_numbers = sum(int(x) for x in re.split(r'\?+', match))
        question_marks = len(re.sub(r'[^\?]', '', match))

        if not (sum_of_numbers == 10 and question_marks == 3):
            return False

    return True


def main() -> None:
    """Take user input and validate it."""
    if validate_question_marks(input('Please input a string: ')):
        print('Valid')
    else:
        print('Invalid')


if __name__ == "__main__":
    main()
