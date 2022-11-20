//! Additional methods for self-consictency checking on [`RBTree`]
use super::*;
use crate::forest::Node;

#[warn(missing_docs)]
impl<'a, K, V, const KSIZE: usize, const VSIZE: usize> RBTree<'a, K, V, KSIZE, VSIZE>
where
    K: Eq + Ord + BorshDeserialize + BorshSerialize,
    V: Eq + BorshDeserialize + BorshSerialize,
{
    pub fn set_node(&mut self, id: usize, node: &Node<KSIZE, VSIZE>) {
        {
            self.0.set_node(id, node);
        }
    }

    pub fn struct_eq(&self, other: &Self) -> bool {
        self.0.struct_eq(0, &other.0, 0)
    }

    pub fn is_child_parent_links_consistent(&self) -> bool {
        self.0.is_child_parent_links_consistent(0)
    }

    #[must_use]
    pub fn is_balanced(&self) -> bool {
        self.0.is_balanced(0)
    }

    #[must_use]
    pub fn no_double_red(&self) -> bool {
        self.0.no_double_red(0)
    }

    pub fn apply_method(&mut self, method: RBTreeMethod<K, V>) {
        use RBTreeMethod::*;
        match method {
            Len => {
                let _ = self.len();
            }
            Clear => {
                let _ = self.clear();
            }
            FreeNodesLeft => {
                let _ = self.free_nodes_left();
            }
            Insert { key, value } => {
                let _ = self.insert(key, value);
            }
            ContainsKey(key) => {
                let _ = self.contains_key(&key);
            }
            Get(key) => {
                let _ = self.get(&key);
            }
            GetEntry(key) => {
                let _ = self.get_entry(&key);
            }
            Remove(key) => {
                let _ = self.remove(&key);
            }
            IsEmpty => {
                let _ = self.is_empty();
            }
            RemoveEntry(key) => {
                let _ = self.remove_entry(&key);
            }
            Delete(key) => {
                let _ = self.delete(&key);
            }
            FirstEntry => {
                let _ = self.first_entry();
            }
            LastEntry => {
                let _ = self.last_entry();
            }
            Pairs => {
                let iter = self.pairs();
                let _: Vec<_> = iter.collect();
            }
            Keys => {
                let iter = self.keys();
                let _: Vec<_> = iter.collect();
            }
            Values => {
                let iter = self.values();
                let _: Vec<_> = iter.collect();
            }
        }
    }
}

#[warn(missing_docs)]
#[derive(Debug)]
#[cfg_attr(fuzzing, derive(arbitrary::Arbitrary))]
pub enum RBTreeMethod<K, V> {
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
