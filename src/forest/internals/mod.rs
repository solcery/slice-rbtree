//! # Intternal structure of the [`RBForest`](super::RBForest)
//!
//! Each [`RBForest`](super::RBForest) consists of [`Header`], array of the tree roots and a pool of [`Nodes`](Node).
//! All this structs are designed in such a way, that they does not have any alignment requirements
//! (all of them are byte-aligned).
//! [`Header`] contains parameters and sizes of sections and a magic string [`HEADER_MAGIC`](header::HEADER_MAGIC) used to check, that the given slice is indeed [`RBForest`](super::RBForest).
//!
//! After the [`Header`] the array of `max_roots` (see [`Header`] docs) indices is placed. Indices
//! are `Option<u32>` encoded as big-endian  `u32` with `None` variant encoded as `u32::MAX`.
//!
//!
//!The last part of the [`RBForest`](crate::forest::RBForest) is an array of `max_nodes` (see [`Header`] docs)
//![`Nodes`](Node).
//!
//![`from_slice()`](super::RBForest::from_slice) method checks the following invariants:
//! * magic string is present
//! * `KSIZE` and `VSIZE` matches corresponding fields in the [Header]
//! * node pool contains exactly `max_nodes` [Nodes](Node)
mod header;
mod node;

pub(crate) use header::Header;
pub(crate) use node::Node;
