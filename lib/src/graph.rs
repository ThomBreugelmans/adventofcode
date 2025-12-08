use core::hash::Hash;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct KruskalNode {
    parent: usize,
    rank: u64,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Edge {
    pub src: usize,
    pub dst: usize,
    pub weight: i64,
}

impl Edge {
    pub fn new(src: usize, dst: usize, weight: i64) -> Self {
        Self { src, dst, weight }
    }
}

pub struct Graph<T: Eq + PartialEq + Hash> {
    vertices: HashMap<T, usize>,
    #[allow(dead_code)]
    edges: Vec<Edge>,
    _kruskal_nodes: Vec<KruskalNode>,
    _mst_roots: HashMap<usize, Vec<usize>>,
    disjoint_graphs: usize,
}

impl<T> Graph<T>
where
    T: Eq + PartialEq + Hash + Clone,
{
    pub fn new(vertices: Vec<T>) -> Self {
        let mut _vertices = HashMap::with_capacity(vertices.len());
        let mut _kruskal_nodes = Vec::with_capacity(vertices.len());
        let mut _mst_roots = HashMap::with_capacity(vertices.len());
        for (i, v) in vertices.iter().enumerate() {
            _vertices.insert(v.clone(), i);
            _kruskal_nodes.push(KruskalNode { parent: i, rank: 1 });
            _mst_roots.insert(i, Vec::new());
        }
        Graph {
            vertices: _vertices,
            edges: Vec::new(),
            _kruskal_nodes: _kruskal_nodes,
            _mst_roots: _mst_roots,
            disjoint_graphs: vertices.len(),
        }
    }

    pub fn add_vertice(&mut self, v: T) {
        if self.vertices.contains_key(&v) {
            return;
        }
        let id = self.vertices.len();
        self.vertices.insert(v, id);
        self._kruskal_nodes.push(KruskalNode {
            parent: id,
            rank: 1,
        });
        self._mst_roots.insert(id, Vec::new());
        self.disjoint_graphs += 1;
    }

    pub fn get_disjoint_graph_count(&self) -> usize {
        self.disjoint_graphs
    }

    fn _add_edge(&mut self, edge: Edge) {
        let max_id = edge.src.max(edge.dst);
        assert!(
            max_id < self.vertices.len(),
            "Source or destination of added edge does not exist in vertices"
        );

        let src_root = self.find_root(edge.src);
        let dst_root = self.find_root(edge.dst);
        if src_root != dst_root {
            // we join the smallest under the largest
            if self._kruskal_nodes[src_root].rank > self._kruskal_nodes[dst_root].rank {
                self._kruskal_nodes[dst_root].parent = src_root;
                // need to remove the root as well
                self._mst_roots.remove(&dst_root);
            } else if self._kruskal_nodes[src_root].rank < self._kruskal_nodes[dst_root].rank {
                self._kruskal_nodes[src_root].parent = dst_root;
                // need to remove the root as well
                self._mst_roots.remove(&src_root);
            } else {
                self._kruskal_nodes[dst_root].parent = src_root;
                self._kruskal_nodes[src_root].rank += 1;
                // need to remove the root as well
                self._mst_roots.remove(&dst_root);
            }
            self.disjoint_graphs -= 1;
        }
    }

    pub fn add_edge(&mut self, from: T, to: T, weight: i64) {
        let src = if let Some(id) = self.vertices.get(&from) {
            *id
        } else {
            self.disjoint_graphs += 1;
            let id = self.vertices.len();
            self.vertices.insert(from, id);
            id
        };
        let dst = if let Some(id) = self.vertices.get(&to) {
            *id
        } else {
            self.disjoint_graphs += 1;
            let id = self.vertices.len();
            self.vertices.insert(to, id);
            id
        };

        let edge = Edge::new(src, dst, weight);
        self._add_edge(edge);
    }

    /// Find the root of a node, and updates the internal structure of the node, where parent points to the root
    fn find_root(&mut self, vertice: usize) -> usize {
        if self._kruskal_nodes[vertice].parent == vertice {
            // this is the root
            vertice
        } else {
            let root = self.find_root(self._kruskal_nodes[vertice].parent);
            self._kruskal_nodes[vertice].parent = root;
            root
        }
    }

    /// Gets the minimal spanning tree of the graph
    pub fn kruskal(&mut self) -> Vec<Vec<usize>> {
        for vi in 0..self._kruskal_nodes.len() {
            let root = self.find_root(vi);
            self._mst_roots.get_mut(&root).unwrap().push(vi);
        }
        self._mst_roots.values().map(|v| v.clone()).collect()
    }
}

#[test]
fn test_kruskal() {
    let vertices = vec![0, 1, 2, 3];
    let edges = vec![
        Edge {
            src: 2,
            dst: 3,
            weight: 4,
        },
        Edge {
            src: 0,
            dst: 3,
            weight: 5,
        },
        Edge {
            src: 0,
            dst: 1,
            weight: 10,
        },
    ];
    let mut graph = Graph::new(vertices);
    for e in edges {
        graph.add_edge(e.src, e.dst, e.weight);
    }
    assert_eq!(vec![vec![0, 1, 2, 3]], graph.kruskal());
}

#[test]
fn test_kruskal_aoc() {
    let vertices = (0usize..=19usize).collect();
    let edges = vec![
        Edge::new(0, 19, 100427),
        Edge::new(0, 7, 103401),
        Edge::new(2, 13, 103922),
        Edge::new(7, 19, 107662),
        Edge::new(17, 18, 111326),
        Edge::new(9, 12, 114474),
        Edge::new(11, 16, 118604),
        Edge::new(2, 8, 120825),
        Edge::new(14, 19, 123051),
        Edge::new(2, 18, 124564),
    ];
    let mut graph = Graph::new(vertices);
    for e in edges {
        graph.add_edge(e.src, e.dst, e.weight);
    }
    let k = graph.kruskal();
    let mut k_sizes = k.iter().map(|c| c.len()).collect::<Vec<usize>>();
    k_sizes.sort();
    assert_eq!(vec![1, 1, 1, 1, 1, 1, 1, 2, 2, 4, 5], k_sizes)
}
