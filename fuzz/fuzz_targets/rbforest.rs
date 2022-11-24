#![no_main]

use libfuzzer_sys::fuzz_target;
use slice_rbtree::forest::internal_checks::RBForestMethod;
use slice_rbtree::forest::{forest_size, ForestParams, RBForest};
use std::mem::size_of;

const SIZE: usize = 40;

type Key = u8;
type Value = u8;

const FOREST_PARAMS: ForestParams = ForestParams {
    k_size: size_of::<Key>(),
    v_size: size_of::<Value>(),
    max_roots: 5,
};

fuzz_target!(|methods: Vec<RBForestMethod<Key, Value>>| {
    // fuzzed code goes here
    let expected_size = forest_size(FOREST_PARAMS, SIZE);

    let mut slice = vec![0; expected_size];

    let mut forest: RBForest<Key, Value, { FOREST_PARAMS.k_size }, { FOREST_PARAMS.v_size }> =
        RBForest::init_slice(&mut slice, FOREST_PARAMS.max_roots).unwrap();

    for method in methods {
        forest.apply_method(method);

        for i in 0..FOREST_PARAMS.max_roots {
            assert!(forest.is_balanced(i));
            assert!(forest.no_double_red(i));
            assert!(forest.is_child_parent_links_consistent(i));
        }
    }
});
