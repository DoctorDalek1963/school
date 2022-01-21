"""Given a staircase, find the total number of ways to reach the nth stair from the bottom
of the stair case when a person is only allowed to climb either 1 or 2 steps.

In essence: partition a given number (nth step) into ones and twos.
"""


class Partitioner:
    """This class holds state about partitioning, allowing for much more efficient recursion."""

    def __init__(self):
        """Create the Partitioner object with its cache."""
        # There is 1 way to partition 0, and 1 way to partition 1
        self.cache: list[int] = [1, 1]

    def compute(self, n: int) -> int:
        """Compute the number of ways to partition n into ones and twos."""
        # If we've already computed this, just return it
        if n < len(self.cache):
            return self.cache[n]

        # The number of ways to partition this number is the number of ways to partition n - 1,
        # plus the number of ways to partition n - 2
        number_of_ways = self.compute(n - 1) + self.compute(n - 2)
        self.cache.append(number_of_ways)
        return number_of_ways


if __name__ == '__main__':
    print(Partitioner().compute(4))
