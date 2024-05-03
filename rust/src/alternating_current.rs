use godot::engine::{Node, RefCounted};
use godot::prelude::*;

// An AcNode represents a node in the electrical diagram. This can be constrained to have a set voltage
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AcNode {
    base: Base<Node>,
}

// Represents an edge in the electrical diagram
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AcComponent {
    base: Base<Node>,
}

/// Represents a bridge between two AcSystems
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AcBridge {
    base: Base<Node>,
    // Todo
}

// Represents a connected component in the electrical diagram
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AcSystem {
    base: Base<Node>,
    id_generator: IdentifierGenerator,
}

#[godot_api]
impl INode for AcSystem {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            id_generator: IdentifierGenerator::new(),
        }
    }
}
