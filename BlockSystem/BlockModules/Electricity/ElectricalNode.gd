class_name ElectricalNode
extends RefCounted

const MIN_CAPACITANCE = 0.0001

# Assumed set on creation and never changed:
@export var capacitance: float = MIN_CAPACITANCE

# May change over time:
@export var charge: float = 0

# Internal:
var delta_charge: float = 0


func initialize(params) -> void:
	capacitance = max(params.capacitance, MIN_CAPACITANCE)

func get_voltage() -> float:
	return charge / capacitance

func get_charge() -> float:
	return charge

func get_capacitance() -> float:
	return capacitance

func send_charge_to(other_node : ElectricalNode, charge : float) -> void:
	if other_node != null:
		other_node._change_charge_by(-charge)
	else:
		# Magic charge input
		pass
	_change_charge_by(charge)

func on_after_electrical_update() -> void:
	charge += delta_charge
	delta_charge = 0

func _change_charge_by(additional_charge):
	delta_charge += additional_charge

# Data:
# Capacitance (charge per volt)
# Charge
