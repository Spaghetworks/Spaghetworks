use godot::engine::{Node, RefCounted};
use godot::prelude::*;

// An AcNode represents a node in the electrical diagram.
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct AcNode {
    #[base]
    base: Base<RefCounted>,
}

// Represents an edge in the electrical diagram
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct AcComponent {
    #[base]
    base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct AcBridge {
    #[base]
    base: Base<RefCounted>,
}

// Represents a connected component in the electrical diagram
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AcSystem {
    #[base]
    base: Base<Node>,
}
