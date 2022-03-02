#!/usr/bin/env python

"""A simple module for a little RPN calculator."""

import re


class StackError(Exception):
    """A simple stack error."""


class OperatorError(Exception):
    """A simple stack error."""


class RPNCalculator:
    """A class to hold a stack and execute commands in RPN."""

    def __init__(self, stack: list[int | float] = None):
        """Initialise an RPNCalculator with a given stack ([] if None)."""
        self.stack = stack if stack is not None else []

    def __repr__(self) -> str:
        """Return a nice repr of the calculator."""
        return f'{self.__class__.__module__}.{self.__class__.__name__}(stack={self.stack})'

    def execute(self, expression: str) -> list[int | float]:
        """Execute an arbitrary expression.

        :raises OperatorError: If the expression is invalid
        :raises StackError: If there are not enough values on the stack
        """
        tokens = re.split(r'\s+', expression)

        for token in [x for x in tokens if x]:
            try:
                num = float(token)

                if num == int(num):
                    num = int(num)

                self.stack.append(num)

            except ValueError:
                self._apply_operator(token)

        return self.stack.copy()

    def _apply_operator(self, operator: str) -> None:
        """Apply an operator to the elements on the stack.

        :raises OperatorError: If the operator is invalid
        :raises StackError: If there are not enough values on the stack
        """
        try:
            if operator == '+':
                self.stack.append(self.stack.pop() + self.stack.pop())

            elif operator == '-':
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(b - a)

            elif operator == '*':
                self.stack.append(self.stack.pop() * self.stack.pop())

            elif operator == '/':
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(b / a)

            elif operator in ('^', '**'):
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(b ** a)

            elif operator == 'drop':
                self.stack.pop()

            elif operator == 'swap':
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(a)
                self.stack.append(b)

            elif operator == 'dup':
                a = self.stack.pop()
                self.stack.append(a)
                self.stack.append(a)

            elif operator == 'over':
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(b)
                self.stack.append(a)
                self.stack.append(b)

            elif operator == 'floor':
                self.stack.append(int(self.stack.pop()))

            elif operator == 'ceil':
                self.stack.append(int(self.stack.pop() + 1))

            elif operator in ('int', 'round'):
                self.stack.append(int(round(self.stack.pop(), 0)))

            elif operator == 'clear':
                self.stack = []

            else:
                raise OperatorError(f'Unknown operator "{operator}"')

        except IndexError as e:
            raise StackError(f'Not enough elements on the stack for operator "{operator}"') from e


def calculate() -> None:
    """Give the user an RPN calculator in the terminal."""
    calc = RPNCalculator()

    while True:
        try:
            inp = input('> ')
            calc.execute(inp)

        except (OperatorError, StackError) as e:
            print(e)

        except (EOFError, KeyboardInterrupt):
            print()
            return

        print(calc.stack)
        print()


if __name__ == '__main__':
    calculate()
