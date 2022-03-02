"""A library for graph and network classes and functions.

Classes:
    Vertex:
        A simple Vertex class holding only a name.

    Graph:
        A simple graph class to hold vertices and edges between them.

    DijkstraVertex:
        A dataclass to hold the information Dijkstra needs about vertices.

Exceptions:
    VertexAlreadyAddedError
    VertexDoesntExistError

Functions:
    create_vertices(names: str) -> tuple[Vertex, ...]:
        Construct multiple vertices from multiple names.

    kruskal(graph: Graph) -> Graph:
        Perform Kruskal's algorithm to find the minimum spanning tree of this graph.

    dijkstra(graph: Graph, start: Vertex, end: Vertex) -> list[Vertex]:
        Implement Dijkstra's algorithm on the graph, starting at the start vertex and ending at the end vertex.
"""

import math


class VertexAlreadyAddedError(Exception):
    """A simple exception class."""


class VertexDoesntExistError(Exception):
    """A simple exception class."""


class EdgeDoesntExistError(Exception):
    """A simple exception class."""


class Vertex:
    """A Vertex class, holding only a name."""
    __slots__ = ['name']

    def __init__(self, name: str):
        """Create a Vertex object with just a name."""
        self.name = name

    def __repr__(self) -> str:
        """Return a simple repr of the vertex with its name."""
        return f'{self.__class__.__module__}.{self.__class__.__name__}(name="{self.name}")'

    def __str__(self) -> str:
        """Return the string name of the vertex."""
        return f'Vertex "{self.name}"'

    def __eq__(self, other) -> bool:
        """Check equality of vertices by name rather than id()."""
        if not isinstance(other, self.__class__):
            return NotImplemented
        return self.name == other.name

    def __hash__(self) -> int:
        """Hash the name of the vertex."""
        return hash(self.name)


def create_vertices(names: str) -> tuple[Vertex, ...]:
    """Construct multiple vertices from multiple names."""
    return tuple(Vertex(name) for name in names.split(' ') if name != '')


