extends Node

@export var type :LogicChannel.ChannelType
@export var target_name :String
@export var target_func_name :String
@export var interface_name :String
@export var channel_number :int

var interface
var target
var target_func

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
	target_name = params["target_name"]
	target_func_name = params["target_func_name"]
	interface_name = params["interface_name"]
	channel_number = params["channel_number"]

func _enter_tree():
	if target == null:
		target = get_node("../" + target_name)
	if interface == null:
		interface = get_node("../" + interface_name)
	target_func = Callable(target, target_func_name)
	interface.add_output(channel_number, self)

func _exit_tree():
	interface.remove_output(channel_number, self)

func on_value_updated(value):
	target_func.call(value)
