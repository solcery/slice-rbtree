use bytemuck::{Pod, Zeroable};
use core::fmt;

const HEADER_MAGIC: [u8; 12] = *b"Slice_RBTree";

/// [RBForest](super::RBForest) header struct
#[repr(C)]
#[derive(Pod, Clone, Copy, Zeroable)]
pub struct Header {
    /// offset: 0 - Magic string, must be equal to [HEADER_MAGIC]
    magic: [u8; 12],
    /// offset: 12 - big-endian encoded `u16`, must be equal to `KSIZE` parameter of [`RBForest`](super::RBForest)
    k_size: [u8; 2],
    /// offset: 14 - big-endian encoded `u16`, must be equal to `VSIZE` parameter of [`RBForest`](super::RBForest)
    v_size: [u8; 2],
    /// offset: 16 - big-endian encoded `u32`, must be equal to the node pool size
    max_nodes: [u8; 4],
    /// offset: 20 - big-endian encoded `u32`, must be equal to the size of the array of tree roots
    max_roots: [u8; 4],
    /// offset: 24 - `Option<u32>`  encoded as big-endian `u32` with `None` value represented by
    /// `u32::MAX`, head of the linked list of empty nodes
    head: [u8; 4],
}

impl Header {
    pub fn k_size(&self) -> u16 {
        u16::from_be_bytes(self.k_size)
    }

    pub fn v_size(&self) -> u16 {
        u16::from_be_bytes(self.v_size)
    }

    pub fn max_nodes(&self) -> u32 {
        u32::from_be_bytes(self.max_nodes)
    }

    pub fn max_roots(&self) -> u32 {
        u32::from_be_bytes(self.max_roots)
    }

    pub fn head(&self) -> Option<u32> {
        let num = u32::from_be_bytes(self.head);
        if num == u32::MAX {
            None
        } else {
            Some(num)
        }
    }

    pub unsafe fn set_head(&mut self, head: Option<u32>) {
        match head {
            Some(idx) => {
                assert!(idx < u32::MAX);
                self.head = u32::to_be_bytes(idx);
            }
            None => {
                self.head = u32::to_be_bytes(u32::MAX);
            }
        }
    }

    /// This function guarantees, that the header will be initialized in fully known state
    pub unsafe fn fill(
        &mut self,
        k_size: u16,
        v_size: u16,
        max_nodes: u32,
        max_roots: u32,
        head: Option<u32>,
    ) {
        self.k_size = u16::to_be_bytes(k_size);
        self.v_size = u16::to_be_bytes(v_size);
        self.max_nodes = u32::to_be_bytes(max_nodes);
        self.max_roots = u32::to_be_bytes(max_roots);
        self.magic = HEADER_MAGIC;
        unsafe {
            self.set_head(head);
        }
    }

    pub fn check_magic(&self) -> bool {
        self.magic == HEADER_MAGIC
    }

    #[cfg(test)]
    unsafe fn from_raw_parts(
        k_size: u16,
        v_size: u16,
        max_nodes: u32,
        max_roots: u32,
        head: Option<u32>,
    ) -> Self {
        let k_size = u16::to_be_bytes(k_size);
        let v_size = u16::to_be_bytes(v_size);
        let max_nodes = u32::to_be_bytes(max_nodes);

        let max_roots = u32::to_be_bytes(max_roots);

        let head = match head {
            Some(index) => u32::to_be_bytes(index),
            None => u32::to_be_bytes(u32::MAX),
        };

        Self {
            k_size,
            v_size,
            max_nodes,
            max_roots,
            head,
            magic: HEADER_MAGIC,
        }
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Header")
            .field("k_size", &self.k_size())
            .field("v_size", &self.v_size())
            .field("max_nodes", &self.max_nodes())
            .field("max_roots", &self.max_roots())
            .field("head", &self.head())
            .finish()
    }
}

#[cfg(test)]
mod header_tests {
    use super::*;
    use paste::paste;
    use pretty_assertions::assert_eq;

    #[test]
    fn head() {
        let mut head = unsafe { Header::from_raw_parts(1, 2, 3, 5, None) };

        unsafe {
            head.set_head(Some(1));
        }
        assert_eq!(head.head(), Some(1));

        unsafe {
            head.set_head(Some(2));
        }
        assert_eq!(head.head(), Some(2));
        unsafe {
            paste! {
                head.set_head(None);
            }
        }

        assert_eq!(head.k_size(), 1);
        assert_eq!(head.v_size(), 2);
        assert_eq!(head.max_nodes(), 3);
        assert_eq!(head.max_roots(), 5);
        assert_eq!(head.head(), None);
    }
}
