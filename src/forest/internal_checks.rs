//! Additional methods for self-consictency checking on [`RBForest`]
use super::*;

#[warn(missing_docs)]
impl<'a, K, V, const KSIZE: usize, const VSIZE: usize> RBForest<'a, K, V, KSIZE, VSIZE>
where
    K: Eq + Ord + BorshDeserialize + BorshSerialize,
    V: Eq + BorshDeserialize + BorshSerialize,
    [(); mem::size_of::<Header>()]: Sized,
{
    #[must_use]
    pub fn is_balanced(&self, tree_id: usize) -> bool {
        let mut black = 0;
        let mut node = self.root(tree_id);
        while let Some(id) = node {
            if !self.nodes[id as usize].is_red() {
                black += 1;
            }
            node = self.nodes[id as usize].left();
        }
        self.node_balanced(self.root(tree_id), black)
    }

    fn node_balanced(&self, maybe_id: Option<u32>, black: i32) -> bool {
        if let Some(id) = maybe_id {
            let id = id as usize;
            if self.nodes[id].is_red() {
                let is_left_balanced = self.node_balanced(self.nodes[id].left(), black);
                let is_right_balanced = self.node_balanced(self.nodes[id].right(), black);

                is_left_balanced && is_right_balanced
            } else {
                let is_left_balanced = self.node_balanced(self.nodes[id].left(), black - 1);
                let is_right_balanced = self.node_balanced(self.nodes[id].right(), black - 1);

                is_left_balanced && is_right_balanced
            }
        } else {
            black == 0
        }
    }

    pub fn set_node(&mut self, id: usize, node: &Node<KSIZE, VSIZE>) {
        self.nodes[id] = *node;
    }

    pub fn set_head(&mut self, head: Option<u32>) {
        {
            self.header.set_head(head);
        }
    }

    #[must_use]
    pub fn struct_eq(&self, tree_id: usize, other: &Self, other_tree_id: usize) -> bool {
        self.node_eq(self.root(tree_id), other.root(other_tree_id))
    }

    fn node_eq(&self, a: Option<u32>, b: Option<u32>) -> bool {
        match (a, b) {
            (Some(self_id), Some(other_id)) => {
                let self_id = self_id as usize;
                let other_id = other_id as usize;

                if self.nodes[self_id].is_red() ^ self.nodes[self_id].is_red() {
                    return false;
                }

                let self_key =
                    K::deserialize(&mut self.nodes[self_id].key.as_slice()).expect("Key corrupted");
                let other_key = K::deserialize(&mut self.nodes[other_id].key.as_slice())
                    .expect("Key corrupted");

                if self_key != other_key {
                    return false;
                }

                let self_value = V::deserialize(&mut self.nodes[self_id].value.as_slice())
                    .expect("Value corrupted");
                let other_value = V::deserialize(&mut self.nodes[other_id].value.as_slice())
                    .expect("Value corrupted");

                if self_value != other_value {
                    return false;
                }

                let self_left = self.nodes[self_id].left();
                let other_left = self.nodes[other_id].left();

                let self_right = self.nodes[self_id].right();
                let other_right = self.nodes[other_id].right();

                self.node_eq(self_left, other_left) && self.node_eq(self_right, other_right)
            }
            (None, None) => true,
            _ => false,
        }
    }

    //FIXME: rewrite this method so that it returns bool
    pub fn child_parent_link_test(&self, tree_id: usize) {
        if let Some(id) = self.root(tree_id) {
            assert_eq!(self.nodes[id as usize].parent(), None);
            self.node_link_test(id as usize);
        }
    }

    fn node_link_test(&self, id: usize) {
        if let Some(left_id) = self.nodes[id].left() {
            assert_eq!(self.nodes[left_id as usize].parent(), Some(id as u32));
            self.node_link_test(left_id as usize);
        }

        if let Some(right_id) = self.nodes[id].right() {
            assert_eq!(self.nodes[right_id as usize].parent(), Some(id as u32));
            self.node_link_test(right_id as usize);
        }
    }

    // One of the invariants of Red-Black tree is that red node must not have red child
    // This function checks this invariant
    #[must_use]
    pub fn no_double_red(&self, tree_id: usize) -> bool {
        if let Some(id) = self.root(tree_id) {
            self.does_not_have_red_child(id as usize)
        } else {
            true
        }
    }

    fn does_not_have_red_child(&self, node_id: usize) -> bool {
        match (self.nodes[node_id].left(), self.nodes[node_id].right()) {
            (None, None) => true,
            (Some(id), None) => {
                let id = id as usize;
                let self_redness = self.nodes[node_id].is_red();
                let child_redness = self.nodes[id].is_red();
                if self_redness & child_redness {
                    false
                } else {
                    self.does_not_have_red_child(id)
                }
            }
            (None, Some(id)) => {
                let id = id as usize;
                let self_redness = self.nodes[node_id].is_red();
                let child_redness = self.nodes[id].is_red();
                if self_redness & child_redness {
                    false
                } else {
                    self.does_not_have_red_child(id)
                }
            }
            (Some(left_id), Some(right_id)) => {
                let left_id = left_id as usize;
                let right_id = right_id as usize;
                let self_redness = self.nodes[node_id].is_red();
                let left_child_redness = self.nodes[left_id].is_red();
                let right_child_redness = self.nodes[right_id].is_red();
                // If either of the children is red AND self is red, return FALSE
                if self_redness & (left_child_redness | right_child_redness) {
                    false
                } else {
                    self.does_not_have_red_child(left_id) & self.does_not_have_red_child(right_id)
                }
            }
        }
    }
}
