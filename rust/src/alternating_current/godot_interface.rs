use std::iter::{Empty, FlatMap, Once};

use crate::util::TypedInstanceId;

use super::ac_system::AcCalculator;
use super::component_abstraction::ElectricalNode;
use super::RealType;
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node, no_init)]
pub struct AcSystemNode {
    base: Base<Node>,
    frequency: RealType,
    calculator: AcCalculator<InstanceId>,
}

#[derive(Debug)]
enum ParentOrNode {
    Parent(TypedInstanceId<WireTerminal>),
    Node(ElectricalNode),
}
impl ParentOrNode {
    fn is_parent(&self) -> bool {
        matches!(self, ParentOrNode::Parent(_))
    }
    fn is_node(&self) -> bool {
        matches!(self, ParentOrNode::Node(_))
    }

    fn node(&self) -> Option<&ElectricalNode> {
        match self {
            Self::Node(node) => Some(node),
            _ => None,
        }
    }
    fn node_mut(&mut self) -> Option<&mut ElectricalNode> {
        match self {
            Self::Node(node) => Some(node),
            _ => None,
        }
    }

    fn parent(&self) -> Option<TypedInstanceId<WireTerminal>> {
        match self {
            Self::Parent(parent) => Some(*parent),
            _ => None,
        }
    }
}

#[derive(GodotClass, Debug)]
#[class(base=Node, no_init)]
pub struct WireTerminal {
    base: Base<Node>,
    // Implementation details
    parent_strong_connection: ParentOrNode,
    child_strong_connections: Vec<TypedInstanceId<WireTerminal>>,
    weak_connections: Vec<TypedInstanceId<WireTerminal>>,
}

#[godot_api]
impl WireTerminal {
    #[func]
    fn add_connection(&mut self, mut other: Gd<WireTerminal>) {
        if self.is_connected(other.clone()) {
            // Already directly connected using a wire; should do nothing
        } else if self.does_share_node(other.clone()) {
            // Already indirectly connected; add a weak connection
            self.weak_connections.push((&other).into());
            other
                .bind_mut()
                .weak_connections
                .push((&self.to_gd()).into());
        } else if self.parent_strong_connection.is_node() {
            // We're not connected, and self has no parent
            other
                .bind_mut()
                .child_strong_connections
                .push((&self.to_gd()).into());
            if let ParentOrNode::Node(old_node) = core::mem::replace(
                &mut self.parent_strong_connection,
                ParentOrNode::Parent((&other).into()),
            ) {
                old_node.merge_into(
                    self.get_root()
                        .bind_mut()
                        .parent_strong_connection
                        .node_mut()
                        .expect("Invariant broken: Root should always have ParentOrNode::Node"),
                );
            } else {
                panic!(
                    "wtf is_node() was true so mem::replace'ing should return a ParentOrNode::Node"
                );
            }
        } else if other.bind().parent_strong_connection.is_node() {
            self.child_strong_connections.push((&other).into());
            let mut other_node = other.bind_mut();
            if let ParentOrNode::Node(old_node) = core::mem::replace(
                &mut other_node.parent_strong_connection,
                ParentOrNode::Parent((&self.to_gd()).into()),
            ) {
                old_node.merge_into(
                    self.get_root()
                        .bind_mut()
                        .parent_strong_connection
                        .node_mut()
                        .expect("Invariant broken: Root should always have ParentOrNode::Node"),
                );
            }
        } else {
            self.reroot();
            if let ParentOrNode::Node(old_node) = core::mem::replace(
                &mut self.parent_strong_connection,
                ParentOrNode::Parent((&other).into()),
            ) {
                old_node.merge_into(
                    self.get_root()
                        .bind_mut()
                        .parent_strong_connection
                        .node_mut()
                        .expect("Invariant broken: Root should always have ParentOrNode::Node"),
                );
            } else {
                panic!("Invariant broken: Root should always have ParentOrNode::Node")
            }
            todo!("add_connection({})", other)
        }
    }
    #[func]
    fn remove_connection(&mut self, other: Gd<WireTerminal>) {
        let _ = self
            .try_remove_weak_connection(other.clone())
            .or_else(|_| self.try_remove_strong_connection(other));
    }

