use crate::{schema::*, types::Node};

pub struct Transform {
    from_kind: &'static str,
    to_kind: &'static str,
    transform: fn(&Node) -> Node,
}

// TODO: could field additions be modelled as a transform?

pub static TRANSFORMS: &[Transform] = &[Transform {
    from_kind: RUST_PRIMITIVE_TYPE_STR,
    to_kind: RUST_ARRAY_TYPE,
    transform: |node| {
        let mut node = node.clone();
        node.kind = RUST_ARRAY_TYPE.to_string();
        node
    },
}];
