extends Node

@export var element_names:Array
@export var ratio:Array # Radius element a for radius of element b
@export var breaking_force:float # Maximum constraint force before slip
@export var type:String
@export var velocity:float

var element_a
var element_b
var update

func initialize(params):
	name = params["constraint_name"]
	element_names = params["elements"]
	type = params["type"]
	match params["type"]:
		"proportional":
			ratio = params["ratio"]
			print("ratio: ", ratio[0], ":",  ratio[1])
			breaking_force = params["breaking_force"]
		"constantvel":
			velocity = params["velocity"]
			ratio = [1,1]

func _enter_tree():
	if element_a == null:
		element_a = get_node("../" + element_names[0])
		element_a.add_a_constraint(self)
		
		if element_names.size() == 2:
			element_b = get_node("../" + element_names[1])
			element_b.add_b_constraint(self)
	update = true

#func _physics_process(_delta):
#	if !update:
#		return
#	if type == "constantvel":
#		if element_a.body && !element_a.rebuild_this_frame && element_a.shaft_system.physics_delay == 0:
#			print("setting velocity")
#			element_a.body.velocity = velocity
#			update = false
