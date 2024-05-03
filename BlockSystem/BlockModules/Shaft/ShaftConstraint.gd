extends Node

@export var element_names:Array
@export var ratio:Array # Radius element a for radius of element b
@export var breaking_force:float # Maximum constraint force before slip
@export var type:String
@export var velocity:float

var element_a
var element_b
var ui

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
		assemble_ui()
		get_parent().ui_requested.connect(_on_ui_requested)
		
		element_a = get_node("../" + element_names[0])
		element_a.add_a_constraint(self)
		
		if element_names.size() == 2:
			element_b = get_node("../" + element_names[1])
			element_b.add_b_constraint(self)

func _on_constraint_velocity_changed(text):
	print(text)
	velocity = float(text)

func _on_breaking_force_changed(text):
	print(text)
	breaking_force = float(text)

func assemble_ui():
	ui = VBoxContainer.new()
	ui.name = "Constraint"
	ui.add_child(Label.new())
	ui.get_child(0).text = name
	ui.add_child(GridContainer.new())
	
	var lines = []
	var entry
	entry = Label.new()
	entry.text = "Type: " + type
	lines.append(entry)
	match type:
		"proportional":
			entry = Label.new()
			entry.text = "Breaking Force"
			lines.append(entry)
			entry = LineEdit.new()
			entry.text = str(breaking_force)
			entry.text_submitted.connect(_on_breaking_force_changed)
			lines.append(entry)
		
		"constantvel":
			entry = Label.new()
			entry.text = "Velocity"
			lines.append(entry)
			entry = LineEdit.new()
			entry.text = str(velocity)
			entry.text_submitted.connect(_on_constraint_velocity_changed)
			lines.append(entry)
	
	for line in lines:
		ui.get_child(1).add_child(line)

func _on_ui_requested():
	get_parent().provide_ui(ui)
