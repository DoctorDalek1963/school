#!/usr/bin/env python

"""A simple module for a little RPN calculator."""

from __future__ import annotations

import math
import os
import pathlib
import re
import readline
from inspect import signature
from math import ceil, floor, sqrt, sin, cos, tan, asin, acos, atan, log
from typing import Callable, TypeAlias


Number: TypeAlias = int | float


def file_in_local_dir(filename: str) -> str:
    """Return the full path of a file in the same directory as this script."""
    return os.path.join(
        pathlib.Path(__file__).parent.absolute(),
        filename
    )


def is_number(string: str) -> bool:
    """Check if the given string is a number."""
    try:
        float(string)
        return True
    except ValueError:
        return False


class MacroError(Exception):
    """A simple macro error."""


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
        'abs': lambda a: [abs(a)],
        'sin': lambda a: [sin(a)],
        'cos': lambda a: [cos(a)],
        'tan': lambda a: [tan(a)],
        'asin': lambda a: [asin(a)],
        'acos': lambda a: [acos(a)],
        'atan': lambda a: [atan(a)],
        'ln': lambda a: [log(a)],
        'log': lambda a, b: [log(a, b)],
        'pi': lambda: [math.pi],
        'deg2rad': lambda a: [math.radians(a)],
        'rad2deg': lambda a: [math.degrees(a)],
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
        'rrot': lambda c, b, a: [c, a, b],
    }

    def __init__(self, stack: list[Number] = None, *, illegal_chars: str = None):
        """Initialize an RPNCalculator with a given stack ([] if None) and string of illegal characters."""
        self.stack = stack if stack else []
        self.macros: dict[str, str] = {}

        illegal_chars = illegal_chars if illegal_chars else readline.get_completer_delims()
        self.illegal_char_pattern = re.compile('.*[' + re.escape(illegal_chars) + '].*')

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

    def get_help(self, command: str) -> str:
        """Return the help text for the given operator, or the definition of the given macro.

        :raises OperatorError: If the command is invalid
        """
        op_help = {
            '+': 'Add the top two elements',
            '-': 'Subtract the top two elements',
            '*': 'Multiply the top two elements',
            '/': 'Divide the top two elements',
            '//': 'Divide the top two elements and cast the result to an int',
            '**': 'Exponentiate the top two elements',
            'sqrt': 'Take the square root of the top element',
            '<<': 'Bitshift the second element left by the top element',
            '>>': 'Bitshift the second element right by the top element',
            'ceil': 'Round the top element up',
            'floor': 'Round the top element down',
            'int': 'Round the top element to the nearest int',
            'round': 'Round the second element to the top element number of decimal places',
            'abs': 'Take the absolute value of the top element',
            'sin': 'Take the sine of the top element',
            'cos': 'Take the cosine of the top element',
            'tan': 'Take the tangent of the top element',
            'asin': 'Take the arcsin of the top element',
            'acos': 'Take the arccos of the top element',
            'atan': 'Take the arctan of the top element',
            'ln': 'Take the natural logarithm of the top element',
            'log': 'Take the log of the top element using the element underneath it as the base',
            'pi': 'Add pi to the top of the stack',
            'deg2rad': 'Convert the top element from degrees to radians',
            'rad2deg': 'Convert the top element from radians to degrees',
            'inc': 'Increment the top element',
            'dec': 'Decrement the top element',
            'max': 'Take the maximum of the top two elements',
            'min': 'Take the minimum of the top two elements',
            'neg': 'Negate the top element',
            'drop': 'Drop the top element',
            'swap': 'Swap the top two elements',
            'dup': 'Duplicate the top element',
            'over': 'Duplicate the second element and add it to the top',
            'nip': 'Drop the second element',
            'tuck': 'Duplicate the top element and tuck it behind the second element',
            'rot': 'Rotate the top three elements',
            'rrot': 'Rotate the top three elements in the opposite direction'
        }

        if command in op_help:
            return op_help[command]

        if command in self.macros:
            return self.macros[command]

        raise OperatorError(f'Operator "{command}" not recognised')

    def fully_expand_macros(self, name: str) -> str:
        """Return a recursive expansion of the given macro."""
        if name not in self.macros:
            if name in self.operators:
                raise MacroError(f'"{name}" is an operator, not a macro')

            raise MacroError(f'Undefined macro "{name}"')

        try:
            return self._fully_expand_macros(name)
        except RecursionError:
            raise MacroError('Recursion error in expansion (probably a circular definition)')

    def _fully_expand_macros(self, expression: str) -> str:
        """Return a recursive expansion of the given macro.

        This is the internal equivalent of :meth:`fully_expand_macros`.
        This one will return an operator name when a given an operator name,
        which would be very confusing for the user.
        """
        if expression in self.operators or is_number(expression):
            return expression

        if expression in self.macros:
            return self._fully_expand_macros(self.macros[expression])

        commands = re.sub(r'((\d+|rep):|[{}])', '', expression).split()
        for command in commands:
            expression = expression.replace(command, self._fully_expand_macros(command))

        return expression

    def load_macros(self) -> list[str]:
        """Load macros from macros.rpn in same directory and return loaded macro names."""
        macro_names = []
        filename = file_in_local_dir('macros.rpn')

        if os.path.isfile(filename):
            with open(filename, 'r', encoding='utf-8') as f:
                for line in f.read().splitlines():
                    if match := re.match(r'^(\S+)!\{(.+)}$', line):
                        name = match.group(1)
                        if re.match(self.illegal_char_pattern, name):
                            raise MacroError(f'Illegal character in macro name "{name}" in macros.rpn')

                        self.macros[name] = match.group(2)
                        macro_names.append(match.group(1))

        return macro_names

    def execute(self, expression: str) -> None:
        """Execute an arbitrary expression.

        :raises OperatorError: If the operator is invalid or fails (sqrt of a negative number, for example)
        :raises StackError: If there are not enough values on the stack
        """
        if expression == '':
            return

        tokens = RPNCalculator.tokenize(expression)

        for token in [x for x in tokens if x]:
            try:
                num = float(token)

                if num == int(num):
                    num = int(num)

                self.stack.append(num)

            except ValueError:
                self._apply_operator(token)

    def _apply_operator(self, operator: str) -> None:
        """Apply an operator to the elements on the stack.

        :raises OperatorError: If the operator is invalid or fails (sqrt of a negative number, for example)
        :raises StackError: If there are not enough values on the stack
        """
        if operator == 'clear':
            self.stack = []
            return

        if match := re.match(r'(\d+):{(.+)}', operator):
            for _ in range(int(match.group(1))):
                self.execute(match.group(2))

            return

        if match := re.match(r'(\d+):(\S+)', operator):
            for _ in range(int(match.group(1))):
                self._apply_operator(match.group(2))

            return

        if match := re.match(r'rep:{(.+)}', operator):
            rep = self.stack[-1]

            if rep == int(rep):
                self.stack.pop()
            else:
                raise OperatorError('`rep:{commands}` requires an integer on the top of the stack')

            for _ in range(rep):
                self.execute(match.group(1))

            return

        if match := re.match(r'rep:(\S+)', operator):
            rep = self.stack[-1]

            if rep == int(rep):
                self.stack.pop()
            else:
                raise OperatorError('`rep:command` requires an integer on the top of the stack')

            for _ in range(rep):
                self._apply_operator(match.group(1))

            return

        if match := re.match(r'^(\S+)!\{(.+)}$', operator):
            name = match.group(1)
            if re.match(self.illegal_char_pattern, name):
                raise MacroError(f'Illegal character in macro name "{name}"')

            self.macros[match.group(1)] = match.group(2)
            return

        if match := re.match(r'^!(\S+)$', operator):
            if match.group(1) in self.macros:
                self.macros.pop(match.group(1))
                return

            raise MacroError(f'Macro "{match.group(1)}" not defined')

        if operator in self.macros:
            self.execute(self.macros[operator])
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

        except (ValueError, ZeroDivisionError) as e:
            # Restore stack state
            for arg in args:
                self.stack.append(arg)

            raise OperatorError(f'Operator "{operator}" failed with operands {args}') from e

    def repl_complete(self, text: str, state: int) -> str | None:
        """Complete the text prompt as given in the REPL.

        This function is meant to be registered as the completer for ``readline.set_completer()``.
        """
        candidates = list(self.operators.keys()) + list(self.macros.keys())

        if match := re.match(r'.*?(\S+)$', text):
            token = match.group(1)

            # Filter candidates based on each character of the token
            for i, c in enumerate(token):
                candidates = [
                    x for x in candidates
                    if x[i] == c
                ]

        if state < len(candidates):
            # Add the space at the end to stop readline trying to complete the same token again
            return candidates[state] + ' '

        return None


