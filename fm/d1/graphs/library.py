"""A library for graph and network classes and functions.

Classes:
    Vertex:
        A simple Vertex class holding only a name.

    Graph:
        A simple graph class to hold vertices and edges between them.

Exceptions:
    VertexAlreadyAddedError
    VertexDoesntExistError

Functions:
    create_vertices(names: str) -> tuple[Vertex, ...]:
        Construct multiple vertices from multiple names.

    kruskal(graph: Graph) -> Graph:
        Perform Kruskal's algorithm to find the minimum spanning tree of this graph.
"""


class VertexAlreadyAddedError(Exception):
    """A simple exception class."""


class VertexDoesntExistError(Exception):
    """A simple exception class."""


class Vertex:
    """A simple Vertex class holding only a name."""

    def __init__(self, name: str):
        """Create a Vertex object with just a name."""
        self.name = name

    def __repr__(self) -> str:
        """Return a simple repr of the vertex with its name."""
        return f'{self.__class__.__module__}.{self.__class__.__name__}(name="{self.name}")'

    def __str__(self) -> str:
        """Return the string name of the vertex."""
        return self.name


def create_vertices(names: str) -> tuple[Vertex, ...]:
    """Construct multiple vertices from multiple names."""
    return tuple(Vertex(name) for name in names.split(' '))


class Graph:
    """A simple graph class to hold vertices and edges between them.

    They may have arbitrary weight and may or may not be directed.
    This implementation does not support multiple connections of the same direction between two vertices.

    Methods:
        add_vertex(vertex: Vertex) -> None:
            Add a vertex to the graph.

        add_vertices(self, *vertices) -> None:
            Add mutiple vertices, passed as *args.

        add_edge(v: Vertex, u: Vertex, directed: bool = False, weight: int | float = 1) -> None:
            Add an edge between vertices v and u.

        remove_edge(v: Vertex, u: Vertex, directed: bool = False) -> None:
            Remove the edge between vertices v and u.

    Properties:
        is_connected: bool
            Check if the graph is fully connected.

        has_cycles: bool
            Check if the graph has cycles.

        is_tree: bool
            Check if the graph is a tree.
    """

    def __init__(self):
        """Init the graph object with no vertices or edges."""
        self.vertices: list[Vertex] = []
        self.matrix: list[list[int | float]] = [[]]

    def __repr__(self) -> str:
        """Return a simple repr of the graph with the number of vertices."""
        return f'<{self.__class__.__module__}.{self.__class__.__name__} object with ' \
            f'{len(self.vertices)} vertices [' + ", ".join(["\"" + str(v) + "\"" for v in self.vertices]) + ']>'

    def __str__(self) -> str:
        """Return the string representation of the distance matrix."""
        return '\n'.join([
            '\t'.join([
                str(cell) for cell in row
            ]) for row in self.matrix
        ])

    def add_vertex(self, vertex: Vertex) -> None:
        """Add a vertex to the graph."""
        if vertex in self.vertices:
            raise VertexAlreadyAddedError(f'Vertex "{vertex.name}" has already been added')

        self.vertices.append(vertex)

        if self.matrix == [[]]:
            self.matrix = [[0]]
        else:
            for row in self.matrix:
                row.append(0)

            self.matrix.append([0 for _ in range(len(self.matrix[0]))])

    def add_vertices(self, *vertices) -> None:
        """Add mutiple vertices, passed as *args."""
        for vertex in vertices:
            self.add_vertex(vertex)

    def _set_edge(self, v: Vertex, u: Vertex, directed: bool, weight: int | float) -> None:
        """Set the weight of the edge between vertices v and u."""
        for x in (v, u):
            if x not in self.vertices:
                raise VertexDoesntExistError(f'Vertex "{x.name}" has not been added to the graph')

        vi = self.vertices.index(v)
        ui = self.vertices.index(u)

        self.matrix[vi][ui] = weight

        if not directed:
            self.matrix[ui][vi] = weight

    def add_edge(self, v: Vertex, u: Vertex, directed: bool = False, weight: int | float = 1) -> None:
        """Add an edge between vertices v and u."""
        self._set_edge(v, u, directed, weight)

    def remove_edge(self, v: Vertex, u: Vertex, directed: bool = False) -> None:
        """Remove the edge between vertices v and u."""
        self._set_edge(v, u, directed, 0)

    def _get_connected_vertices(self, vertex: Vertex, avoid: list[Vertex]) -> list[Vertex]:
        """Return a list of vertices that are connected to the vertex, ignoring the last visited vertex."""
        # Look at all the connections in this row of the matrix, and if the weight != 0, then that vertex is connected
        return [
            self.vertices[i]
            for i, w in enumerate(self.matrix[self.vertices.index(vertex)])
            # If the weight != 0 and it's not the vertex we just came from, then it's connected
            # We're excluding the previously visited vertex to avoid infinite loops
            # We have to check for an empty list here, and if visited == [], then we just `w != 0 and True` == `w != 0`
            if w != 0 and self.vertices[i] not in avoid
        ]

    def _is_connected(self, vertex: Vertex, visited: list[Vertex]) -> bool:
        """Find if every vertex in the graph is connected."""
        # If this is the final vertex and we've connected the graph, return
        if set(visited + [vertex]) == set(self.vertices):
            return True

        connected = False

        for v in self._get_connected_vertices(vertex, visited):
            if self._is_connected(v, visited + [vertex]):
                connected = True

        return connected

    def _has_cycles(self, vertex: Vertex, visited: list[Vertex]) -> bool:
        """Recursively find cycles in the graph by a depth first search, tracking previously visited vertices in a list."""
        # If we've been to this vertex already, and it's not the one we just came from, then we've cycled
        if vertex in visited[:-1]:
            return True

        cycle_found = False

        for v in self._get_connected_vertices(vertex, [visited[-1]] if visited != [] else []):
            if self._has_cycles(v, visited + [vertex]):
                cycle_found = True

        return cycle_found

    @property
    def is_connected(self) -> bool:
        """Check if the graph is fully connected."""
        # We have to search the graph starting at each vertex to see if it's connected
        for vertex in self.vertices:
            if self._is_connected(vertex, []):
                return True

        return False

    @property
    def has_cycles(self) -> bool:
        """Check if the graph has cycles."""
        # If there are no vertices, then there are no cycles
        if len(self.vertices) == 0:
            return False

        for i in range(len(self.vertices)):
            if self.matrix[i][i] != 0:  # If we have a loop
                return True

        # Try looking for cycles starting at each vertex
        for vertex in self.vertices:
            if self._has_cycles(vertex, []):
                return True

        return False

    @property
    def is_tree(self) -> bool:
        """Check if the graph is a tree."""
        return not self.has_cycles

    @property
    def number_of_odd_nodes(self) -> int:
        """Return the number of odd nodes in the graph."""
        return sum([
            len(list(filter(lambda e: e != 0, row))) % 2 for row in self.matrix
        ])

    @property
    def is_eulerian(self) -> bool:
        """Check if the graph is Eulerian."""
        return self.number_of_odd_nodes == 0

    @property
    def is_semi_eulerian(self) -> bool:
        """Check if the graph is semi-Eulerian."""
        return self.number_of_odd_nodes == 2


