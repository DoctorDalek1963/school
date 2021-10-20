#!/usr/bin/env python
"""A simple unittest for testing the graph library classes and functions."""

import unittest
from library import *


class TestLibrary(unittest.TestCase):
    """A class to hold methods for testing the graph classes and f."""

    def test_vertex_creation(self) -> None:
        """Test the creation of Vertex objects with the Vertex constructor."""
        testing_io: dict[str, Vertex] = {
            'A': Vertex('A'),
            'vertex': Vertex('vertex'),
            'space test': Vertex('space test'),
            'unicode ∵ ∴': Vertex('unicode ∵ ∴'),
            '': Vertex(''),
            'qwertyuiopasdfghjklzxcvbnm': Vertex('qwertyuiopasdfghjklzxcvbnm'),
            'newline\ntest': Vertex('newline\ntest'),
            'tab\ttest': Vertex('tab\ttest'),
            'CaPiTaLs': Vertex('CaPiTaLs')
        }
        for name, vertex in testing_io.items():
            self.assertEqual(name, vertex.name)

    def test_vertices_creation(self) -> None:
        """Test the creation of Vertex objects with the create_vertices() function."""
        a, vertex, newline_test, capitals, tab_test = create_vertices('a vertex newline\ntest CaPiTaLs tab\ttest')
        self.assertEqual(a.name, 'a')
        self.assertEqual(vertex.name, 'vertex')
        self.assertEqual(newline_test.name, 'newline\ntest')
        self.assertEqual(capitals.name, 'CaPiTaLs')
        self.assertEqual(tab_test.name, 'tab\ttest')


if __name__ == "__main__":
    unittest.main()
