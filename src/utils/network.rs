mod network {
    use pathfinding::prelude::dijkstra;
    use std::{collections::HashMap, hash::Hash};

    #[allow(dead_code)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Network<I: Clone + PartialEq + Hash + Eq, T: Clone> {
        root: Option<I>,
        nodes: HashMap<I, Node<I, T>>,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Link<I: Clone + PartialEq + Hash + Eq> {
        end: I,
        distance: usize,
    }

    #[allow(dead_code)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Node<I: Clone + PartialEq + Eq + Hash, T: Clone> {
        id: I,
        successors: Vec<Link<I>>,
        data: T,
    }

    #[allow(dead_code)]
    impl<I: Clone + PartialEq + Hash + Eq, T: Clone> Network<I, T> {
        pub fn new() -> Self {
            Network {
                root: None,
                nodes: HashMap::new(),
            }
        }

        pub fn insert_node(&mut self, id: I, data: T) {
            let node = Node {
                id: id.clone(),
                successors: Vec::new(),
                data,
            };

            self.nodes.insert(id, node);
        }

        pub fn set_root(&mut self, id: &I) {
            if self.nodes.get(id).is_some() {
                self.root = Some(id.clone());
            }
        }

        pub fn add_link(&mut self, id: I, link: Link<I>) {
            self.nodes.get_mut(&id).unwrap().successors.push(link);
        }

        pub fn get_node_ids(&self) -> Vec<I> {
            self.nodes.keys().cloned().collect()
        }

        pub fn get_node(&self, id: &I) -> Option<Node<I, T>> {
            self.nodes.get(id).cloned()
        }

        pub fn reduce(
            &mut self,
            filter: impl Fn(&Node<I, T>) -> bool,
            map: impl FnMut(Node<I, T>) -> Node<I, T>,
        ) {
            let nodes = self.nodes.values().cloned();
            self.nodes = nodes
                .map(map)
                .filter(filter)
                .map(|node| (node.id.clone(), node.clone()))
                .collect::<HashMap<I, Node<I, T>>>();
        }

        pub fn get_shortest_path_to(&self, id: &I) -> Option<Vec<I>> {
            if self.root.is_none() {
                return None;
            }
            let res: Option<(Vec<I>, usize)> = dijkstra(
                &self.root.clone().unwrap(), // root
                |n| {
                    self.nodes
                        .get(n)
                        .unwrap()
                        .successors
                        .iter()
                        .map(|link| (link.end.clone(), link.distance))
                        .collect::<Vec<(I, usize)>>()
                }, // successors
                |n| n == id,                 // reached end
            );
            if let Some((path, _)) = res {
                Some(path)
            } else {
                None
            }
        }
    }

    #[allow(dead_code)]
    impl<I: Clone + Hash + PartialEq + Eq> Link<I> {
        pub fn new(destination: I, distance: usize) -> Self {
            Link {
                end: destination,
                distance,
            }
        }
    }

    #[allow(dead_code)]
    impl<I: Clone + Hash + PartialEq + Eq, T: Clone> Node<I, T> {
        pub fn get_data(&self) -> &T {
            &self.data
        }
    }

    #[test]
    fn create_network() {
        let mut net: Network<usize, ()> = Network::new();
        let mut control_net = Network {
            root: None,
            nodes: HashMap::new(),
        };
        assert_eq!(net, control_net);

        let root = Node {
            id: 0,
            successors: Vec::new(),
            data: (),
        };
        control_net.root = Some(0);
        control_net.nodes.insert(0, root.clone());
        net.insert_node(0, ());
        net.set_root(&0);
        assert_eq!(net, control_net);

        for x in 1..=3 {
            net.insert_node(x, ());
            control_net.nodes.insert(
                x,
                Node {
                    id: x,
                    successors: Vec::new(),
                    data: (),
                },
            );
        }
        assert_eq!(net, control_net);
    }
}
