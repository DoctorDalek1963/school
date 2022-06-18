#!/usr/bin/env python

"""A simple module for a little RPN calculator."""

import re
from inspect import signature
from math import ceil, floor, sqrt
from typing import Callable, TypeAlias


Number: TypeAlias = int | float


class OperatorError(Exception):
    """A simple operator error."""


class ParseError(Exception):
    """A simple parse error."""


class StackError(Exception):
    """A simple stack error."""


class RPNCalculator:
    """A class to hold a stack and execute commands in RPN."""

    operators: dict[str, Callable[..., list[Number]]] = {
        '+': lambda a, b: [b + a],
        '-': lambda a, b: [b - a],
        '*': lambda a, b: [b * a],
        '/': lambda a, b: [b / a],
        '//': lambda a, b: [int(b // a)],
        '**': lambda a, b: [b ** a],
        'sqrt': lambda a: [sqrt(a)],
        '<<': lambda a, b: [b << a],
        '>>': lambda a, b: [b >> a],
        'ceil': lambda a: [ceil(a)],
        'floor': lambda a: [floor(a)],
        'int': lambda a: [int(round(a, 0))],
        'round': lambda a, b: [round(b, a)],
        'inc': lambda a: [a + 1],
        'dec': lambda a: [a - 1],
        'max': lambda a, b: [max(a, b)],
        'min': lambda a, b: [min(a, b)],
        'neg': lambda a: [-a],
        'drop': lambda _: [],
        'swap': lambda a, b: [a, b],
        'dup': lambda a: [a, a],
        'over': lambda a, b: [b, a, b],
        'nip': lambda a, _: [a],
        'tuck': lambda a, b: [a, b, a],
        'rot': lambda c, b, a: [b, c, a],
        '-rot': lambda c, b, a: [c, a, b],
    }

    def __init__(self, stack: list[Number] = None):
        """Initialize an RPNCalculator with a given stack ([] if None)."""
        self.stack = stack if stack is not None else []

    def __repr__(self) -> str:
        """Return a nice repr of the calculator."""
        return f'{self.__class__.__module__}.{self.__class__.__name__}(stack={self.stack})'

    @staticmethod
    def tokenize(expression: str) -> list[str]:
        """Tokenize the given expression into executable chunks.

        This method exists to allow syntax like "2:{multiple words}" to be parsed correctly as one token.

        :raises ParseError: If there are unmatched braces in the expression
        """
        tokens: list[str] = []
        string = ''
        brace_depth = 0

        while True:
            char = expression[0]
            expression = expression[1:]

            if char.isspace() and brace_depth == 0:
                tokens.append(string)
                string = ''

            else:
                string += char

                if char == '{':
                    brace_depth += 1
                elif char == '}':
                    brace_depth -= 1

            if expression == '':
                tokens.append(string)
                break

        if brace_depth != 0:
            raise ParseError('Unmatched braces in expression')

        return tokens

    def execute(self, expression: str) -> list[Number]:
        """Execute an arbitrary expression.

        :raises OperatorError: If the operator is invalid or fails (sqrt of a negative number, for example)
        :raises StackError: If there are not enough values on the stack
        """
        tokens = RPNCalculator.tokenize(expression)

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

        :raises OperatorError: If the operator is invalid or fails (sqrt of a negative number, for example)
        :raises StackError: If there are not enough values on the stack
        """
        if operator == 'clear':
            self.stack = []
            return

        if (match := re.match(r'(\d+)\:{(.+)}', operator)) is not None:
            for _ in range(int(match.group(1))):
                self.execute(match.group(2))

            return

        if (match := re.match(r'(\d+)\:([^\s]+)', operator)) is not None:
            for _ in range(int(match.group(1))):
                self._apply_operator(match.group(2))

            return

        if operator not in RPNCalculator.operators:
            raise OperatorError(f'Operator "{operator}" not recognised')

        func = RPNCalculator.operators[operator]
        arg_count = len(signature(func).parameters)

        if len(self.stack) < arg_count:
            raise StackError(f'Not enough elements on the stack for operator "{operator}" (takes {arg_count})')

        args = []

        for _ in range(arg_count):
            args.append(self.stack.pop())

        try:
            for value in func(*args):
                self.stack.append(value)

        except ValueError as e:
            # Restore stack state
            for arg in args:
                self.stack.append(arg)

            raise OperatorError(f'Operator "{operator}" failed with operands {args}') from e


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

        except (OperatorError, ParseError, StackError) as e:
            print(e)

        except (EOFError, KeyboardInterrupt):
            print()
            return

        print(calc.stack)
        print()


if __name__ == '__main__':
    calculate()
