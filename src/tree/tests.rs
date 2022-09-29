use super::*;
use crate::forest::{tests as forest_helpers, Node};
use core::fmt::Debug;
use pretty_assertions::assert_eq;

impl<'a, K, V, const KSIZE: usize, const VSIZE: usize> RBTree<'a, K, V, KSIZE, VSIZE>
where
    K: Eq + Ord + BorshDeserialize + BorshSerialize,
    V: Eq + BorshDeserialize + BorshSerialize,
{
    unsafe fn set_node(&mut self, id: usize, node: &Node<KSIZE, VSIZE>) {
        unsafe {
            self.0.set_node(id, node);
        }
    }

    fn struct_eq(&self, other: &Self) -> bool {
        self.0.struct_eq(0, &other.0, 0)
    }

    fn child_parent_link_test(&self) {
        self.0.child_parent_link_test(0)
    }
}

#[test]
fn init() {
    let mut vec = create_vec(4, 4, 5);

    let mut tree = RBTree::<i32, u32, 4, 4>::init_slice(vec.as_mut_slice()).unwrap();
    assert!(tree.is_empty());

    assert_eq!(tree.insert(12, 32), Ok(None));
    assert_eq!(tree.get(&12), Some(32));
    assert_eq!(tree.len(), 1);

    assert_eq!(tree.insert(32, 44), Ok(None));
    assert_eq!(tree.get(&32), Some(44));
    assert_eq!(tree.len(), 2);

    assert_eq!(tree.insert(123, 321), Ok(None));
    assert_eq!(tree.get(&123), Some(321));
    assert_eq!(tree.len(), 3);

    assert_eq!(tree.insert(123, 322), Ok(Some(321)));
    assert_eq!(tree.get(&123), Some(322));
    assert_eq!(tree.len(), 3);

    assert_eq!(tree.insert(14, 32), Ok(None));
    assert_eq!(tree.get(&14), Some(32));
    assert_eq!(tree.len(), 4);

    assert_eq!(tree.insert(1, 2), Ok(None));
    assert_eq!(tree.insert(1, 4), Ok(Some(2)));
    assert_eq!(tree.insert(3, 4), Err(Error::NoNodesLeft));

    assert_eq!(tree.get(&15), None);

    assert_eq!(tree.len(), 5);
}

#[test]
fn swap_nodes() {
    let mut vec = create_vec(4, 4, 6);

    let mut tree = RBTree::<i32, u32, 4, 4>::init_slice(vec.as_mut_slice()).unwrap();
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
    }

    let mut expected_vec = create_vec(4, 4, 6);

    let mut expected_tree =
        RBTree::<i32, u32, 4, 4>::init_slice(expected_vec.as_mut_slice()).unwrap();
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
    }

    assert!(tree.struct_eq(&expected_tree));
}

#[test]
fn test_tree_strings() {
    let mut vec = create_vec(4, 10, 10);

    let mut tree = RBTree::<i32, String, 4, 10>::init_slice(vec.as_mut_slice()).unwrap();
    assert!(tree.is_empty());

    assert_eq!(tree.insert(12, "val".to_string()), Ok(None));
    assert_eq!(tree.insert(32, "44".to_string()), Ok(None));
    assert_eq!(tree.insert(123, "321".to_string()), Ok(None));
    assert_eq!(
        tree.insert(123, "321".to_string()),
        Ok(Some("321".to_string()))
    );
    assert_eq!(tree.insert(1, "2".to_string()), Ok(None));
    assert_eq!(tree.insert(14, "32".to_string()), Ok(None));
    assert_eq!(tree.insert(20, "41".to_string()), Ok(None));
    assert_eq!(tree.insert(6, "64".to_string()), Ok(None));
    assert_eq!(tree.insert(41, "22".to_string()), Ok(None));
    assert_eq!(tree.insert(122, "14".to_string()), Ok(None));
    assert_eq!(
        tree.insert(41, "99".to_string()),
        Ok(Some("22".to_string()))
    );
    assert_eq!(
        tree.insert(12, "very long value".to_string()),
        Err(Error::ValueSerializationError)
    );

    assert_eq!(tree.get(&41).unwrap(), "99".to_string());
    assert_eq!(tree.get(&12).unwrap(), "val".to_string());
    assert_eq!(tree.len(), 9);
}

