#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use slice_rbtree::tree::{tree_size, RBTree, TreeParams};
use std::mem::size_of;

const SIZE: usize = 1000;

type Key = u32;
type Value = [u64; 4];

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
        use RBTreeMethod::*;
        match method {
            Len => {
                let _ = tree.len();
            }
            Clear => {
                let _ = tree.clear();
            }
            FreeNodesLeft => {
                let _ = tree.free_nodes_left();
            }
            Insert { key, value } => {
                let _ = tree.insert(key, value);
            }
            ContainsKey(key) => {
                let _ = tree.contains_key(&key);
            }
            Get(key) => {
                let _ = tree.get(&key);
            }
            GetEntry(key) => {
                let _ = tree.get_entry(&key);
            }
            Remove(key) => {
                let _ = tree.remove(&key);
            }
            IsEmpty => {
                let _ = tree.is_empty();
            }
            RemoveEntry(key) => {
                let _ = tree.remove_entry(&key);
            }
            Delete(key) => {
                let _ = tree.delete(&key);
            }
            FirstEntry => {
                let _ = tree.first_entry();
            }
            LastEntry => {
                let _ = tree.last_entry();
            }
            Pairs => {
                let iter = tree.pairs();
                let _: Vec<_> = iter.collect();
            }
            Keys => {
                let iter = tree.keys();
                let _: Vec<_> = iter.collect();
            }
            Values => {
                let iter = tree.values();
                let _: Vec<_> = iter.collect();
            }
        }
    }
});

#[derive(Arbitrary, Debug)]
enum RBTreeMethod<K, V> {
    Len,
    Clear,
    FreeNodesLeft,
    ContainsKey(K),
    GetEntry(K),
    Get(K),
    Insert { key: K, value: V },
    IsEmpty,
    Remove(K),
    RemoveEntry(K),
    Delete(K),
    FirstEntry,
    LastEntry,
    Pairs,
    Keys,
    Values,
}
