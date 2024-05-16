use std::collections::HashMap;

use super::RealType;
use super::VertexType;
use super::{
    ac_graph_data::{AcMeasurable, ResistorData},
    Constraint,
};
use crate::util::{TypedInstanceId, Validatable};
use godot::engine::Node;
use godot::prelude::*;

/// An AcVertex represents either a node or a component in the electrical diagram.
#[derive(GodotClass, Debug)]
#[class(base=Node, no_init)]
pub struct AcVertex {
    base: Base<Node>,

    up_to_date: bool,
    owner_system_id: TypedInstanceId<AcSystem>,
    vertex_type: VertexType,
    connections: Vec<AcConnection>,
    constraints: Vec<Constraint>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, GodotClass)]
#[class(no_init)]
pub struct AcConnection {
    node: TypedInstanceId<AcVertex>,
    component: TypedInstanceId<AcVertex>,
}

impl Validatable for AcConnection {
    fn is_valid(&self) -> bool {
        self.node.is_valid() && self.component.is_valid()
    }
}

#[godot_api]
impl AcVertex {
    #[func]
    pub fn is_node(&self) -> bool {
        matches!(self.vertex_type, VertexType::Node)
    }
    #[func]
    pub fn is_component(&self) -> bool {
        !matches!(self.vertex_type, VertexType::Node)
    }

    fn internal_mark_as_modified(&mut self) {
        self.up_to_date = false;
    }

    /// Returns true if the connection was dropped. Returns false otherwise.
    #[func]
    pub fn drop_connection_with_component(&mut self, component: Gd<AcVertex>) {
        debug_assert!(
            self.is_node(),
            "drop_connection_with_component was called on a component"
        );

        if let Ok(mut system) =
            <TypedInstanceId<AcSystem> as TryInto<Gd<AcSystem>>>::try_into(self.owner_system_id)
        {
            system.bind_mut().drop_connection(self.to_gd(), component)
        }
    }

    #[func]
    pub fn drop_connection_with_node(&mut self, node: Gd<AcVertex>) {
        debug_assert!(
            self.is_component(),
            "drop_component_with_node was called on a node"
        );

        if let Ok(mut system) =
            <TypedInstanceId<AcSystem> as TryInto<Gd<AcSystem>>>::try_into(self.owner_system_id)
        {
            system.bind_mut().drop_connection(node, self.to_gd())
        }
    }

    fn internal_try_drop_connection(&mut self, connection: AcConnection) -> Result<(), ()> {
        if let Some(index) = self.connections.iter().position(|x| x == &connection) {
            self.connections.swap_remove(index);
            Ok(())
        } else {
            Err(())
        }
    }

    fn internal_add_connection(&mut self, connection: AcConnection) {
        debug_assert!(
            (self.is_component() && connection.component == TypedInstanceId::from(self.to_gd()))
                || (self.is_node() && connection.node == TypedInstanceId::from(self.to_gd()))
        );
        self.internal_mark_as_modified();
        self.connections.push(connection);
    }
}

impl Validatable for AcVertex {
    fn is_valid(&self) -> bool {
        (match self.vertex_type {
            VertexType::Node => true,
            VertexType::Resistor(resistor_data) => {
                self.connections.len() == 2 && resistor_data.resistance >= (0 as RealType)
            }
            VertexType::VoltageSource(_) => self.connections.len() == 2,
            VertexType::OtherComponent => true,
        }) && self.connections.is_valid()
    }
}

/// Represents a bridge between two AcSystems
#[derive(GodotClass)]
#[class(no_init, base=Node)]
pub struct AcBridge {
    base: Base<Node>,
    // Todo
}

// Represents a connected component in the electrical diagram with the same frequency
#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct AcSystem {
    base: Base<Node>,
    nodes: Vec<TypedInstanceId<AcVertex>>,
    components: Vec<TypedInstanceId<AcVertex>>,
    connections: Vec<AcConnection>,
    measurable_to_id: HashMap<AcMeasurable, usize>,
}

#[godot_api]
impl INode for AcSystem {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            nodes: Vec::new(),
            components: Vec::new(),
            connections: Vec::new(),
            measurable_to_id: HashMap::new(),
        }
    }
}

