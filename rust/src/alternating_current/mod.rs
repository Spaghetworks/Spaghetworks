use std::collections::HashSet;
use std::rc::{Rc, Weak};

use godot::engine::Node;
use godot::prelude::*;
use nalgebra::Complex;

mod constraints;
use constraints::*;

use crate::util::{TypedInstanceId, Validatable};

type Complex64 = Complex<f64>;

// An AcVertex represents either a node or a component in the electrical diagram.
#[derive(GodotClass)]
#[class(base=Node, no_init)]
pub struct AcVertex {
    base: Base<Node>,
    owner_system_node_id: TypedInstanceId<AcSystem>,
    vertex_type: VertexType,
    connections: Vec<Rc<AcConnection>>,
    constraints: ConstraintList,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AcConnection {
    node: TypedInstanceId<AcVertex>,
    component: TypedInstanceId<AcVertex>,
}

#[derive(Copy, Clone, Debug)]
pub enum VertexType {
    Node,
    Resistor(ResistorData),
    VoltageSource(VoltageSourceData),
    OtherComponent,
}
#[godot_api]
impl AcVertex {
    #[func]
    pub fn is_node(&self) -> bool {
        matches!(self.vertex_type, VertexType::Node)
    }
    pub fn is_component(&self) -> bool {
        !matches!(self.vertex_type, VertexType::Node)
    }
}

impl Validatable for AcVertex {
    fn is_valid(&self) -> bool {
        match self.vertex_type {
            VertexType::Node => true,
            VertexType::Resistor(_) | VertexType::VoltageSource(_) => {
                if self.connections.len() == 2 {
                    self.connections.iter().all(|c| c.component.is_valid())
                } else {
                    false
                }
            }
            VertexType::OtherComponent => true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResistorData {
    pub resistance: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct VoltageSourceData {
    pub voltage: Complex64,
}

/// Represents a bridge between two AcSystems
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AcBridge {
    base: Base<Node>,
    // Todo
}

// Represents a connected component in the electrical diagram with the same frequency
#[derive(GodotClass)]
#[class(base=Node)]
pub struct AcSystem {
    base: Base<Node>,
    descendant_nodes: Vec<TypedInstanceId<AcVertex>>,
    descendant_components: Vec<TypedInstanceId<AcVertex>>,
    contained_connections: Vec<Weak<AcConnection>>,
}

#[godot_api]
impl INode for AcSystem {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            descendant_nodes: Vec::new(),
            descendant_components: Vec::new(),
            contained_connections: Vec::new(),
        }
    }
}

#[godot_api]
impl AcSystem {
    #[func]
    pub fn create_node(&mut self) -> Gd<AcVertex> {
        let node = Gd::from_init_fn(|base| AcVertex {
            base,
            owner_system_node_id: self.to_gd().into(),
            vertex_type: VertexType::Node,
            connections: Vec::new(),
            constraints: ConstraintList::default(),
        });
        self.descendant_nodes.push(node.clone().into());
        node
    }

    #[func]
    pub fn create_resistor(&mut self) -> Gd<AcVertex> {
        let resistor = Gd::from_init_fn(|base| AcVertex {
            base,
            owner_system_node_id: self.to_gd().into(),
            vertex_type: VertexType::Resistor(ResistorData { resistance: 0f64 }),
            connections: Vec::new(),
            constraints: ConstraintList::default(),
        });

        resistor
    }
}
