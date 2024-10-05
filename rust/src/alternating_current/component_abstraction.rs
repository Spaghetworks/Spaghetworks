use std::{
    cell::{Ref, RefCell},
    num::NonZero,
    rc::Rc,
};

use super::ac_system::{AcCalculator, AcVertexIdentifier};

/// A collection of ElectricalTerminals pointing to itself and the associated AcVertexIdentifier of an electrical node
#[derive(Debug)]
pub(crate) struct ElectricalNode {
    inner: ElectricalNodeData,
}

#[derive(Debug)]
enum ElectricalNodeData {
    Root(ElectricalNodeRoot),
    Delegate(ElectricalNodeReference),
}

#[derive(Debug)]
struct ElectricalNodeRoot {
    vertex_identifier: AcVertexIdentifier,
    size: NonZero<usize>,
}

type ElectricalNodeReference = Rc<RefCell<ElectricalNode>>;

impl ElectricalNode {
    pub(crate) fn new<T: Clone>(calculator: &mut AcCalculator<T>, payload: T) -> Self {
        Self {
            inner: ElectricalNodeData::Root(ElectricalNodeRoot {
                vertex_identifier: calculator.register_vertex(payload),
                size: unsafe { NonZero::new_unchecked(1) },
            }),
        }
    }

    fn get_root(this: ElectricalNodeReference) -> ElectricalNodeReference {
        let x = match &this.borrow().inner {
            ElectricalNodeData::Root(_) => None,
            ElectricalNodeData::Delegate(parent) => Some(ElectricalNode::get_root(parent.clone())),
        };
        x.unwrap_or(this)
    }
}

impl Drop for ElectricalNode {
    fn drop(&mut self) {
        // Need to destroy the associated AcVertexIdentifier in the AcCalculator
    }
}

pub(crate) struct NodeSplitOffBuilder {}

/// A reference to an ElectricalNode
#[derive(Debug)]
pub(crate) struct ElectricalTerminal {}
impl ElectricalTerminal {
    pub(crate) fn new() -> Self {
        todo!()
    }

    fn merge_node_with(&mut self, terminal: &mut ElectricalTerminal) {
        todo!()
    }
    fn split_off(&mut self) {
        todo!()
    }
    fn set_node_to(&mut self, terminal: &mut ElectricalTerminal) {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct ElectricalComponent {
    source_terminal: !,
    sink_terminal: !,
    sense_terminals: Vec<!>,
    current_node: !,
    current_constraint: !,
}
