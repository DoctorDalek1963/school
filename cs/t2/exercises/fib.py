"""A simple module to hold a class for generalise Fibonacci-like sequences."""


class FibSequence:
    """A simple class to generate Fibonacci-like sequences, and hold an internal cache of them."""

    def __init__(self, a: int, b: int):
        self.cache = [a, b]

    # This method allows an instance of this class to be directly called with ()
    def __call__(self, n: int) -> int:
        if n < len(self.cache):
            return self.cache[n]

        num = self(n - 1) + self(n - 2)
        self.cache.append(num)
        return num


# These are instances of the class that can be called as functions
fib = FibSequence(1, 1)
lucas = FibSequence(2, 1)
