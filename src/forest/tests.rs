use super::*;
use core::fmt::Debug;
use pretty_assertions::assert_eq;

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

    pub unsafe fn set_node(&mut self, id: usize, node: &Node<KSIZE, VSIZE>) {
        self.nodes[id] = *node;
    }

    pub unsafe fn set_head(&mut self, head: Option<u32>) {
        unsafe {
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
}

pub const INSERT_KEYS: [u8; 256] = [
    123, 201, 112, 93, 21, 236, 41, 121, 42, 10, 147, 254, 220, 148, 76, 245, 94, 142, 75, 222,
    132, 215, 86, 150, 31, 137, 60, 120, 14, 36, 77, 35, 192, 224, 204, 97, 129, 80, 252, 99, 79,
    202, 196, 172, 221, 165, 185, 102, 157, 2, 138, 233, 164, 206, 12, 190, 105, 151, 33, 188, 56,
    174, 71, 247, 128, 73, 65, 229, 5, 255, 109, 38, 200, 171, 49, 217, 232, 7, 43, 92, 24, 183,
    67, 19, 149, 159, 238, 44, 198, 248, 69, 162, 34, 244, 203, 26, 101, 100, 143, 241, 187, 210,
    126, 131, 87, 50, 59, 179, 32, 197, 55, 70, 113, 115, 82, 125, 64, 37, 230, 251, 184, 211, 47,
    110, 133, 83, 72, 116, 68, 124, 156, 195, 89, 216, 178, 182, 45, 191, 114, 1, 228, 250, 30, 61,
    189, 231, 27, 57, 235, 181, 11, 29, 239, 194, 40, 84, 160, 209, 106, 4, 205, 249, 74, 111, 9,
    8, 81, 240, 173, 16, 154, 48, 46, 90, 54, 17, 166, 25, 225, 66, 155, 103, 168, 53, 212, 214,
    161, 13, 186, 122, 52, 152, 15, 199, 28, 20, 104, 58, 253, 208, 176, 0, 237, 96, 163, 246, 226,
    146, 223, 175, 22, 39, 88, 95, 207, 234, 130, 63, 219, 23, 243, 180, 3, 193, 119, 144, 98, 51,
    218, 139, 18, 85, 170, 117, 107, 6, 158, 177, 145, 141, 78, 169, 118, 242, 136, 134, 91, 140,
    62, 127, 167, 135, 108, 213, 227, 153,
];

#[test]
fn init() {
    let mut vec = create_vec(4, 4, 5, 1);

    let mut tree = RBForest::<i32, u32, 4, 4>::init_slice(vec.as_mut_slice(), 1).unwrap();
    assert!(tree.is_empty(0));
    assert_eq!(tree.free_nodes_left(), 5);

    assert_eq!(tree.insert(0, 12, 32), Ok(None));
    assert_eq!(tree.get(0, &12), Some(32));
    assert_eq!(tree.len(0), 1);
    assert_eq!(tree.free_nodes_left(), 4);

    assert_eq!(tree.insert(0, 32, 44), Ok(None));
    assert_eq!(tree.get(0, &32), Some(44));
    assert_eq!(tree.len(0), 2);
    assert_eq!(tree.free_nodes_left(), 3);

    assert_eq!(tree.insert(0, 123, 321), Ok(None));
    assert_eq!(tree.get(0, &123), Some(321));
    assert_eq!(tree.len(0), 3);
    assert_eq!(tree.free_nodes_left(), 2);

    assert_eq!(tree.insert(0, 123, 322), Ok(Some(321)));
    assert_eq!(tree.get(0, &123), Some(322));
    assert_eq!(tree.len(0), 3);
    assert_eq!(tree.free_nodes_left(), 2);

    assert_eq!(tree.insert(0, 14, 32), Ok(None));
    assert_eq!(tree.get(0, &14), Some(32));
    assert_eq!(tree.len(0), 4);
    assert_eq!(tree.free_nodes_left(), 1);

    assert_eq!(tree.insert(0, 1, 2), Ok(None));
    assert_eq!(tree.free_nodes_left(), 0);
    assert_eq!(tree.insert(0, 1, 4), Ok(Some(2)));
    assert_eq!(tree.free_nodes_left(), 0);
    assert_eq!(tree.insert(0, 3, 4), Err(Error::NoNodesLeft));

    assert_eq!(tree.get(0, &15), None);

    assert_eq!(tree.len(0), 5);

    tree.clear();
    assert_eq!(tree.len(0), 0);
    assert_eq!(tree.free_nodes_left(), 5);
}

#[test]
fn swap_nodes() {
    let mut vec = create_vec(4, 4, 6, 1);

    let mut tree = RBForest::<i32, u32, 4, 4>::init_slice(vec.as_mut_slice(), 1).unwrap();
    // Initial structure
    //          parent
    //           /
    // black-> swap1
    //        /   \
    //red-> swap2 node1 <-red
    //      /
    //  node2            <-black
    unsafe {
        let parent = Node::from_raw_parts(
            // 0
            u32::to_be_bytes(1),
            u32::to_be_bytes(4),
            Some(1),
            None,
            None,
            false,
        );

        let swap1 = Node::from_raw_parts(
            // 1
            u32::to_be_bytes(2),
            u32::to_be_bytes(5),
            Some(2),
            Some(3),
            Some(0),
            false,
        );

        let swap2 = Node::from_raw_parts(
            // 2
            u32::to_be_bytes(3),
            u32::to_be_bytes(6),
            Some(4),
            None,
            Some(1),
            true,
        );

        let node1 = Node::from_raw_parts(
            // 3
            u32::to_be_bytes(7),
            u32::to_be_bytes(9),
            None,
            None,
            Some(1),
            true,
        );

        let node2 = Node::from_raw_parts(
            // 4
            u32::to_be_bytes(8),
            u32::to_be_bytes(8),
            None,
            None,
            Some(2),
            false,
        );

        tree.set_node(0, &parent);
        tree.set_node(1, &swap1);
        tree.set_node(2, &swap2);
        tree.set_node(3, &node1);
        tree.set_node(4, &node2);
        tree.set_root(0, Some(0));
    }

    let mut expected_vec = create_vec(4, 4, 6, 1);

    let mut expected_tree =
        RBForest::<i32, u32, 4, 4>::init_slice(expected_vec.as_mut_slice(), 1).unwrap();
    // Final structure
    //          parent
    //           /
    // black-> swap2
    //        /   \
    //red-> swap1 node1 <-red
    //      /
    //  node2            <-black
    unsafe {
        let parent = Node::from_raw_parts(
            // 0
            u32::to_be_bytes(1),
            u32::to_be_bytes(4),
            Some(1),
            None,
            None,
            false,
        );

        let swap2 = Node::from_raw_parts(
            // 1
            u32::to_be_bytes(2),
            u32::to_be_bytes(5),
            Some(4),
            None,
            Some(1),
            true,
        );

        let swap1 = Node::from_raw_parts(
            // 2
            u32::to_be_bytes(3),
            u32::to_be_bytes(6),
            Some(2),
            Some(3),
            Some(0),
            false,
        );

        let node1 = Node::from_raw_parts(
            // 3
            u32::to_be_bytes(7),
            u32::to_be_bytes(9),
            None,
            None,
            Some(1),
            true,
        );

        let node2 = Node::from_raw_parts(
            // 4
            u32::to_be_bytes(8),
            u32::to_be_bytes(8),
            None,
            None,
            Some(2),
            false,
        );

        expected_tree.set_node(0, &parent);
        expected_tree.set_node(1, &swap2);
        expected_tree.set_node(2, &swap1);
        expected_tree.set_node(3, &node1);
        expected_tree.set_node(4, &node2);
        expected_tree.set_root(0, Some(0));
    }

    assert!(tree.struct_eq(0, &expected_tree, 0));
}

#[test]
fn test_tree_strings() {
    let mut vec = create_vec(4, 10, 10, 1);

    let mut tree = RBForest::<i32, String, 4, 10>::init_slice(vec.as_mut_slice(), 1).unwrap();
    assert!(tree.is_empty(0));

    assert_eq!(tree.insert(0, 12, "val".to_string()), Ok(None));
    assert_eq!(tree.insert(0, 32, "44".to_string()), Ok(None));
    assert_eq!(tree.insert(0, 123, "321".to_string()), Ok(None));
    assert_eq!(
        tree.insert(0, 123, "321".to_string()),
        Ok(Some("321".to_string()))
    );
    assert_eq!(tree.insert(0, 1, "2".to_string()), Ok(None));
    assert_eq!(tree.insert(0, 14, "32".to_string()), Ok(None));
    assert_eq!(tree.insert(0, 20, "41".to_string()), Ok(None));
    assert_eq!(tree.insert(0, 6, "64".to_string()), Ok(None));
    assert_eq!(tree.insert(0, 41, "22".to_string()), Ok(None));
    assert_eq!(tree.insert(0, 122, "14".to_string()), Ok(None));
    assert_eq!(
        tree.insert(0, 41, "99".to_string()),
        Ok(Some("22".to_string()))
    );
    assert_eq!(
        tree.insert(0, 12, "very long value".to_string()),
        Err(Error::ValueSerializationError)
    );

    assert_eq!(tree.get(0, &41).unwrap(), "99".to_string());
    assert_eq!(tree.get(0, &12).unwrap(), "val".to_string());
    assert_eq!(tree.len(0), 9);
}

#[test]
fn test_tree_string_keys() {
    let mut vec = create_vec(10, 10, 10, 1);

    let mut tree = RBForest::<String, String, 10, 10>::init_slice(vec.as_mut_slice(), 1).unwrap();
    assert!(tree.is_empty(0));

    assert_eq!(
        tree.insert(0, "12".to_string(), "val".to_string()),
        Ok(None)
    );
    assert_eq!(tree.insert(0, "32".to_string(), "44".to_string()), Ok(None));
    assert_eq!(
        tree.insert(0, "123".to_string(), "321".to_string()),
        Ok(None)
    );
    assert_eq!(
        tree.insert(0, "123".to_string(), "321".to_string()),
        Ok(Some("321".to_string()))
    );
    assert_eq!(tree.insert(0, "1".to_string(), "2".to_string()), Ok(None));
    assert_eq!(tree.insert(0, "14".to_string(), "32".to_string()), Ok(None));
    assert_eq!(tree.insert(0, "20".to_string(), "41".to_string()), Ok(None));
    assert_eq!(tree.insert(0, "6".to_string(), "64".to_string()), Ok(None));
    assert_eq!(tree.insert(0, "41".to_string(), "22".to_string()), Ok(None));
    assert_eq!(
        tree.insert(0, "122".to_string(), "14".to_string()),
        Ok(None)
    );
    assert_eq!(
        tree.insert(0, "41".to_string(), "99".to_string()),
        Ok(Some("22".to_string()))
    );

    assert_eq!(
        tree.insert(0, "12".to_string(), "very long value".to_string()),
        Err(Error::ValueSerializationError)
    );

    assert_eq!(
        tree.insert(0, "very long key".to_string(), "1".to_string()),
        Err(Error::KeySerializationError)
    );

    assert_eq!(tree.get(0, &"41".to_string()).unwrap(), "99".to_string());
    assert_eq!(tree.get(0, &"12".to_string()).unwrap(), "val".to_string());
    assert_eq!(tree.len(0), 9);
}

#[test]
fn delete() {
    let mut vec = create_vec(1, 1, 256, 1);

    let mut tree = RBForest::<u8, u8, 1, 1>::init_slice(vec.as_mut_slice(), 1).unwrap();
    assert!(tree.is_empty(0));

    for key in &INSERT_KEYS {
        assert_eq!(tree.insert(0, *key, *key), Ok(None));
    }

    for key in &INSERT_KEYS {
        assert_eq!(tree.get(0, key), Some(*key));
    }

    tree.child_parent_link_test(0);

    let mut len = INSERT_KEYS.len();
    assert_eq!(tree.len(0), len);
    for key in &INSERT_KEYS {
        assert_rm(key, 0, &mut tree);
        len -= 1;
        assert_eq!(tree.len(0), len);
    }
}

#[test]
fn pairs_iterator() {
    let mut vec = create_vec(1, 1, 256, 1);

    let mut tree = RBForest::<u8, u8, 1, 1>::init_slice(vec.as_mut_slice(), 1).unwrap();
    assert!(tree.is_empty(0));

    for key in &INSERT_KEYS {
        assert_eq!(tree.insert(0, *key, *key), Ok(None));
    }

    let tree_iter = tree.pairs(0);

    let tree_data: Vec<(u8, u8)> = tree_iter.collect();

    assert_eq!(tree_data.len(), INSERT_KEYS.len());

    let mut prev_elem = (0, 0);

    for elem in tree_data {
        assert!(prev_elem <= elem);
        prev_elem = elem;
    }
}

#[test]
fn too_small() {
    let mut vec = vec![1, 2, 3];
    let tree = RBForest::<u8, u8, 1, 1>::init_slice(vec.as_mut_slice(), 1).unwrap_err();
    assert_eq!(tree, Error::TooSmall);
}

#[test]
fn fractional_node_count() {
    let mut vec = vec![0; forest_size(1, 1, 1, 1) + 1];
    let tree = RBForest::<u8, u8, 1, 1>::init_slice(vec.as_mut_slice(), 1).unwrap_err();
    assert_eq!(tree, Error::WrongSliceSize);
}

// This is an example of byte-packed forest used to check binary compatibility
const FOREST_BYTES: [u8; 160] = [
    83, 108, 105, 99, 101, 95, 82, 66, 84, 114, 101, 101, 0, 1, 0, 1, 0, 0, 0, 8, 0, 0, 0, 3, 255,
    255, 255, 255, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 12, 1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 2, 5, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0,
    4, 3, 5, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 5, 5, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 4,
    2, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 4, 4, 3, 0, 0, 0, 6, 0, 0, 0, 5, 0, 0, 0, 6, 3, 0, 0,
    0, 7, 0, 0, 0, 3, 0, 0, 0, 1,
];

#[test]
fn deserialization() {
    let bytes_slice = &mut FOREST_BYTES.clone();
    let forest = unsafe { RBForest::<u8, u8, 1, 1>::from_slice(bytes_slice).unwrap() };
    let mut vec = create_vec(1, 1, 8, 3);
    let mut expected_tree = RBForest::<u8, u8, 1, 1>::init_slice(vec.as_mut_slice(), 3).unwrap();

    expected_tree.insert(0, 4, 3).unwrap();
    expected_tree.insert(0, 2, 9).unwrap();
    expected_tree.insert(0, 5, 1).unwrap();
    expected_tree.insert(1, 5, 7).unwrap();
    expected_tree.insert(1, 2, 5).unwrap();
    expected_tree.insert(1, 1, 2).unwrap();
    expected_tree.insert(2, 1, 4).unwrap();
    expected_tree.insert(1, 4, 0).unwrap();

    for root in 0..expected_tree.max_roots() {
        for (key_value, expected_key_value) in forest.pairs(root).zip(expected_tree.pairs(root)) {
            assert_eq!(key_value, expected_key_value);
        }
    }
}

pub fn create_vec(k_size: usize, v_size: usize, num_entries: usize, max_roots: usize) -> Vec<u8> {
    let len = mem::size_of::<Header>()
        + 4 * max_roots
        + (mem::size_of::<Node<0, 0>>() + k_size + v_size) * num_entries;
    vec![0; len]
}

pub fn assert_rm<K, V, const KSIZE: usize, const VSIZE: usize>(
    val: &K,
    tree_id: usize,
    tree: &mut RBForest<K, V, KSIZE, VSIZE>,
) where
    K: Eq + Ord + BorshDeserialize + BorshSerialize + Debug,
    V: Eq + BorshDeserialize + BorshSerialize + Debug,
    [(); mem::size_of::<Header>()]: Sized,
{
    dbg!(val);
    assert!(tree.is_balanced(tree_id));
    assert!(tree.contains_key(tree_id, val));
    assert!(tree.remove_entry(tree_id, val).is_some());
    tree.child_parent_link_test(tree_id);
    assert_eq!(tree.get_key_index(tree_id, val), None);
    assert!(tree.is_balanced(tree_id));
}

mod init_forest_tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn assert_init_test<const KSIZE: usize, const VSIZE: usize>(
        num_entries: usize,
        max_roots: usize,
    ) {
        let mut reference_vec = create_vec(KSIZE, VSIZE, num_entries, max_roots);
        let mut testing_vec = create_vec(KSIZE, VSIZE, num_entries, max_roots);

        RBForest::<i32, u32, KSIZE, VSIZE>::init_slice(reference_vec.as_mut_slice(), max_roots)
            .unwrap();

        init_forest(KSIZE, VSIZE, testing_vec.as_mut_slice(), max_roots).unwrap();

        assert_eq!(testing_vec, reference_vec);
    }

    #[test]
    fn one_one_tree() {
        assert_init_test::<1, 1>(10, 1);
    }

    #[test]
    fn one_ten_tree() {
        assert_init_test::<1, 10>(10, 1);
    }

    #[test]
    fn one_one_forest() {
        assert_init_test::<1, 1>(10, 10);
    }

    #[test]
    fn one_ten_forest() {
        assert_init_test::<1, 10>(10, 10);
    }
}