class Graph:
    """A simple graph class to hold vertices and edges between them.

    They may have arbitrary weight and may or may not be directed.
    This implementation does not support multiple connections of the same direction between two vertices.
    Calling str() on the graph will return a printable distance matrix.

    Methods:
        add_vertex(vertex: Vertex) -> None:
            Add a vertex to the graph.

        add_vertices(self, *vertices) -> None:
            Add multiple vertices, passed as *args.

        add_edge(v: Vertex, u: Vertex, weight: int | float = 1, directed: bool = False) -> None:
            Add an edge between vertices v and u.

        remove_edge(v: Vertex, u: Vertex, directed: bool = False) -> None:
            Remove the edge between vertices v and u.

        get_connected_vertices(vertex: Vertex, avoid: list[Vertex]) -> list[Vertex]:
            Return a list of vertices that are connected to the vertex, ignoring the vertices in the avoid list.

        weight_of_path(path: list[Vertex]) -> int | float:
            Return the weight of the given path. Raises VertexDoesntExistError if any vertex isn't in the graph.

    Properties:
        vertices: list[Vertex]
            The vertices in the graph, in the order they were added.

        matrix: list[list[int | float]]
            The distance matrix of the graph.

        is_connected: bool
            Check if the graph is fully connected.

        has_cycles: bool
            Check if the graph has cycles.

        is_tree: bool
            Check if the graph is a tree.

        number_of_odd_nodes: int
            Return the number of odd nodes in the graph.

        is_eulerian: bool
            Check if the graph is Eulerian.

        is_semi_eulerian: bool
            Check if the graph is semi-Eulerian.

        total_weight: int | float
            Return the total weight of the graph.
    """

    def __init__(self):
        """Init the graph object with no vertices or edges."""
        self.vertices: list[Vertex] = []
        self.matrix: list[list[int | float]] = [[]]

    def __repr__(self) -> str:
        """Return a simple repr of the graph with the number of vertices."""
        return f'<{self.__class__.__module__}.{self.__class__.__name__} object with ' \
            f'{len(self.vertices)} vertices [' + ', '.join(["'" + str(v) + "'" for v in self.vertices]) + ']>'

    def __str__(self) -> str:
        """Return the string representation of the distance matrix."""
        return '\n'.join([
            '\t'.join([
                # If the cell weight is 0, we print it with a dash rather than a 0
                str(cell) if cell != 0 else '-' for cell in row
            ]) for row in self.matrix
        ])

    def __getitem__(self, vertex: Vertex):
        """Return the neighbours of the vertex."""
        if not isinstance(vertex, Vertex):
            raise ValueError(f'Can only get Vertex objects from {self.__class__.__name__}')
        return self.matrix[self.vertices.index(vertex)]

    def __eq__(self, other) -> bool:
        """Test for equality between Graph objects by testing for the same vertices and string matrices."""
        if not isinstance(other, self.__class__):
            return NotImplemented
        return self.vertices == other.vertices and str(self) == str(other)

    def __hash__(self) -> int:
        """Hash the graph by vertex list and matrix."""
        return hash((self.vertices, str(self)))

    def add_vertex(self, vertex: Vertex) -> None:
        """Add a vertex to the graph."""
        if vertex in self.vertices:
            raise VertexAlreadyAddedError(str(vertex) + ' has already been added')

        self.vertices.append(vertex)

        if self.matrix == [[]]:
            self.matrix = [[0]]
        else:
            for row in self.matrix:
                row.append(0)

            self.matrix.append([0 for _ in self.matrix[0]])

    def add_vertices(self, *vertices) -> None:
        """Add multiple vertices, passed as *args."""
        for vertex in vertices:
            self.add_vertex(vertex)

    def _set_edge(self, v: Vertex, u: Vertex, weight: int | float, directed: bool) -> None:
        """Set the weight of the edge between vertices v and u."""
        for x in (v, u):
            if x not in self.vertices:
                raise VertexDoesntExistError(str(x) + ' has not been added to the graph')

        vi = self.vertices.index(v)
        ui = self.vertices.index(u)

        self.matrix[vi][ui] = weight

        if not directed:
            self.matrix[ui][vi] = weight

    def add_edge(self, v: Vertex, u: Vertex, weight: int | float = 1, directed: bool = False) -> None:
        """Add an edge between vertices v and u."""
        self._set_edge(v, u, weight, directed)

    def remove_edge(self, v: Vertex, u: Vertex, directed: bool = False) -> None:
        """Remove the edge between vertices v and u."""
        self._set_edge(v, u, 0, directed)

    def get_connected_vertices(self, vertex: Vertex, avoid: list[Vertex]) -> list[Vertex]:
        """Return a list of vertices that are connected to the vertex, ignoring the vertices in the avoid list.

        A vertex in the avoid list will be avoided unless it's connected by a different weight in this direction.
        This means that if two vertices are connected by different weights in their different directions,
        then we can walk between them.
        """
        # Look at all the connections in this row of the matrix, and if the weight != 0, then that vertex is connected
        vi = self.vertices.index(vertex)

        return [
            self.vertices[i]
            for i, w in enumerate(self[vertex])
            # For a vertex to be connected, it must have a non-zero weight

            # If a vertex is in the avoid list, then we want to avoid it,
            # UNLESS it's connected by an edge of a different weight in this direction

            # This means that if two vertices are connected by directed edges in different
            # directions in different weights, then we are allowed to traverse both edges
            if w != 0 and (self.vertices[i] not in avoid or self.matrix[i][vi] != self.matrix[vi][i])
        ]

    def weight_of_path(self, path: list[Vertex]) -> int | float:
        """Return the weight of the given path. Raises VertexDoesntExistError if any vertex isn't in the graph."""
        for vertex in path:
            if vertex not in self.vertices:
                raise VertexDoesntExistError(str(vertex) + ' is not in the graph.')

        weight: int | float = 0
        # We slice to avoid the end of the list. This lets us use i + 1
        for i, vertex in enumerate(path[:-1]):
            # index(path[i + 1]) gives us the index of the next vertex, so we can get the weight of that edge
            edge = self[vertex][self.vertices.index(path[i + 1])]

            # If the weight is 0, there is no edge here, so this path is impossible
            if edge == 0:
                raise EdgeDoesntExistError(f"This path doesn't exist. There is no edge between {vertex} and {path[i + 1]}")

            weight += edge

        return weight

    def _is_connected(self, vertex: Vertex, visited: list[Vertex]) -> bool:
        """Find if every vertex in the graph is connected."""
        # If this is the final vertex and we've connected the graph, return
        if set(visited + [vertex]) == set(self.vertices):
            return True

        connected = False

        for v in self.get_connected_vertices(vertex, visited):
            if self._is_connected(v, visited + [vertex]):
                connected = True

        return connected

    def _has_cycles(self, vertex: Vertex, visited: list[Vertex]) -> bool:
        """Recursively find cycles in the graph by a depth first search, tracking previously visited vertices in a list."""
        # If we've been to this vertex already, and it's not the one we just came from, then we've cycled
        if vertex in visited[:-1]:
            return True

        for v in self.get_connected_vertices(vertex, [visited[-1]] if visited != [] else []):
            if self._has_cycles(v, visited + [vertex]):
                return True

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
        return sum(len([e for e in row if e != 0]) % 2 for row in self.matrix)

    @property
    def is_eulerian(self) -> bool:
        """Check if the graph is Eulerian."""
        return self.number_of_odd_nodes == 0

    @property
    def is_semi_eulerian(self) -> bool:
        """Check if the graph is semi-Eulerian."""
        return self.number_of_odd_nodes == 2

    @property
    def total_weight(self) -> int | float:
        """Return the total weight of the graph."""
        return sum(weight for row in self.matrix for weight in row)


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

    tree.add_vertices(*graph.vertices)

    for e in directed_edges:
        tree.add_edge(e[0], e[1], weight=e[2], directed=e[3])
        if tree.has_cycles:
            tree.remove_edge(e[0], e[1], directed=e[3])

        if tree.is_connected:
            break

    return tree


