//! Iterators over [`RBTree`](crate::tree::RBTree) and [`RBForest`](crate::forest::RBForest)
use borsh::{BorshDeserialize, BorshSerialize};
use core::cmp::Ord;
use core::fmt;
use core::iter::FusedIterator;

use super::RBForest;

/// An iterator over key-value pairs ordered by key
pub struct PairsIterator<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    next_node: Option<usize>,
    tree: &'a RBForest<'b, K, V, KSIZE, VSIZE>,
}
impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> PairsIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    pub(super) fn from_raw_parts(
        tree: &'a RBForest<'b, K, V, KSIZE, VSIZE>,
        next_node: Option<usize>,
    ) -> Self {
        Self { next_node, tree }
    }
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> Iterator
    for PairsIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_node.map(|mut id| {
            let nodes = &self.tree.nodes;

            let key = K::deserialize(&mut nodes[id].key.as_slice()).expect("Key corrupted");
            let value = V::deserialize(&mut nodes[id].value.as_slice()).expect("Value corrupted");

            // find next
            if let Some(right_id) = nodes[id].right() {
                self.next_node = Some(self.tree.min(right_id as usize));
            } else {
                self.next_node = None;
                while let Some(parent_id) = nodes[id].parent() {
                    let parent_id = parent_id as usize;
                    if Some(id as u32) == nodes[parent_id].left() {
                        self.next_node = Some(parent_id);
                        break;
                    } else {
                        id = parent_id;
                    }
                }
            }

            (key, value)
        })
    }
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> FusedIterator
    for PairsIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> fmt::Debug
    for PairsIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize + fmt::Debug,
    V: BorshDeserialize + BorshSerialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let PairsIterator { next_node, tree } = self;
        let new_iter = PairsIterator {
            next_node: *next_node,
            tree,
        };
        f.debug_map().entries(new_iter).finish()
    }
}

/// An ordered iterator over keys
pub struct KeysIterator<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    next_node: Option<usize>,
    tree: &'a RBForest<'b, K, V, KSIZE, VSIZE>,
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> KeysIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    pub(super) fn from_raw_parts(
        tree: &'a RBForest<'b, K, V, KSIZE, VSIZE>,
        next_node: Option<usize>,
    ) -> Self {
        Self { next_node, tree }
    }
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> Iterator
    for KeysIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_node.map(|mut id| {
            let nodes = &self.tree.nodes;

            let key = K::deserialize(&mut nodes[id].key.as_slice()).expect("Key corrupted");

            // find next
            if let Some(right_id) = nodes[id].right() {
                self.next_node = Some(self.tree.min(right_id as usize));
            } else {
                self.next_node = None;
                while let Some(parent_id) = nodes[id].parent() {
                    let parent_id = parent_id as usize;
                    if Some(id as u32) == nodes[parent_id].left() {
                        self.next_node = Some(parent_id);
                        break;
                    } else {
                        id = parent_id;
                    }
                }
            }

            key
        })
    }
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> FusedIterator
    for KeysIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> fmt::Debug
    for KeysIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize + fmt::Debug,
    V: BorshDeserialize + BorshSerialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let KeysIterator { next_node, tree } = self;
        let new_iter = KeysIterator {
            next_node: *next_node,
            tree,
        };
        f.debug_set().entries(new_iter).finish()
    }
}

/// An iterator over values ordered by key
pub struct ValuesIterator<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    next_node: Option<usize>,
    tree: &'a RBForest<'b, K, V, KSIZE, VSIZE>,
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize>
    ValuesIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    pub(super) fn from_raw_parts(
        tree: &'a RBForest<'b, K, V, KSIZE, VSIZE>,
        next_node: Option<usize>,
    ) -> Self {
        Self { next_node, tree }
    }
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> Iterator
    for ValuesIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_node.map(|mut id| {
            let nodes = &self.tree.nodes;

            let value = V::deserialize(&mut nodes[id].value.as_slice()).expect("Value corrupted");

            // find next
            if let Some(right_id) = nodes[id].right() {
                self.next_node = Some(self.tree.min(right_id as usize));
            } else {
                self.next_node = None;
                while let Some(parent_id) = nodes[id].parent() {
                    let parent_id = parent_id as usize;
                    if Some(id as u32) == nodes[parent_id].left() {
                        self.next_node = Some(parent_id);
                        break;
                    } else {
                        id = parent_id;
                    }
                }
            }

            value
        })
    }
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> FusedIterator
    for ValuesIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize,
    V: BorshDeserialize + BorshSerialize,
{
}

impl<'a, 'b, K, V, const KSIZE: usize, const VSIZE: usize> fmt::Debug
    for ValuesIterator<'a, 'b, K, V, KSIZE, VSIZE>
where
    K: Ord + BorshDeserialize + BorshSerialize + fmt::Debug,
    V: BorshDeserialize + BorshSerialize + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let ValuesIterator { next_node, tree } = self;
        let new_iter = ValuesIterator {
            next_node: *next_node,
            tree,
        };
        f.debug_set().entries(new_iter).finish()
    }
}
