use std::mem::size_of;

pub trait Node {
    type Key;

    fn key_size_in_bits() -> usize {
        size_of::<Self::Key>() * 8
    }
    fn max_height() -> usize {
        Self::key_size_in_bits()
    }

    fn leaf_key(&self) -> Self::Key;
    fn is_leaf(&self) -> bool;
}

pub trait ParentNode: Node {
    fn left_child(&self) -> Self;
    fn right_child(&self) -> Self;
}