#[godot_api]
impl AcSystem {
    #[func]
    pub fn create_node(&mut self) -> Gd<AcVertex> {
        let node = Gd::from_init_fn(|base| AcVertex {
            base,
            up_to_date: true,
            owner_system_id: self.to_gd().into(),
            vertex_type: VertexType::Node,
            connections: Vec::new(),
            constraints: Vec::with_capacity(1),
        });
        self.nodes.push(node.clone().into());
        node
    }

    #[func]
    pub fn create_resistor(&mut self) -> Gd<AcVertex> {
        let mut resistor = self.create_node();
        resistor.bind_mut().vertex_type = VertexType::Resistor(ResistorData {
            resistance: (0 as RealType),
        });
        resistor
    }

    #[func]
    pub fn create_connection(&mut self, mut node: Gd<AcVertex>, mut component: Gd<AcVertex>) {
        let node_id = TypedInstanceId::from(node.clone());
        let component_id = TypedInstanceId::from(component.clone());
        let connection = AcConnection {
            node: node_id,
            component: component_id,
        };

        debug_assert!(self.nodes.contains(&node_id));
        debug_assert!(self.components.contains(&component_id));

        if !self.connections.contains(&connection) {
            self.connections.push(connection);
            let mut node = node.bind_mut();
            let mut component = component.bind_mut();

            node.connections.push(connection);
            node.internal_mark_as_modified();
            component.connections.push(connection);
            node.internal_mark_as_modified();
        }
    }

    /// Attempts to drop a connection, removing it from the internal connection list as well as the
    /// connection lists of the endpoint node and component.
    #[func]
    pub fn drop_connection(&mut self, mut node: Gd<AcVertex>, mut component: Gd<AcVertex>) {
        let node_id: TypedInstanceId<AcVertex> = node.clone().into();
        let component_id: TypedInstanceId<AcVertex> = component.clone().into();

        let connection = AcConnection {
            node: node_id,
            component: component_id,
        };

        let option_connection_index = self.connections.iter().position(|x| x == &connection);

        if let Some(connection_index) = option_connection_index {
            let _ = node.bind_mut().internal_try_drop_connection(connection);
            let _ = component
                .bind_mut()
                .internal_try_drop_connection(connection);
            self.components.swap_remove(connection_index);
        }
    }

    /// Attempts to drop a node, removing it from the internal node list and removing all connections
    /// to it from components.
    #[func]
    pub fn drop_node(&mut self, node: Gd<AcVertex>) {
        self.internal_drop_vertex(node, true);
    }

    #[func]
    pub fn drop_component(&mut self, component: Gd<AcVertex>) {
        self.internal_drop_vertex(component, false);
    }

    fn internal_drop_vertex(&mut self, mut vertex: Gd<AcVertex>, is_node: bool) {
        let vertex_id: TypedInstanceId<AcVertex> = vertex.clone().into();
        let source_list = if is_node {
            &mut self.nodes
        } else {
            &mut self.components
        };

        debug_assert!(source_list.contains(&vertex_id));

        for connection in vertex.bind().connections.iter() {
            let connected_vertex_result: Result<Gd<AcVertex>, _> = if is_node {
                connection.component
            } else {
                connection.node
            }
            .try_into();
            if let Ok(mut connected_vertex) = connected_vertex_result {
                let _ = connected_vertex
                    .bind_mut()
                    .internal_try_drop_connection(*connection);
            }
        }
        vertex.bind_mut().connections.clear();
        if let Some(index) = source_list.iter().position(|x| x == &vertex_id) {
            source_list.swap_remove(index);
        }
    }
}
// TODO: Implement Kirchoff's Current Law in constraints
// TODO: Implement resistance in constraints
// TODO: Implement inductance in constraints
// TODO: Implement capacitance in constraints
// TODO: When a connection is added, modified, or removed, mark the vertices at both ends as modified.
// TODO: Add a map from (vertices, or component properties, or connections) -> (index in measurable vector)
// TODO: Auto-sync measurable vector layout to the map
// TODO: Add a dynamic matrix to the system
// TODO: Keep the matrix up to date with the constraints
// TODO: Go from Matrix x Measurable = Zero Vector to finding Measurable from Matrix