def kruskal(graph: Graph) -> Graph:
    """Perform Kruskal's algorithm to find the minimum spanning tree of this graph."""
    edges: list[tuple[Vertex, Vertex, int | float]] = []

    for i, row in enumerate(graph.matrix):
        for j, weight in enumerate(row):
            if weight != 0:
                edges.append((graph.vertices[i], graph.vertices[j], weight))

    edges.sort(key=lambda t: t[2])

    directed_edges: list[tuple[Vertex, Vertex, int | float, bool]] = []

    for v, u, weight in edges:
        # If we have the same edge with the same weight but directed the other way, just make it undirected
        if (u, v, weight, True) in directed_edges:
            directed_edges = list(map(
                lambda t: (u, v, weight, False) if t == (u, v, weight, True) else t,
                directed_edges
            ))

        else:
            directed_edges.append((v, u, weight, True))

    # Construct the final graph object to return
    tree = Graph()

    for vertex in graph.vertices:
        tree.add_vertex(vertex)

    for e in directed_edges:
        tree.add_edge(e[0], e[1], weight=e[2], directed=e[3])
        if tree.has_cycles:
            tree.remove_edge(e[0], e[1], directed=e[3])

        if tree.is_connected:
            break

    return tree


def test():
    """Test library functions."""
    g = Graph()
    a, b, c, d = create_vertices('A B C D')

    g.add_vertices(a, b, c, d)

    g.add_edge(a, b)
    g.add_edge(c, d, weight=2)

    g.add_edge(b, c, weight=4)
    g.add_edge(b, d, weight=5)

    print(g, g.has_cycles)
    print()
    k = kruskal(g)
    print(k, k.has_cycles)


if __name__ == "__main__":
    test()
