extends Node

@export var spring_constant:float
@export var element_names:Array
@export var breaking_torque:float

var element_a
var element_b

var spring_preload = 1

func initialize(params):
	name = params["spring_name"]
	element_names = params["elements"]
	spring_constant = params["spring_constant"]

func _enter_tree():
	element_a = get_node("../" + element_names[0])
	element_b = get_node("../" + element_names[1])
	
	element_a.add_spring(self)
	element_b.add_spring(self)

func get_torque(body):
	var displacement = \
		element_a.get_position() - \
		element_b.get_position()
	displacement += spring_preload
	if body != element_b.body:
		displacement *= -1
#	print(displacement)
	return displacement * spring_constant
