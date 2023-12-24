class_name ElectricalSimpleElement
extends RefCounted

enum ConnectionType { SOURCE, SINK, OBSERVE }

signal electrical_element_connected(element : ElectricalSimpleElement)
signal electrical_element_disconnected(element : ElectricalSimpleElement)

# @param {energy} energy
signal energy_dissipated(energy : float)

### @type {resistance}
@export_range(1e-12, 100, 1, "or_greater", "suffix:ohms") var resistance : float = 1e-12
### @type {inductance}
@export_range(0, 100, 1, "or_greater", "suffix:henry") var inductance : float = 1e-9

# Reduces in charge when current is positive
### @type {WeakRef<ElectricalNode>}
var source_end : WeakRef = weakref(null)

# Increases in charge when current is positive
### @type {WeakRef<ElectricalNode>}
var sink_end : WeakRef = weakref(null)

# Maps from voltage drop to effective resistance
### @type {(voltage) -> resistance}
var get_effective_resistance : Callable = make_constant_resistor.bind(resistance);

var has_been_processed_this_tick : bool = false

### @type {current}
@export_range(0, 0, 0, "or_less", "or_greater", "hide_slider", "suffix:amperes") \
	var current : float = 0

# First differential of current in respect to time
### @type {current / seconds}
var delta_current : float = 0

### @param {seconds} delta_time
func _on_electrical_update(delta_time : float) -> void:
	var source_voltage : float = 0
	var sink_voltage : float = 0
	var source_node : ElectricalNode = source_end.get_ref() as ElectricalNode
	var sink_node : ElectricalNode = sink_end.get_ref() as ElectricalNode
	
	if source_node != null:
		source_voltage = source_node.get_voltage()
	if sink_node != null:
		sink_voltage = sink_node.get_voltage()
	
	var voltage_drop : float = source_voltage - sink_voltage
	var effective_resistance = get_effective_resistance.call(voltage_drop)
	if not (effective_resistance is float):
		assert(false, "get_effective_resistance was set to a function of a signature other than (float) -> float")
	
	# Calculate change in charges
	var correction_from_induction : float = get_inductance_correction(delta_time)
	var delta_charge : float = current * delta_time + correction_from_induction
	var energy_dissipated : float = delta_charge * absf(voltage_drop)
	
	# Update current and differential of current
	var last_current : float = current
	current = voltage_drop / effective_resistance
	delta_current = delta_current * exp(-delta_time * resistance / inductance) + (current - last_current) / delta_time
	
	_execute_charge_transfer(delta_charge, voltage_drop)

func _execute_charge_transfer(delta_charge : float, voltage_drop : float) -> void:
	var source_node : ElectricalNode = source_end.get_ref() as ElectricalNode
	var sink_node : ElectricalNode = sink_end.get_ref() as ElectricalNode
	
	var limit : float = INF
	
	if delta_charge > 0 and source_node != null:
		limit = source_node.get_numeric_charge_change_cutoff()
	elif delta_charge < 0 and sink_node != null:
		pass
	
	if source_node != null:
		source_node._change_charge_by(-delta_charge)
	if sink_node != null:
		sink_node._change_charge_by(delta_charge)
	energy_dissipated.emit(delta_charge * voltage_drop)

func _on_pre_electrical_update() -> void:
	has_been_processed_this_tick = false

func connect_electrical_nodes(source : ElectricalNode, sink : ElectricalNode) -> void:
	if source_end.get_ref() != null or sink_end.get_ref() != null:
		push_warning("[ElectricalSystem]", "Attempted to connect electrical component when already connected", get_stack())
		disconnect_electrical_nodes()
	
	source_end = weakref(source)
	sink_end = weakref(sink)
	
	if source != null:
		electrical_element_connected.connect(source._on_connected_element)
		electrical_element_disconnected.connect(source._on_disconnected_element)
	if sink != null:
		electrical_element_connected.connect(sink._on_connected_element)
		electrical_element_disconnected.connect(source._on_disconnected_element)
	
	electrical_element_connected.emit(self)
	electrical_element_connected.emit(self)

func get_inductance_correction(delta_time : float):
	var R_over_L : float = resistance / inductance
	var L_over_R : float = inductance / resistance
	var Lsq_over_Rsq : float = L_over_R * L_over_R
	return (exp(-delta_time * R_over_L) - 1) * Lsq_over_Rsq * delta_current

func disconnect_electrical_nodes():
	var source_element = source_end.get_ref() as ElectricalNode
	var sink_element = sink_end.get_ref() as ElectricalNode
	current = 0
	delta_current = 0
	source_end = weakref(null)
	sink_end = weakref(null)

	electrical_element_disconnected.emit(self, ConnectionType.SOURCE)
	electrical_element_disconnected.emit(self, ConnectionType.SINK)
	
	if source_element != null:
		electrical_element_connected.disconnect(source_element._on_connected_element)
		electrical_element_disconnected.disconnect(source_element._on_disconnected_element)
	if sink_element != null:
		electrical_element_connected.disconnect(sink_element._on_connected_element)
		electrical_element_disconnected.disconnect(sink_element._on_disconnected_element)
	if source_element == null and sink_element == null:
		push_warning("[ElectricalSystem]", "Attempting to disconnect already-disconnected element.", get_stack())

static func make_constant_resistor(resistance : float, _voltage : float):
	return resistance

static func make_piecewise_linear_diode(forward_resistance : float, backwards_resistance : float, voltage: float):
	return forward_resistance if voltage > 0 else backwards_resistance
