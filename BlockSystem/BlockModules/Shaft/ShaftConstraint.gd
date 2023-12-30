extends Node

@export var element_names:Array
@export var ratio:Array # Number of turns of element a for number of turns of element b

var element_a
var element_b

func initialize(params):
	element_names = params["elements"]
	ratio = params["ratio"]
	print("ratio: ", ratio[0], ":",  ratio[1])
	name = params["constraint_name"]

func _enter_tree():
	if element_a == null:
		element_a = get_node("../" + element_names[0])
		element_b = get_node("../" + element_names[1])
		
		element_a.add_a_constraint(self)
		element_b.add_b_constraint(self)
