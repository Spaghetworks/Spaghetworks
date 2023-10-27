class_name ElectricalSimpleElement
extends RefCounted

enum ConnectionType { SOURCE, SINK, OBSERVE }

signal electrical_element_connected(element : ElectricalSimpleElement)
signal electrical_element_disconnected(element : ElectricalSimpleElement)

### @type {WeakRef<ElectricalNode>}
var source_end : WeakRef = weakref(null)
### @type {WeakRef<ElectricalNode>}
var sink_end : WeakRef = weakref(null)
 
@export_range(0, 0, 0, "or_less", "or_greater", "hide_slider", "suffix:amperes") \
	var current : float = 0

var delta_current : float = 0


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