    fn try_remove_strong_connection(&mut self, mut other: Gd<WireTerminal>) -> Result<(), ()> {
        let other_identifier = TypedInstanceId::from(&other);

        if self
            .parent_strong_connection
            .parent()
            .is_some_and(|parent| parent == other_identifier)
        {
            //
            let _ = self.remove_parent();
            Ok(())
        } else if self.child_strong_connections.contains(&other_identifier) {
            let _ = other.bind_mut().remove_parent();
            todo!()
        } else {
            Err(())
        }
    }
    fn remove_parent(&mut self) -> Result<(), godot::builtin::meta::ConvertError> {
        if let ParentOrNode::Parent(parent_id) = self.parent_strong_connection {
            // TODO
            let self_id = TypedInstanceId::from(&self.to_gd());
            let parent = Gd::try_from(&parent_id)?;
            let parent = parent.bind();

            self.parent_strong_connection = todo!("Invalid state");
            let mut own_connected_component = self.get_strong_descendants();
            own_connected_component.push(self_id);

            let mut parent_connected_component = parent.get_strong_descendants();
            parent_connected_component.push(parent_id);
        } else {
            Ok(())
        }
    }

    fn try_remove_weak_connection(&mut self, mut other: Gd<WireTerminal>) -> Result<(), ()> {
        let self_identifier = TypedInstanceId::from(&self.to_gd());
        let other_identifier = TypedInstanceId::from(&other);

        let mut found_index = None;
        for (index, weak_connection) in self.weak_connections.iter().enumerate() {
            if other_identifier == *weak_connection {
                found_index = Some(index);
                break;
            }
        }
        if let Some(index) = found_index {
            self.weak_connections.swap_remove(index);

            let mut other_mut = other.bind_mut();
            let mut other_found_index = None;
            for (index, weak_connection) in other_mut.weak_connections.iter().enumerate() {
                if self_identifier == *weak_connection {
                    other_found_index = Some(index);
                    break;
                }
            }
            if let Some(other_index) = other_found_index {
                other_mut.weak_connections.swap_remove(other_index);
            } else {
                panic!("Invariant broken: Two WireTerminals which are weakly connected together should both have each others' ids in their weak_connection vector, but here only one vector contained the other id");
            }
            Ok(())
        } else {
            Err(())
        }
    }

    #[func]
    fn is_connected(&self, other: Gd<WireTerminal>) -> bool {
        let other_id = TypedInstanceId::from(&other);

        self.is_strongly_connected(other_id) || self.is_weakly_connected(other_id)
    }

    fn is_weakly_connected(&self, other_id: TypedInstanceId<WireTerminal>) -> bool {
        self.weak_connections.contains(&other_id)
    }
    fn is_strongly_connected(&self, other_id: TypedInstanceId<WireTerminal>) -> bool {
        self.parent_strong_connection
            .parent()
            .is_some_and(|parent| parent == other_id)
            || self.child_strong_connections.contains(&other_id)
    }

    #[func]
    fn does_share_node(&self, other: Gd<WireTerminal>) -> bool {
        todo!("{}", other)
    }
    fn get_root(&self) -> Gd<WireTerminal> {
        match &self.parent_strong_connection {
            ParentOrNode::Node(_) => self.to_gd(),
            ParentOrNode::Parent(parent) => parent.try_into().expect("Invariant broken: "),
        }
    }
    fn reroot(&mut self) {
        todo!("reroot");
    }

    fn get_strong_descendants(&self) -> Vec<TypedInstanceId<WireTerminal>> {
        Vec::from_iter(
            self.child_strong_connections
            .iter()
            .flat_map(|child| {
                std::iter::once(*child)
                    .chain(
                            Gd::try_from(child)
                            .expect("Invariant broken: a strongly connected child of a WireTerminal must be a valid TypedInstancedId")
                            .bind()
                            .get_strong_descendants()
                    )
                }
            )
        )
    }
}

#[derive(Debug, GodotConvert, Var, Export)]
#[godot(via=GString)]
pub enum AcTypeHint {
    Unknown,
    Voltage,
    Current,
}
