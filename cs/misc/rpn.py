#!/usr/bin/env python

"""A simple module for a little RPN calculator."""

import re
from math import ceil, floor, sqrt
from typing import Callable, TypeAlias


Number: TypeAlias = int | float


class StackError(Exception):
    """A simple stack error."""


class OperatorError(Exception):
    """A simple stack error."""


class RPNCalculator:
    """A class to hold a stack and execute commands in RPN."""

    operators: dict[str, tuple[int, Callable[..., list[Number]]]] = {
        '+': (2, lambda a, b: [b + a]),
        '-': (2, lambda a, b: [b - a]),
        '*': (2, lambda a, b: [b * a]),
        '/': (2, lambda a, b: [b / a]),
        '//': (2, lambda a, b: [int(b // a)]),
        '**': (2, lambda a, b: [b ** a]),
        'sqrt': (1, lambda a: [sqrt(a)]),
        '<<': (2, lambda a, b: [b << a]),
        '>>': (2, lambda a, b: [b >> a]),
        'ceil': (1, lambda a: [ceil(a)]),
        'floor': (1, lambda a: [floor(a)]),
        'int': (1, lambda a: [int(round(a, 0))]),
        'round': (2, lambda a, b: [round(b, a)]),
        'inc': (1, lambda a: [a + 1]),
        'dec': (1, lambda a: [a - 1]),
        'max': (2, lambda a, b: [max(a, b)]),
        'min': (2, lambda a, b: [min(a, b)]),
        'neg': (1, lambda a: [-a]),
        'drop': (1, lambda _: []),
        'swap': (2, lambda a, b: [a, b]),
        'dup': (1, lambda a: [a, a]),
        'over': (2, lambda a, b: [b, a, b]),
        'nip': (2, lambda a, _: [a]),
    }

    def __init__(self, stack: list[Number] = None):
        """Initialise an RPNCalculator with a given stack ([] if None)."""
        self.stack = stack if stack is not None else []

    def __repr__(self) -> str:
        """Return a nice repr of the calculator."""
        return f'{self.__class__.__module__}.{self.__class__.__name__}(stack={self.stack})'

    def execute(self, expression: str) -> list[Number]:
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
        if operator == 'clear':
            self.stack = []
            return

        if operator not in RPNCalculator.operators:
            raise OperatorError(f'Operator "{operator}" not recognised')

        arg_count, func = RPNCalculator.operators[operator]

        if len(self.stack) < arg_count:
            raise StackError(f'Not enough elements on the stack for operator "{operator}" (takes {arg_count})')

        args = []

        for _ in range(arg_count):
            args.append(self.stack.pop())

        for value in func(*args):
            self.stack.append(value)


def calculate() -> None:
    """Give the user an RPN calculator in the terminal."""
    calc = RPNCalculator()

    while True:
        try:
            inp = input('> ').lower()

            if inp in ('help', '?'):
                print('Available operators:')
                print(' '.join(RPNCalculator.operators.keys()), 'clear')
                print()
                continue

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
