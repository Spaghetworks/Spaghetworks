extends Node

signal updated

@export var type :LogicChannel.ChannelType
@export var source_name :String
@export var source_signal :String
@export var interface_name :String
@export var channel_number :int

var source
var interface
var value

func initialize(params):
	name = params["listener_name"]
	match params["type"]:
		"bool":
			type = LogicChannel.ChannelType.BOOL
		"float_raw":
			type = LogicChannel.ChannelType.FLOAT_RAW
		"float_norm":
			type = LogicChannel.ChannelType.FLOAT_NORM
		"serial":
			type = LogicChannel.ChannelType.SERIAL
	source_name = params["source_name"]
	source_signal = params["source_signal"]
	interface_name = params["interface_name"]
	channel_number = params["channel_number"]

func _enter_tree():
	source = get_node("../" + source_name)
	interface = get_node("../" + interface_name)
	source.connect(source_signal, on_updated)
	interface.add_input(channel_number, self)

func _exit_tree():
	interface.remove_input(channel_number, self)

func on_updated(new_value):
	value = new_value
	updated.emit(value)