mod init_forest {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn init() {
        let mut vec = create_vec(4, 4, 5, 1);

        init_forest(4, 4, vec.as_mut_slice(), 1).unwrap();
        let mut tree =
            unsafe { RBForest::<i32, u32, 4, 4>::from_slice(vec.as_mut_slice()).unwrap() };
        assert!(tree.is_empty(0));

        assert_eq!(tree.insert(0, 12, 32), Ok(None));
        assert_eq!(tree.get(0, &12), Some(32));
        assert_eq!(tree.len(0), 1);

        assert_eq!(tree.insert(0, 32, 44), Ok(None));
        assert_eq!(tree.get(0, &32), Some(44));
        assert_eq!(tree.len(0), 2);

        assert_eq!(tree.insert(0, 123, 321), Ok(None));
        assert_eq!(tree.get(0, &123), Some(321));
        assert_eq!(tree.len(0), 3);

        assert_eq!(tree.insert(0, 123, 322), Ok(Some(321)));
        assert_eq!(tree.get(0, &123), Some(322));
        assert_eq!(tree.len(0), 3);

        assert_eq!(tree.insert(0, 14, 32), Ok(None));
        assert_eq!(tree.get(0, &14), Some(32));
        assert_eq!(tree.len(0), 4);

        assert_eq!(tree.insert(0, 1, 2), Ok(None));
        assert_eq!(tree.insert(0, 1, 4), Ok(Some(2)));
        assert_eq!(tree.insert(0, 3, 4), Err(Error::NoNodesLeft));

        assert_eq!(tree.get(0, &15), None);

        assert_eq!(tree.len(0), 5);
    }

    #[test]
    fn too_small() {
        let mut vec = vec![1, 2, 3];
        let tree = init_forest(1, 1, vec.as_mut_slice(), 1).unwrap_err();
        assert_eq!(tree, Error::TooSmall);
    }

    #[test]
    fn fractional_node_count() {
        let mut vec = vec![0; forest_size(1, 1, 1, 1) + 1];
        let tree = init_forest(1, 1, vec.as_mut_slice(), 1).unwrap_err();
        assert_eq!(tree, Error::WrongSliceSize);
    }
}
