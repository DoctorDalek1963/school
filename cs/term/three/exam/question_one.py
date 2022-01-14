"""The owner of a small toy store, stores details of their stock and sales using a paper based system.
This system is becoming cumbersome as the business grows and he requires a computer based system. You
have been asked to program.

The following details need to be store for each toy:
1. Name of toy
2. Stock ID – first 2 letters of the name of the toy followed by 3 digits. Eg. TR001.
3. Price
4. Toy company
5. Number in stock (max 50)

Write a program that inputs 10 toys and store in an array. Your program must also be able to do the following:
a. Search for a toy by allowing the user to input a Stock ID as search criteria.
b. Write a procedure that outputs the “StockID” and “name of toy” for all toys that have a number in stock below 5.
c. Display the total income.
d. Allow the user input a toy company and calculate the total sales for that toy company.
"""

import re
from collections import defaultdict
from typing import TypeVar

T = TypeVar('T')


def pretty_print_list(items: list[T]) -> str:
    """Format the list nicely for pretty printing."""
    return '\n'.join('  ' + str(item) for item in items)


def pretty_currency(amount: int) -> str:
    """Return a nice-looking version of an integer amount of pennies."""
    return f'£{amount / 100:.2f}'


class Toy:
    """A simple Toy dataclass to store information."""

    def __init__(self, name: str, id_number: str, price: int, company: str):
        """Create the Toy object.

        :raises ValueError: If the name isn't at least 2 characters
        :raises ValueError: If the id_number isn't 3 digits
        """
        if len(name) < 2:
            raise ValueError('Name must be at least two characters')

        if not re.match(r'\d{3}', id_number):
            raise ValueError('ID number must be 3 digits')

        self.name = name
        self._id_number = id_number
        self.price = price
        self.company = company

    def __repr__(self) -> str:
        return f'<{self.__class__.__module__}.{self.__class__.__name__} object with name="{self.name}", ' \
               f'stock_id="{self.stock_id}", price="{pretty_currency(self.price)}", company="{self.company}">'

    def __str__(self) -> str:
        """Return a nice string of the toy."""
        return f'{self.name} toy with stock ID {self.stock_id}'

    @property
    def stock_id(self) -> str:
        """Return the stock ID of the toy."""
        return self.name[:2].upper() + self._id_number


class Shop:
    """A class to represent the whole shop."""

    def __init__(self, stock: list[Toy]):
        """Create the Shop object."""
        self.stock = stock
        self.income = 0

        # This is a dictionary from company name to total sales (default 0 if unknown company)
        self.company_sales: dict[str, int] = defaultdict(lambda: 0)

    def __repr__(self) -> str:
        return f'<{self.__class__.__module__}.{self.__class__.__name__} object with {len(self.stock)} items: '\
               f'[{", ".join(toy.stock_id for toy in self.stock)}]>'

    def search_stock(self, toy: Toy) -> list[Toy]:
        """Return all the copies of this Toy."""
        return [t for t in self.stock if t == toy]

    def add_toys(self, *toys: Toy) -> None:
        """Add a toy to stock.

        :raises ValueError: If there are already too many of this toy
        """
        for toy in toys:
            if len(self.search_stock(toy)) == 50:
                raise ValueError('Cannot have more than 50 of the same toy')

            self.stock.append(toy)

    def sell(self, toy: Toy) -> None:
        """Sell a toy."""
        if len(self.search_stock(toy)) < 1:
            raise ValueError(str(toy) + ' is out of stock')

        self.income += toy.price
        self.company_sales[toy.company] += toy.price

        self.stock.remove(toy)

    def print_company_sales(self, company: str) -> None:
        """Print the total sales for a given company."""
        print(pretty_currency(self.company_sales[company]))

    def print_total_income(self) -> None:
        """Print the total income."""
        print(pretty_currency(self.income))

    def search_by_id(self) -> None:
        """Take input and search by id, printing result."""
        stock_id = input('Please enter a stock ID: ')

        while not re.match(r'[A-Z]{2}\d{3}', stock_id.upper()):
            stock_id = input('Stock ID must be of the form "AB123". Please try again: ')

        found_toys = [toy for toy in self.stock if toy.stock_id == stock_id]

        if len(found_toys) == 0:
            print('No results found')

        else:
            print('Found:\n' + pretty_print_list(found_toys))

    def print_low_quantity(self) -> None:
        """Print the stock ID and name of all toys with less than 5 in stock."""
        print('Low on:\n' + pretty_print_list([toy for toy in self.stock if len(self.search_stock(toy)) < 5]))


if __name__ == '__main__':
    print('This program is designed to run interactively in a terminal.'
          'Please open an IPython instance and import this module.')
