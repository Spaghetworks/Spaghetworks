extends Node

@export var torque:float
@export var element_name:String

var element
var ui

func initialize(params):
	name = params["source_name"]
	torque = params["torque"]
	element_name = params["element_name"]

func _enter_tree():
	if element == null:
		element = get_node("../" + element_name)
		element.add_spring(self)
		
		assemble_ui()
		get_parent().ui_requested.connect(_on_ui_requested)

func get_torque(body):
	return torque * element.get_alignment()

func get_sub_torque(body):
	return torque * element.get_alignment()

func _on_torque_changed(text):
	torque = float(text)

func _on_ui_requested():
	get_parent().provide_ui(ui)

func assemble_ui():
	ui = VBoxContainer.new()
	ui.name = "Torque Source"
	ui.add_child(Label.new())
	ui.get_child(0).text = name
	ui.add_child(GridContainer.new())
	
	var line = []
	line.append(Label.new())
	line.append(LineEdit.new())
	line[0].text = "Torque"
	line[1].text = str(torque)
	line[1].text_submitted.connect(_on_torque_changed)
	
	ui.get_child(1).add_child(line[0])
	ui.get_child(1).add_child(line[1])

func serialize():
	return {
		"name" : name,
		"torque" : torque
	}

func deserialize(data):
	torque = data["torque"]
