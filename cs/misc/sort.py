#!/usr/bin/env python
"""A module containing a Sorter class, which can sort lists using different algorithms."""

import random
import sys
import time
from typing import Callable


def timed(f: Callable) -> Callable:
    """Time the passed function. Used as a wrapper."""

    def dummy(*args, **kwargs):
        start = time.time()
        result = f(*args, **kwargs)
        end = time.time()

        print(f'{f.__name__} took {1000 * (end - start):.4f} ms')
        return result

    return dummy


class Sorter:
    """A class that takes a list and can sort it with different algorithms."""

    def __init__(self, original_list: list[int]) -> None:
        """Initialise the Sorter with the given list."""
        self.__original_list = original_list

    @property
    def original_list(self) -> list[int]:
        """Return a copy of the original list."""
        return self.__original_list.copy()

    @timed
    def bubble_sort(self) -> list[int]:
        """Sort the list with bubble sort."""
        new_list = self.original_list

        iterations = len(new_list) - 1
        for _ in range(len(new_list) - 1):
            for i in range(iterations):
                if new_list[i] > new_list[i + 1]:
                    new_list[i], new_list[i + 1] = new_list[i + 1], new_list[i]

        return new_list

    @timed
    def recursive_quick_sort(self) -> list[int]:
        """Sort the list with recursive quick sort."""
        return Sorter._static_recursive_quick_sort(self.original_list)

    @staticmethod
    def _static_recursive_quick_sort(static_list: list[int]) -> list[int]:
        """Sort a given list with recursive quick sort."""
        if len(static_list) == 0:
            return []

        pivot = static_list[0]
        new_list = static_list[1:]

        sorted_first = Sorter._static_recursive_quick_sort([x for x in new_list if x < pivot])
        sorted_last = Sorter._static_recursive_quick_sort([x for x in new_list if x >= pivot])

        return sorted_first + [pivot] + sorted_last

    @timed
    def stalin_sort(self) -> list[int]:
        """Remove all elements that aren't in order."""
        new_list = self.original_list
        i = 0
        highest = 0

        while True:
            try:
                if new_list[i] < highest:
                    del new_list[i]

                else:
                    highest = new_list[i]
                    i += 1

            except IndexError:
                break

        return new_list

    @timed
    def bogo_sort(self) -> list[int]:
        """Randomise list. Is it sorted? If not, randomise again. Repeat."""
        new_list = self.original_list

        while True:
            random.shuffle(new_list)
            if check_sorted(new_list):
                return new_list

    @timed
    def merge_sort(self) -> list[int]:
        """Perform a merge sort on the list."""
        return Sorter._static_merge_sort(self.original_list)

    @staticmethod
    def _static_merge_sort(items: list[int]) -> list[int]:
        """Perform a recursive static merge sort on the given list."""
        if len(items) < 2:
            return items

        new_list: list[int] = []
        mid = len(items) // 2

        left = Sorter._static_merge_sort(items[:mid])
        right = Sorter._static_merge_sort(items[mid:])

        li = 0
        ri = 0

        # Add as much as we can to the new_list
        while li < len(left) and ri < len(right):
            if left[li] < right[ri]:
                new_list.append(left[li])
                li += 1
            else:
                new_list.append(right[ri])
                ri += 1

        # One of theses slices will be empty, but the other will contain unmerged, sorted elements that we just append
        new_list += left[li:]
        new_list += right[ri:]

        return new_list

    @timed
    def insertion_sort(self) -> list[int]:
        """Perform an insertion sort on the list."""
        new_list = self.original_list

        for j in range(1, len(new_list)):
            next_item = new_list[j]
            i = j - 1

            while i >= 0 and new_list[i] > next_item:
                new_list[i + 1] = new_list[i]
                i -= 1

            new_list[i + 1] = next_item

        return new_list


def check_sorted(list_to_check: list[int]) -> bool:
    """Loop over the given list and if it's not sorted, return False, else return True."""
    for i in range(len(list_to_check) - 1):
        if list_to_check[i] > list_to_check[i + 1]:
            return False

    # If nothing broke and returned False
    return True


def main() -> None:
    """Do the main method."""
    # If the script was given a number as an argument, use it, otherwise, use 1000
    if len(sys.argv) > 1 and sys.argv[1].isdigit():
        x = int(sys.argv[1])
    else:
        x = 1000

    list_ = list(range(x))

    random.shuffle(list_)
    sorter = Sorter(list_)

    print()

    sorter.bubble_sort()
    sorter.stalin_sort()
    sorter.recursive_quick_sort()
    # sorter.bogo_sort()
    sorter.merge_sort()
    sorter.insertion_sort()


if __name__ == '__main__':
    main()
