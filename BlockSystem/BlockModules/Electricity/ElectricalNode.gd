class_name ElectricalNode
extends RefCounted

const MIN_CAPACITANCE = 1e-12

# Assumed set on creation and never changed:
@export_category("Write-once")
@export_range(0, 100, 1, "or_greater", "hide_slider", "hide_slider", "suffix:farads") \
	var capacitance: float = MIN_CAPACITANCE
@export_subgroup("Voltage source")
@export var is_voltage_source : bool = false
@export_range(-10, 10, 0.5, "or_less", "or_greater", "suffix:volts") var source_voltage : float = 0

# May change over time:
@export_category("Dynamic")
@export_range(0, 0, 0, "or_less", "or_greater", "hide_slider", "suffix:colombs") var charge: float = 0
var incoming_elements : Array[ElectricalSimpleElement] = []
var outgoing_elements : Array[ElectricalSimpleElement] = []
var observing_elements: Array = [] # TODO: specify type
var connected_element_types: Dictionary = {}

# Internal:
var delta_charge: float = 0


#func initialize(params) -> void:
#	capacitance = max(params.capacitance, MIN_CAPACITANCE)

func get_voltage() -> float:
	return charge / capacitance

func get_charge() -> float:
	return charge

func get_numeric_charge_change_cutoff() -> float:
	return charge / 2

func _on_after_electrical_update() -> void:
	charge += delta_charge
	delta_charge = 0
	if is_voltage_source:
		charge = capacitance * source_voltage

func _on_connected_element(element : ElectricalSimpleElement):
	assert(not connected_element_types.has(element), "[ElectricalSystem] The electrical element must not be already connected to this electrical node")

	if element.source_end.get_ref() == self:
		outgoing_elements.append(element)
		connected_element_types[element] = ElectricalSimpleElement.ConnectionType.SOURCE
	elif element.sink_end.get_ref() == self:
		incoming_elements.append(element)
		connected_element_types[element] = ElectricalSimpleElement.ConnectionType.SINK
	else:
		push_warning("[ElectricalSystem]", "The electrical element is not connected to this electrical node")

func _on_disconnected_element(element : ElectricalSimpleElement):
	if connected_element_types.has(element):
		if connected_element_types[element] == ElectricalSimpleElement.ConnectionType.SOURCE:
			assert(outgoing_elements.has(element), "[ElectricalSystem] ElectricalNode in invalid state: connected_element_types indicates that element is connected with this node as a source, but outgoing_elements does not include element!")
			outgoing_elements.erase(element)
		elif connected_element_types[element] == ElectricalSimpleElement.ConnectionType.SINK:
			assert(incoming_elements.has(element), "[ElectricalSystem] ElectricalNode in invalid state: connected_element_types indicates that element is connected with this node as a sink, but incoming_elements does not include element!")
			incoming_elements.erase(element)
		else:
			push_warning("[ElectricalSystem]", "_on_disconnected_element called but electrical element was not connected with this electrical node as a source or sink")

func _on_element_begin_observe(_element):
	assert(false, "TODO: Not implemented")

func _on_element_end_observe(_element):
	assert(false, "TODO: Not implemented")

func _change_charge_by(additional_charge : float):
	delta_charge += additional_charge

# Data:
# Capacitance (charge per volt)
# Charge
