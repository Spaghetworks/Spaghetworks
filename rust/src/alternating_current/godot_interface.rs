use super::ac_system::{AcSystem, AcVertexIdentifier, ConstraintIdentifier};
use super::RealType;
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(base=Node, no_init)]
pub struct AcSystemNode {
    base: Base<Node>,
    frequency: RealType,
    system: AcSystem<InstanceId>,
}
#[godot_api]
impl AcSystemNode {
    fn register_vertex(&mut self, vertex: Gd<AcVertexNode>) -> AcVertexIdentifier {
        self.system.register_vertex(vertex.instance_id())
    }
}

#[derive(Clone, Copy, Debug)]
struct AcVertexNodeSystemAssociation {
    system_node_identifier: InstanceId, // TODO
    associated_id: AcVertexIdentifier,
}

#[derive(GodotClass, Debug)]
#[class(base=Node)]
pub struct AcVertexNode {
    base: Base<Node>,
    #[export]
    measurement_type_hint: AcTypeHint,
    system_association: Option<AcVertexNodeSystemAssociation>,
}
#[godot_api]
impl INode for AcVertexNode {
    fn init(base: Base<Self::Base>) -> Self {
        AcVertexNode {
            base,
            measurement_type_hint: AcTypeHint::Unknown,
            system_association: None,
        }
    }
    fn enter_tree(&mut self) {}
}
#[godot_api]
impl AcVertexNode {
    fn associate_with_system_if_not_associated(&mut self, mut system: Gd<AcSystemNode>) {
        if self.system_association.is_none() {
            let view = AcVertexNodeSystemAssociation {
                system_node_identifier: system.instance_id(),
                associated_id: system.bind_mut().register_vertex(self.to_gd()),
            };
            self.system_association = Some(view);
        }
    }

    fn enter_tree(&mut self) {
        // Find the nearest AcSystem ancestor
        let mut iter_node = Some(self.to_gd().upcast::<Node>());
        let mut ancestor_system = None;
        while let Some(node) = iter_node {
            iter_node = node.get_parent();
            if let Ok(cast) = node.try_cast::<AcSystemNode>() {
                ancestor_system = Some(cast);
                break;
            }
        }
        // Register this with the system
        if let Some(system) = ancestor_system {
            self.associate_with_system_if_not_associated(system);
        }
    }
}

#[derive(Debug, GodotConvert, Var, Export)]
#[godot(via=GString)]
pub enum AcTypeHint {
    Unknown,
    Voltage,
    Current,
}