#[test]
fn test_tree_string_keys() {
    let mut vec = create_vec(10, 10, 10);

    let mut tree = RBTree::<String, String, 10, 10>::init_slice(vec.as_mut_slice()).unwrap();
    assert!(tree.is_empty());

    assert_eq!(tree.insert("12".to_string(), "val".to_string()), Ok(None));
    assert_eq!(tree.insert("32".to_string(), "44".to_string()), Ok(None));
    assert_eq!(tree.insert("123".to_string(), "321".to_string()), Ok(None));
    assert_eq!(
        tree.insert("123".to_string(), "321".to_string()),
        Ok(Some("321".to_string()))
    );
    assert_eq!(tree.insert("1".to_string(), "2".to_string()), Ok(None));
    assert_eq!(tree.insert("14".to_string(), "32".to_string()), Ok(None));
    assert_eq!(tree.insert("20".to_string(), "41".to_string()), Ok(None));
    assert_eq!(tree.insert("6".to_string(), "64".to_string()), Ok(None));
    assert_eq!(tree.insert("41".to_string(), "22".to_string()), Ok(None));
    assert_eq!(tree.insert("122".to_string(), "14".to_string()), Ok(None));
    assert_eq!(
        tree.insert("41".to_string(), "99".to_string()),
        Ok(Some("22".to_string()))
    );

    assert_eq!(
        tree.insert("12".to_string(), "very long value".to_string()),
        Err(Error::ValueSerializationError)
    );

    assert_eq!(
        tree.insert("very long key".to_string(), "1".to_string()),
        Err(Error::KeySerializationError)
    );

    assert_eq!(tree.get(&"41".to_string()).unwrap(), "99".to_string());
    assert_eq!(tree.get(&"12".to_string()).unwrap(), "val".to_string());
    assert_eq!(tree.len(), 9);
}

#[test]
fn delete() {
    let mut vec = create_vec(1, 1, 256);

    let mut tree = RBTree::<u8, u8, 1, 1>::init_slice(vec.as_mut_slice()).unwrap();
    assert!(tree.is_empty());

    for key in &forest_helpers::INSERT_KEYS {
        assert_eq!(tree.insert(*key, *key), Ok(None));
    }

    for key in &forest_helpers::INSERT_KEYS {
        assert_eq!(tree.get(key), Some(*key));
    }

    tree.child_parent_link_test();

    let mut len = forest_helpers::INSERT_KEYS.len();
    assert_eq!(tree.len(), len);

    for key in &forest_helpers::INSERT_KEYS {
        assert_rm(key, &mut tree);
        len -= 1;
        assert_eq!(tree.len(), len);
    }
}

#[test]
fn pairs() {
    let mut vec = create_vec(1, 1, 256);

    let mut tree = RBTree::<u8, u8, 1, 1>::init_slice(vec.as_mut_slice()).unwrap();
    assert!(tree.is_empty());

    for key in &forest_helpers::INSERT_KEYS {
        assert_eq!(tree.insert(*key, *key), Ok(None));
    }

    let tree_iter = tree.pairs();

    let tree_data: Vec<(u8, u8)> = tree_iter.collect();

    assert_eq!(tree_data.len(), forest_helpers::INSERT_KEYS.len());

    let mut prev_elem = (0, 0);

    for elem in tree_data {
        assert!(prev_elem <= elem);
        prev_elem = elem;
    }
}

fn create_vec(k_size: usize, v_size: usize, num_entries: usize) -> Vec<u8> {
    forest_helpers::create_vec(k_size, v_size, num_entries, 1)
}

fn assert_rm<K, V, const KSIZE: usize, const VSIZE: usize>(
    val: &K,
    tree: &mut RBTree<K, V, KSIZE, VSIZE>,
) where
    K: Eq + Ord + BorshDeserialize + BorshSerialize + Debug,
    V: Eq + BorshDeserialize + BorshSerialize + Debug,
{
    forest_helpers::assert_rm(val, 0, &mut tree.0);
}
