#![doc = include_str!("../README.md")]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![cfg_attr(not(any(test, internal_checks, fuzzing)), no_std)]

use borsh::{BorshDeserialize, BorshSerialize};

pub mod forest;
pub mod tree;

/// Possible errors for [`RBTree`](tree::RBTree) and [`RBForest`](forest::RBForest)
#[derive(Debug, PartialEq, Eq, Copy, Clone, BorshDeserialize, BorshSerialize)]
pub enum Error {
    /// Failed to serialize key to key buffer, maybe it is too big?
    KeySerializationError,
    /// no free nodes left in the slice
    NoNodesLeft,
    /// the provided slice is too big for the map: the map internally uses `u32` indices, so there
    /// can't be more than `u32::MAX` nodes
    TooBig,
    /// the provided slice is too small for the map
    TooSmall,
    /// failed to serialize value to value buffer, maybe it is too big?
    ValueSerializationError,
    /// key size of the map does not match key size of the type
    WrongKeySize,
    /// struct header has incorrect magic, maybe it is not initialized?
    WrongMagic,
    /// node pool size from the map header does not match the actual slice size
    WrongNodePoolSize,
    /// slice size is incorrect
    WrongSliceSize,
    /// value size of the map does not match key size of the type
    WrongValueSize,
    /// There are fewer trees than the suppied tree_id
    TooBigTreeId,
}
