pub struct Tree<T> {
    data: Vec<(T, Option<usize>, Vec<usize>)>,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        Tree {
            data: vec![(root, None, Vec::<usize>::new())],
        }
    }

    pub fn get_mut_data_id(&mut self, id: usize) -> &mut T {
        &mut self.data[id].0
    }

    pub fn get_data_id(&self, id: usize) -> &T {
        &self.data[id].0
    }

    pub fn add_to_id(&mut self, id: usize, data: T) {
        let child_id = self.data.len();
        self.data[id].2.push(child_id);
        self.data.push((data, Some(id), Vec::<usize>::new()));
    }

    pub fn find_child_with(&self, id: usize, f: impl Fn(&T) -> bool) -> Option<usize> {
        let child_ids = self.data[id].2.clone();
        for child_id in child_ids {
            if f(&self.data[child_id].0) {
                return Some(child_id);
            }
        }
        None
    }

    pub fn find_nodes_with(&self, f: impl Fn(&T) -> bool) -> Vec<usize> {
        let mut res = Vec::new();
        for (i, (d, _, _)) in self.data.iter().enumerate() {
            if f(d) {
                res.push(i);
            }
        }
        res
    }

    pub fn get_parent(&self, id: usize) -> Option<usize> {
        self.data[id].1
    }
}
