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

    def test_graph_add_vertex(self) -> None:
        """Test the add_vertex() method of a Graph object."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')
        vertices = [a, b, c, d, e]

        for v in vertices:
            g.add_vertex(v)

        for v in vertices:
            self.assertIn(v, g.vertices)

    def test_graph_add_vertices(self) -> None:
        """Test the add_vertices() method of a Graph object."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')

        g.add_vertices(a, b, c, d, e)

        for v in [a, b, c, d, e]:
            self.assertIn(v, g.vertices)

    def test_vertex_already_added_error(self) -> None:
        """Test the raising of VertexAlreadyAddedError."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')

        g.add_vertices(a, b, c, d, e)

        with self.assertRaises(VertexAlreadyAddedError):
            g.add_vertex(a)
            g.add_vertex(b)
            g.add_vertex(c)
            g.add_vertex(d)
            g.add_vertex(e)
            g.add_vertices(a, b, c, d, e)

    def test_vertex_doesnt_exist_error(self) -> None:
        """Test the raising of VertexDoesntExistError when adding an edge."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')

        with self.assertRaises(VertexDoesntExistError):
            g.add_edge(a, b)
            g.add_edge(a, c)
            g.add_edge(a, d)
            g.add_edge(a, e)
            g.add_edge(b, c)
            g.add_edge(b, d)
            g.add_edge(b, e)
            g.add_edge(c, d)
            g.add_edge(c, e)
            g.add_edge(d, e)

    def test_graph_matrix(self) -> None:
        """Test the matrix given by str(Graph)."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')

        g.add_vertices(a, b, c, d, e)

        expected_matrix = '-\t3\t-\t-\t4\n2\t-\t9\t-\t12\n-\t9\t-\t4\t-\n7\t-\t-\t-\t-\n-\t12\t-\t-\t-'
        g.add_edge(a, b, 3, True)
        g.add_edge(a, e, 4, True)
        g.add_edge(b, a, 2, True)
        g.add_edge(b, c, 9, False)
        g.add_edge(b, e, 12, False)
        g.add_edge(c, d, 4, True)
        g.add_edge(d, a, 7, True)

        self.assertEqual(str(g), expected_matrix)


if __name__ == "__main__":
    unittest.main()
