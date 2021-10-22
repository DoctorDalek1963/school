#!/usr/bin/env python
"""A simple file to test the library functions with nice output."""

import timeit
from library import *


def test(time: bool):
    """Test library functions."""
    g = Graph()
    a, b, c, d, e, f, z = create_vertices('A B C D E F Z')

    g.add_vertices(a, b, c, d, e, f, z)

    g.add_edge(a, b, 18, True)
    g.add_edge(a, c, 1, True)
    g.add_edge(b, d, 12)
    g.add_edge(c, a, 4, True)
    g.add_edge(c, d, 3)
    g.add_edge(c, z, 72)
    g.add_edge(d, a, 41, True)
    g.add_edge(d, e, 100)
    g.add_edge(d, f, 2, True)
    g.add_edge(d, z, 69)
    g.add_edge(f, e, 19, True)
    g.add_edge(f, z, 4)

    if time:
        rep = 1000
        avr = 10
        n = 0.0

        for i in range(avr):
            start = timeit.default_timer()

            for j in range(rep):
                _ = kruskal(g)
                # _ = dijkstra(g, a, e)

            n += timeit.default_timer() - start

        print(f'{int((n / avr)*1000)} ms')
    else:
        print(g)
        print()
        print(kruskal(g))
        print(dijkstra(g, a, e))


if __name__ == "__main__":
    test(False)
