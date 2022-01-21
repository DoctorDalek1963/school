"""Given an unsorted list of integers, find a pair with a given sum in the list."""


def find_pairs(nums: list[int], target: int) -> list[tuple[int, int]]:
    """Find all pairs in the list of nums that sum to the target."""
    pairs: list[tuple[int, int]] = []

    # We iterate over every number in the original list and check it against the numbers that come after it in the list
    for start_index, start_num in enumerate(nums):
        # Here, we iterate over a generator, which yields the index in the original nums list
        # for every element in the shortened list where the values sum to the target
        for index in (
                i + start_index + 1
                for i, num in enumerate(nums[start_index + 1:])
                if num + start_num == target
        ):
            pairs.append((start_index, index))

    return pairs


if __name__ == '__main__':
    print(find_pairs([1, 3, 5, 7, 2, 5, 9, 8], 10))
