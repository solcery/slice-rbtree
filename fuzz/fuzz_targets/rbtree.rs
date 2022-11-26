#![no_main]

use libfuzzer_sys::fuzz_target;
use slice_rbtree::tree::internal_checks::RBTreeMethod;
use slice_rbtree::tree::{tree_size, RBTree, TreeParams};
use std::mem::size_of;

const SIZE: usize = 40;

type Key = u8;
type Value = u8;

const TREE_PARAMS: TreeParams = TreeParams {
    k_size: size_of::<Key>(),
    v_size: size_of::<Value>(),
};
fuzz_target!(|methods: Vec<RBTreeMethod<Key, Value>>| {
    // fuzzed code goes here
    let expected_size = tree_size(TREE_PARAMS, SIZE);

    let mut slice = vec![0; expected_size];

    let mut tree: RBTree<Key, Value, { TREE_PARAMS.k_size }, { TREE_PARAMS.v_size }> =
        RBTree::init_slice(&mut slice).unwrap();

    for method in methods {
        tree.apply_method(method);
        assert!(tree.is_balanced());
        assert!(tree.no_double_red());
        assert!(tree.is_child_parent_links_consistent());
    }
});
