extends Node

@export var damping:float
@export var element_names:Array

var element_a
var element_b
var ui

func initialize(params):
	name = params["damper_name"]
	damping = params["damping"]
	element_names = params["element_names"]

func _enter_tree():
	if element_a == null:
		element_a = get_node("../" + element_names[0])
		element_a.add_spring(self)
		element_b = get_node("../" + element_names[1])
		element_b.add_spring(self)
		
#		assemble_ui()
#		get_parent().ui_requested.connect(_on_ui_requested)

func get_torque(body):
	return -damping * (element_a.get_velocity() - element_b.get_velocity()) * (1 if body == element_a.body else -1)

func get_sub_torque(body):
	return -damping * (element_a.get_sub_vel() - element_b.get_sub_vel()) * (1 if body == element_a.body else -1)
