#!/usr/bin/env python
"""Write a program that inputs 5 numbers into an array and returns true if the array contains,
somewhere, three increasing adjacent numbers like .... 4, 5, 6, ... or 23, 24, 25.

tripleUp({1, 4, 5, 6, 2}) → true
tripleUp({1, 2, 3, 7, 23}) → true
tripleUp({1, 2, 4, 123, 45}) → false
"""


def is_consecutive(values: list[int]) -> bool:
    """Check if the list contains only consecutive numbers."""
    try:
        return values == list(range(values[0], values[-1] + 1))
    except TypeError:
        return False


def check_three_consecutive_ints(values: list[int]) -> bool:
    """Check if the list includes three consecutive integers."""
    return any(is_consecutive(sublist) for i in range(len(values)) if len(sublist := values[i:i + 3]) == 3)


def main() -> None:
    """Allow lists to be input."""
    values: list[int] = []
    inp = input('Please enter a number: ')

    while True:
        if inp == '':
            print(f'This list does{"" if check_three_consecutive_ints(values) else " NOT"} '
                  'contain three consecutive values')
            return
        else:
            values.append(int(inp))
            inp = input('Please enter another number: ')


if __name__ == '__main__':
    main()
