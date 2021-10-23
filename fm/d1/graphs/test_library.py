#!/usr/bin/env python
"""A simple unittest for testing the graph library classes and functions."""

import unittest
from library import *


class TestGraphAndVertex(unittest.TestCase):
    """A class to hold methods for testing the Graph and Vertex classes."""

    def test_vertex_init(self) -> None:
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

    def test_create_vertices(self) -> None:
        """Test the creation of Vertex objects with the create_vertices() function."""
        a, vertex, newline_test, capitals, tab_test = create_vertices('a vertex newline\ntest CaPiTaLs tab\ttest')
        self.assertEqual(a.name, 'a')
        self.assertEqual(vertex.name, 'vertex')
        self.assertEqual(newline_test.name, 'newline\ntest')
        self.assertEqual(capitals.name, 'CaPiTaLs')
        self.assertEqual(tab_test.name, 'tab\ttest')

        a, b, c, d = create_vertices('A    B C      D')
        self.assertEqual(a.name, 'A')
        self.assertEqual(b.name, 'B')
        self.assertEqual(c.name, 'C')
        self.assertEqual(d.name, 'D')

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

        g.add_vertex(a)

        with self.assertRaises(VertexDoesntExistError):
            g.add_edge(a, b)
            g.add_edge(a, c)
            g.add_edge(a, d)
            g.add_edge(a, e)

        g.add_vertex(b)
        # Shouldn't raise an error
        g.add_edge(a, b)

    def test_graph_matrix(self) -> None:
        """Test the matrix given by str(Graph)."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')

        g.add_vertices(a, b, c, d, e)

        expected_matrix = '-\t3\t-\t-\t4\n2\t-\t9\t-\t12\n-\t9\t-\t4\t-\n7\t-\t-\t-\t-\n-\t12\t-\t-\t6'
        g.add_edge(a, b, 3, True)
        g.add_edge(a, e, 4, True)
        g.add_edge(b, a, 2, True)
        g.add_edge(b, c, 9, False)
        g.add_edge(b, e, 12, False)
        g.add_edge(c, d, 4, True)
        g.add_edge(d, a, 7, True)
        g.add_edge(e, e, 6)

        self.assertEqual(str(g), expected_matrix)

    def test_is_connected(self) -> None:
        """Test the is_connected bool property of Graph."""
        g = Graph()
        a, b, c, d = create_vertices('A B C D')
        g.add_vertices(a, b, c, d)

        self.assertFalse(g.is_connected)
        g.add_edge(a, b)
        self.assertFalse(g.is_connected)
        g.add_edge(c, d)
        self.assertFalse(g.is_connected)

        g.add_edge(b, c)
        self.assertTrue(g.is_connected)

    def test_has_cycles_and_is_tree(self) -> None:
        """Test the has_cycles bool property of Graph."""
        g = Graph()
        a, b, c, d = create_vertices('A B C D')
        g.add_vertices(a, b, c, d)

        self.assertFalse(g.has_cycles)
        self.assertTrue(g.is_tree)

        g.add_edge(a, b)
        self.assertFalse(g.has_cycles)
        self.assertTrue(g.is_tree)

        g.add_edge(c, d)
        self.assertFalse(g.has_cycles)
        self.assertTrue(g.is_tree)

        g.add_edge(b, c)
        self.assertFalse(g.has_cycles)
        self.assertTrue(g.is_tree)

        g.add_edge(a, d)
        self.assertTrue(g.has_cycles)
        self.assertFalse(g.is_tree)

    def test_is_eulerian(self) -> None:
        """Test the is_eulerian bool property of Graph."""
        g = Graph()
        a, b, c, d = create_vertices('A B C D')
        g.add_vertices(a, b, c, d)

        g.add_edge(a, b)
        g.add_edge(b, c)
        g.add_edge(c, d)

        self.assertFalse(g.is_eulerian)
        g.add_edge(a, d)
        self.assertTrue(g.is_eulerian)
        g.add_edge(b, d)
        self.assertFalse(g.is_eulerian)
        g.add_edge(a, c)
        self.assertFalse(g.is_eulerian)

    def test_is_semi_eulerian(self) -> None:
        """Test the is_semi_eulerian bool property of Graph."""
        g = Graph()
        a, b, c, d = create_vertices('A B C D')
        g.add_vertices(a, b, c, d)

        g.add_edge(a, b)
        g.add_edge(b, c)
        g.add_edge(c, d)

        self.assertTrue(g.is_semi_eulerian)
        g.add_edge(a, d)
        self.assertFalse(g.is_semi_eulerian)
        g.add_edge(b, d)
        self.assertTrue(g.is_semi_eulerian)
        g.add_edge(a, c)
        self.assertFalse(g.is_semi_eulerian)

    def test_number_of_odd_nodes(self) -> None:
        """Test the number_of_odd_nodes bool property of Graph."""
        g = Graph()
        a, b, c, d = create_vertices('A B C D')
        g.add_vertices(a, b, c, d)

        g.add_edge(a, b)
        g.add_edge(b, c)
        g.add_edge(c, d)

        self.assertEqual(g.number_of_odd_nodes, 2)
        g.add_edge(a, d)
        self.assertEqual(g.number_of_odd_nodes, 0)
        g.add_edge(b, d)
        self.assertEqual(g.number_of_odd_nodes, 2)
        g.add_edge(a, c)
        self.assertEqual(g.number_of_odd_nodes, 4)

    def test_graph_getitem(self) -> None:
        """Test the Graph.__getitem__() method."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')
        g.add_vertices(a, b, c, d, e)

        g.add_edge(a, b, 12)
        self.assertEqual(g[a], [0, 12, 0, 0, 0])
        g.add_edge(a, c, 4, True)
        self.assertEqual(g[a], [0, 12, 4, 0, 0])
        g.add_edge(a, d, 39)
        self.assertEqual(g[a], [0, 12, 4, 39, 0])
        g.add_edge(a, e, 100, True)
        self.assertEqual(g[a], [0, 12, 4, 39, 100])

        g.add_edge(a, a, 3)
        self.assertEqual(g[a], [3, 12, 4, 39, 100])

        self.assertEqual(g[b], [12, 0, 0, 0, 0])
        self.assertEqual(g[c], [0, 0, 0, 0, 0])
        self.assertEqual(g[d], [39, 0, 0, 0, 0])
        self.assertEqual(g[e], [0, 0, 0, 0, 0])

    def test_get_connected_vertices(self) -> None:
        """Test the Graph.get_connected_vertices() method."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')
        g.add_vertices(a, b, c, d, e)

        self.assertEqual(g.get_connected_vertices(a, []), [])

        g.add_edge(a, b, 12)
        self.assertEqual(g.get_connected_vertices(a, []), [b])
        g.add_edge(a, c, 4, True)
        self.assertEqual(g.get_connected_vertices(a, []), [b, c])
        g.add_edge(a, d, 39)
        self.assertEqual(g.get_connected_vertices(a, []), [b, c, d])
        g.add_edge(a, e, 100, True)
        self.assertEqual(g.get_connected_vertices(a, []), [b, c, d, e])

        g.add_edge(a, a, 3)
        self.assertEqual(g.get_connected_vertices(a, []), [a, b, c, d, e])

        self.assertEqual(g.get_connected_vertices(b, []), [a])
        self.assertEqual(g.get_connected_vertices(c, []), [])
        self.assertEqual(g.get_connected_vertices(d, []), [a])
        self.assertEqual(g.get_connected_vertices(e, []), [])

    def test_total_weight(self) -> None:
        """Test the Graph.total_weight property."""
        g = Graph()
        a, b, c, d, e = create_vertices('A B C D E')
        g.add_vertices(a, b, c, d, e)

        self.assertEqual(g.total_weight, 0)

        g.add_edge(a, b, 3)
        self.assertEqual(g.total_weight, 6)
        g.add_edge(a, c, 12)
        self.assertEqual(g.total_weight, 30)
        g.add_edge(d, e, 4, True)
        self.assertEqual(g.total_weight, 34)
        g.add_edge(b, e, 2.34)
        self.assertEqual(round(g.total_weight, 2), 38.68)
        g.add_edge(c, e, 902.1)
        self.assertEqual(round(g.total_weight, 2), 1842.88)


class TestAlgorithmsOnGraphs(unittest.TestCase):
    """A class to hold methods for testing algorithms on graphs, like Kruskal and Dijkstra."""

    def setUp(self):
        """Create some graphs to test with."""
        self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i, \
            self.j, self.k, self.l, self.m, self.n, self.o, self.p, self.q, self.r, \
            self.s, self.t, self.u, self.v, self.w, self.x, self.y, self.z \
            = create_vertices('A B C D E F G H I J K L M N O P Q R S T U V W X Y Z')

        self.all_vertices = [
            self.a, self.b, self.c, self.d, self.e, self.f, self.g, self.h, self.i,
            self.j, self.k, self.l, self.m, self.n, self.o, self.p, self.q, self.r,
            self.s, self.t, self.u, self.v, self.w, self.x, self.y, self.z
            ]

        self.small_graph = Graph()
        self.small_graph.add_vertices(self.a, self.b, self.c, self.d, self.e)

        self.small_graph.add_edge(self.a, self.b, 3)
        self.small_graph.add_edge(self.a, self.c, 19)
        self.small_graph.add_edge(self.a, self.e, 12)
        self.small_graph.add_edge(self.b, self.c, 5)
        self.small_graph.add_edge(self.b, self.d, 9)
        self.small_graph.add_edge(self.c, self.d, 2)
        self.small_graph.add_edge(self.c, self.e, 11)

        self.medium_graph = Graph()
        self.medium_graph.add_vertices(self.a, self.b, self.c, self.d, self.e, self.f,
                                       self.g, self.h, self.i, self.j, self.k, self.l)

        self.medium_graph.add_edge(self.a, self.b, 3.9)
        self.medium_graph.add_edge(self.a, self.l, 2, True)
        self.medium_graph.add_edge(self.b, self.d, 7)
        self.medium_graph.add_edge(self.b, self.g, 1.2)
        self.medium_graph.add_edge(self.b, self.i, 12)
        self.medium_graph.add_edge(self.b, self.k, 11)
        self.medium_graph.add_edge(self.c, self.e, 9)
        self.medium_graph.add_edge(self.c, self.g, 8, True)
        self.medium_graph.add_edge(self.c, self.h, 14, True)
        self.medium_graph.add_edge(self.d, self.e, 3)
        self.medium_graph.add_edge(self.d, self.f, 3.1)
        self.medium_graph.add_edge(self.e, self.h, 20)
        self.medium_graph.add_edge(self.f, self.g, 6.3)
        self.medium_graph.add_edge(self.g, self.c, 7.4, True)
        self.medium_graph.add_edge(self.g, self.g, 3)
        self.medium_graph.add_edge(self.h, self.i, 4)
        self.medium_graph.add_edge(self.h, self.j, 5)
        self.medium_graph.add_edge(self.h, self.k, 9)
        self.medium_graph.add_edge(self.i, self.j, 8)
        self.medium_graph.add_edge(self.j, self.k, 12.7)
        self.medium_graph.add_edge(self.k, self.l, 11, True)
        self.medium_graph.add_edge(self.l, self.k, 13, True)
        self.medium_graph.add_edge(self.l, self.a, 3, True)

        self.large_graph = Graph()
        self.large_graph.add_vertices(*self.all_vertices)

        # This large graph is based on Euclidean distance, with the vertices
        # arranged in 6 rows of 4, with 2 in the middle at the bottom
        self.large_graph.add_edge(self.a, self.e)
        self.large_graph.add_edge(self.a, self.g, 2.24)
        self.large_graph.add_edge(self.b, self.c)
        self.large_graph.add_edge(self.b, self.e, 1.41)
        self.large_graph.add_edge(self.b, self.f)
        self.large_graph.add_edge(self.c, self.d)
        self.large_graph.add_edge(self.c, self.f)
        self.large_graph.add_edge(self.c, self.h, 1.41)
        self.large_graph.add_edge(self.c, self.k, 2)
        self.large_graph.add_edge(self.d, self.g, 1.41)
        self.large_graph.add_edge(self.e, self.j, 1.41)
        self.large_graph.add_edge(self.f, self.i, 1.41)
        self.large_graph.add_edge(self.f, self.j)
        self.large_graph.add_edge(self.f, self.k, 1.41)
        self.large_graph.add_edge(self.g, self.h)
        self.large_graph.add_edge(self.h, self.j, 2.24)
        self.large_graph.add_edge(self.h, self.k, 1.41)
        self.large_graph.add_edge(self.i, self.j)
        self.large_graph.add_edge(self.i, self.m)
        self.large_graph.add_edge(self.i, self.o, 2.24)
        self.large_graph.add_edge(self.j, self.k)
        self.large_graph.add_edge(self.j, self.m, 1.41)
        self.large_graph.add_edge(self.j, self.n)
        self.large_graph.add_edge(self.j, self.q, 2.24)
        self.large_graph.add_edge(self.j, self.t, 2.82)
        self.large_graph.add_edge(self.k, self.l)
        self.large_graph.add_edge(self.k, self.r, 2.24)
        self.large_graph.add_edge(self.k, self.s, 2)
        self.large_graph.add_edge(self.l, self.o, 1.41)
        self.large_graph.add_edge(self.l, self.s, 2.24)
        self.large_graph.add_edge(self.n, self.o)
        self.large_graph.add_edge(self.n, self.q, 1.41)
        self.large_graph.add_edge(self.o, self.p)
        self.large_graph.add_edge(self.o, self.s)
        self.large_graph.add_edge(self.p, self.t)
        self.large_graph.add_edge(self.q, self.r)
        self.large_graph.add_edge(self.q, self.y, 2.24)
        self.large_graph.add_edge(self.r, self.s)
        self.large_graph.add_edge(self.r, self.u, 1.41)
        self.large_graph.add_edge(self.r, self.x, 2.24)
        self.large_graph.add_edge(self.s, self.w)
        self.large_graph.add_edge(self.s, self.x, 1.41)
        self.large_graph.add_edge(self.s, self.y, 2.24)
        self.large_graph.add_edge(self.t, self.w, 1.41)
        self.large_graph.add_edge(self.t, self.x)
        self.large_graph.add_edge(self.u, self.v)
        self.large_graph.add_edge(self.v, self.w)
        self.large_graph.add_edge(self.v, self.y)
        self.large_graph.add_edge(self.v, self.z, 1.41)
        self.large_graph.add_edge(self.w, self.z)
        self.large_graph.add_edge(self.x, self.z, 1.41)

    def test_kruskal(self) -> None:
        """Test the implementation of Kruskal's algorithm to find the minimum spanning tree."""
        expected_small = Graph()
        expected_small.add_vertices(*self.small_graph.vertices)

        expected_small.add_edge(self.a, self.b, 3)
        expected_small.add_edge(self.b, self.c, 5)
        expected_small.add_edge(self.c, self.d, 2)
        expected_small.add_edge(self.c, self.e, 11)

        self.assertEqual(kruskal(self.small_graph), expected_small)

        expected_medium = Graph()
        expected_medium.add_vertices(*self.medium_graph.vertices)

        expected_medium.add_edge(self.a, self.b, 3.9)
        expected_medium.add_edge(self.a, self.l, 2, True)
        expected_medium.add_edge(self.b, self.g, 1.2)
        expected_medium.add_edge(self.b, self.k, 11)
        expected_medium.add_edge(self.d, self.f, 3.1)
        expected_medium.add_edge(self.d, self.e, 3)
        expected_medium.add_edge(self.f, self.g, 6.3)
        expected_medium.add_edge(self.g, self.c, 7.4, True)
        expected_medium.add_edge(self.h, self.i, 4)
        expected_medium.add_edge(self.h, self.j, 5)
        expected_medium.add_edge(self.h, self.k, 9)
        expected_medium.add_edge(self.k, self.l, 11, True)

        self.assertEqual(kruskal(self.medium_graph), expected_medium)

        expected_large = Graph()
        expected_large.add_vertices(*self.large_graph.vertices)

        expected_large.add_edge(self.a, self.e)
        expected_large.add_edge(self.b, self.c)
        expected_large.add_edge(self.b, self.f)
        expected_large.add_edge(self.b, self.e, 1.41)
        expected_large.add_edge(self.c, self.d)
        expected_large.add_edge(self.c, self.h, 1.41)
        expected_large.add_edge(self.h, self.g)
        expected_large.add_edge(self.f, self.j)
        expected_large.add_edge(self.j, self.i)
        expected_large.add_edge(self.i, self.m)
        expected_large.add_edge(self.j, self.k)
        expected_large.add_edge(self.j, self.n)
        expected_large.add_edge(self.k, self.l)
        expected_large.add_edge(self.n, self.o)
        expected_large.add_edge(self.o, self.p)
        expected_large.add_edge(self.p, self.t)
        expected_large.add_edge(self.t, self.x)
        expected_large.add_edge(self.o, self.s)
        expected_large.add_edge(self.s, self.r)
        expected_large.add_edge(self.r, self.q)
        expected_large.add_edge(self.s, self.w)
        expected_large.add_edge(self.w, self.v)
        expected_large.add_edge(self.v, self.u)
        expected_large.add_edge(self.v, self.y)
        expected_large.add_edge(self.w, self.z)

        self.assertEqual(kruskal(self.large_graph), expected_large)

    def test_dijkstra(self) -> None:
        """Test the implementation of Dijkstra's shortest path algorithm."""
        expected_small = [self.a, self.b, self.c, self.d]
        self.assertEqual(dijkstra(self.small_graph, self.a, self.d), expected_small)

        expected_medium = [self.a, self.b, self.k]
        self.assertEqual(dijkstra(self.medium_graph, self.a, self.k), expected_medium)

        expected_large = [self.a, self.e, self.j, self.k, self.s, self.w, self.z]
        self.assertEqual(dijkstra(self.large_graph, self.a, self.z), expected_large)


if __name__ == "__main__":
    unittest.main()
