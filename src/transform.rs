use crate::{schema::*, types::Node};

pub struct Transform {
    from_kind: u32,
    to_kind: u32,
    transform: fn(&Node) -> Node,
}

// TODO: could field additions be modelled as a transform?

pub static TRANSFORMS: &[Transform] = &[Transform {
    from_kind: 10,
    to_kind: 11,
    transform: |node| {
        let mut node = node.clone();
        node
    },
}];