class DijkstraVertex:
    """A class to hold the information Dijkstra needs about vertices."""
    __slots__ = ['vertex', 'order', 'working_distance', 'final_distance']

    def __init__(self, vertex: Vertex, order: int = 0,
                 working_distance: int | float = math.inf,
                 final_distance: int | float | None = None):
        """Create a DijkstraVertex object with a vertex, order, working_distance, and final_distance."""
        self.vertex = vertex
        self.order = order
        self.working_distance = working_distance
        self.final_distance = final_distance

    def __repr__(self) -> str:
        """Return a simple repr of the DijkstraVertex."""
        return f'{self.__class__.__module__}.{self.__class__.__name__}(vertex={repr(self.vertex)}, ' + \
            f'order={self.order}, working_distance={self.working_distance}, final_distance={self.final_distance})'

    @property
    def visited(self) -> bool:
        """Check if the DijkstraVertex has been visited."""
        return self.final_distance is not None


def dijkstra(graph: Graph, start: Vertex, end: Vertex) -> list[Vertex]:
    """Implement Dijkstra's algorithm on the graph, starting at the start vertex and ending at the end vertex.

    Returns a list of Vertex objects, representing the path taken through the graph, as well as the total weight.
    """
    # We create a list of DijkstraVertex (abbreviated dv) objects, and init the
    # start vertex with an order, working distance, and final distance
    # We then have to exclude the start vertex from the rest of the list
    dvs: list[DijkstraVertex] = [DijkstraVertex(start, 1, 0, 0)] + \
        [DijkstraVertex(vertex) for vertex in graph.vertices if vertex is not start]

    # This while loop adds the information that Dijkstra needs to every vertex: the order and final_distance
    while len([dv for dv in dvs if not dv.visited]) > 0:
        # The current dv is the one with the highest order
        current_dv: DijkstraVertex = max(dvs, key=lambda dv: dv.order if dv.order is not None else 0)

        # These checks are just to make mypy shut up
        if current_dv.final_distance is None:
            raise ValueError('DijkstraVertex with non-None order must have non-None final_distance')

        if current_dv.order is None:
            raise ValueError('current_dv in dijkstra() has a None order value. Something has gone horribly wrong')

        connected_vertices: list[Vertex] = graph.get_connected_vertices(
            current_dv.vertex,
            # We're avoiding the vertices that we've already visited
            [dv.vertex for dv in dvs if dv.visited]
        )
        connected_dvs: list[DijkstraVertex] = []

        # Get the dv versions of the connected vertices
        for vertex in connected_vertices:
            # There's a probably a better way to get the dv from the normal vertices this but this method works
            dv = [dv for dv in dvs if dv.vertex is vertex][0]
            if not dv.visited:
                connected_dvs.append(dv)

        # For each of the connected dvs, we update the weight if its less than the current working_distance
        for connected_dv in connected_dvs:
            weight = current_dv.final_distance + \
                graph[current_dv.vertex][graph.vertices.index(connected_dv.vertex)]

            if weight < connected_dv.working_distance:
                connected_dv.working_distance = weight

        # The new dv is the one with the minimum working_distance
        new_dv = min(
            # We only want vertices that aren't the current one, and that we've not visited
            [dv for dv in dvs if dv is not current_dv and not dv.visited],
            key=lambda dv: dv.working_distance
        )
        # Mark the new_dv as visited, by giving it a final_distance and order
        new_dv.final_distance = new_dv.working_distance
        new_dv.order = current_dv.order + 1

    # We start looking at the end vertex and work backwards
    working_dv: DijkstraVertex = [dv for dv in dvs if dv.vertex is end][0]
    path: list[Vertex] = [working_dv.vertex]

    # This loop traverses the graph backwards, to find the shortest path using the information previously generated
    while working_dv.vertex is not start:
        # This check is just to satisfy mypy
        if working_dv.final_distance is None:
            raise ValueError('working_dv should always have a non-None final_distance. Something has gone horribly wrong')

        # Find the dvs that can be added to the path (not currently in the path)
        open_dvs = [dv for dv in dvs if dv.vertex not in path]

        # We need to find every vertex that is connected to this working_dv.vertex
        # Because the graph could be directed, we have to do it like this, rather than
        # just finding the connections from working_dv.vertex
        back_connected: list[Vertex] = []
        for dv in open_dvs:
            if working_dv.vertex in graph.get_connected_vertices(dv.vertex, []):
                back_connected.append(dv.vertex)

        # We only want to loop over the dvs that aren't already in the path
        for dv in open_dvs:
            if dv.vertex in back_connected:
                weight = graph[dv.vertex][graph.vertices.index(working_dv.vertex)]

                # This check is just to satisfy mypy
                if dv.final_distance is None:
                    raise ValueError('dv should always have a non-None final_distance. Something has gone horribly wrong')

                # We need to round here to get rid of floating point errors
                if round(working_dv.final_distance - weight, 5) == round(dv.final_distance, 5):
                    path.append(dv.vertex)
                    working_dv = dv
                    break

    # We need to reverse this list before we return it, because we worked backwards
    return path[::-1]