def main() -> None:
    """Give the user an RPN calculator in the terminal."""
    inputrc = os.path.join(os.path.expanduser('~'), '.inputrc')
    history_file = file_in_local_dir('.rpn_history')

    if os.path.isfile(inputrc):
        readline.read_init_file(inputrc)

    readline.parse_and_bind('tab: complete')

    if not os.path.isfile(history_file):
        open(history_file, 'w', encoding='utf-8').close()

    readline.read_history_file(history_file)
    readline.set_history_length(1000)

    calc = RPNCalculator()
    loaded_macros = calc.load_macros()

    readline.set_completer(calc.repl_complete)

    if loaded_macros:
        print('Loaded macros:')
        print(' ', *loaded_macros)
        print()

    while True:
        try:
            inp = input('> ').lower()

            if inp in ('help', '?'):
                print('Operators:')
                print(' ', *RPNCalculator.operators.keys(), 'clear')

                if len(calc.macros) > 0:
                    print()
                    print('Macros:')
                    print(' ', *calc.macros.keys())

                print()
                print('Repeat a command N times with `N:command`')
                print('Repeat a sequence of commands N times with `N:{sequence of commands}`')
                print('Replace N with `rep` to use the number on top of the stack as N')
                print('Define a macro with `macro_name!{macro commands}`')
                print('Remove a macro with `!macro_name`')
                print()
                print('See help for an operator or the definition of a macro with `command?`')
                print('See the full definition of a macro (recursively expanded) with `macro??`')
                print()

                continue

            if match := re.match(r'([^\s?]+)\?\?$', inp):
                print(calc.fully_expand_macros(match.group(1)))
                print()

                continue

            if match := re.match(r'([^\s?]+)\?$', inp):
                print(calc.get_help(match.group(1)))
                print()

                continue

            calc.execute(inp)

        except (MacroError, OperatorError, ParseError, StackError) as e:
            try:
                import rich
                rich.print(f'[bold red]{e.__class__.__name__}[/bold red]: {e}')

            except ModuleNotFoundError:
                print(f'{e.__class__.__name__}: {e}')

        except KeyboardInterrupt:
            print('\r')
            continue

        except EOFError:
            print()
            readline.write_history_file(file_in_local_dir('.rpn_history'))
            return

        print(calc.stack)
        print()


if __name__ == '__main__':
    main()
