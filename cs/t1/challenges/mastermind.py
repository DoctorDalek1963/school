#!/usr/bin/env python

"""A simple module to play Mastermind with the numbers 0-9.

Classes:
    Mastermind:
        A simple class to create and make guesses for a game of Mastermind.

functions:
    main() -> None:
        Play a game of Mastermind.
"""

from random import randint


class Mastermind:
    """A simple class to create and make guesses for a game of Mastermind.

    Methods:
        make_guess(string: str) -> bool:
            Make a guess and tell the user what parts are correct.

        take_input() -> bool:
            Take user input and pass it to make_guess(), then return the return value of that.
    """

    def __init__(self):
        """Create the object and generate a secret number to be guessed."""
        self._secret = str(randint(1, 9))
        for _ in range(3):
            r = randint(0, 9)

            while str(r) in self._secret:
                r = randint(0, 9)

            self._secret += str(r)

    def make_guess(self, string: str) -> bool:
        """Make a guess and tell the user what parts are correct."""
        if len(string) != 4:
            raise ValueError('string must be 4 characters long')

        if string == self._secret:
            print('Correct!')
            return True

        # We zip the strings together and see how many of the resultant tuples have two equal values
        # This gives us the number of characters in the correct position
        num_list = [tup for tup in zip(self._secret, string) if tup[0] == tup[1]]
        nums_correct_pos = len(num_list)

        new_string = string
        # We destructure the num tuple becasue we only need one. They're both the same
        for num, _ in num_list:
            new_string = new_string.replace(num, '')

        nums_wrong_pos = 0
        for c in new_string:
            if c in self._secret:
                nums_wrong_pos += 1

        print(f'You have {nums_correct_pos} numbers in the correct position')
        print(f'And {nums_wrong_pos} numbers correct but in the wrong position')
        print()

        return False

    def take_input(self) -> bool:
        """Take user input and pass it to make_guess(), then return the return value of that."""
        num = input('Please enter a four digit guess: ')

        while not (len(num) == 4 and num.isdigit()):
            num = input('Please try again: ')

        return self.make_guess(num)


def main() -> None:
    """Play a game of Mastermind."""
    print('Welcome to Mastermind!')
    print('In this game, the computer generates a random 4 digit number with no repeating digits.')
    print('You have to guess that number.')
    print()
    print('You will be told after each guess how many digits are in the correct position,')
    print('and how many digits are correct but in the wrong position.')
    print()

    mastermind = Mastermind()

    for _ in range(10):
        if mastermind.take_input():
            break


if __name__ == "__main__":
    main()
