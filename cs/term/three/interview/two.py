"""Given an list of integers, find the largest sublist formed by consecutive integers."""


def find_longest_sublist_length(nums: list[int]) -> int:
    """Find the length of the longest sublist of a given list where the sublist contains only consecutive integers."""
    s = set(nums)
    answer = 0

    # Here, we're using a set and looping to find all the consecutive elements in the set,
    # and then we're just finding the longest run of consecutive integers
    for i, num in enumerate(nums):
        # We only want to loop if we're at the start of a consecutive run
        if num - 1 in s:
            continue

        # Keep looping until the next number is no longer in the set
        while num in s:
            num += 1

        answer = max(answer, num - nums[i])

    return answer


def consecutive(nums: list[int]) -> bool:
    """Test if a list contains only consecutive integers."""
    return sorted(nums) == list(range(min(nums), max(nums) + 1))


def find_consecutive_sublist(nums: list[int], length: int = None) -> list[int]:
    """Given a length, find the first sublist of that length where all the elements are consecutive."""
    if length is None:
        length = find_longest_sublist_length(nums)

    # This is a simple sliding window implementation
    for i in range(len(nums) - length):
        if consecutive(nums[i:i + length]):
            return nums[i:i + length]

    else:  # If we didn't break from the loop and return
        raise ValueError('Length did not give consecutive sublist')


if __name__ == '__main__':
    print(find_consecutive_sublist([2, 0, 2, 1, 4, 3, 1, 0]))
