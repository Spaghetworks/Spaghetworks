extends Node

@export var spring_constant:float
@export var element_names:Array
@export var breaking_torque:float
@export var spring_preload:float

var element_a
var element_b
var ui


func initialize(params):
	name = params["spring_name"]
	element_names = params["elements"]
	spring_constant = params["spring_constant"]
	print(params.has("spring_preload"))
	if params.has("spring_preload"):
		spring_preload = params["spring_preload"]
	else:
		spring_preload = 1

func _enter_tree():
	if element_a && element_b:
		return
	element_a = get_node("../" + element_names[0])
	element_b = get_node("../" + element_names[1])
	
	element_a.add_spring(self)
	element_b.add_spring(self)
	
	assemble_ui()
	get_parent().ui_requested.connect(_on_ui_requested)

func get_torque(body):
	var displacement = \
		element_a.get_position() - \
		element_b.get_position()
	displacement += spring_preload
	if body != element_b.body:
		displacement *= -1
#	print(displacement)
	return displacement * spring_constant
	
func get_sub_torque(body):
	var displacement = \
		element_a.get_sub_pos() - \
		element_b.get_sub_pos()
	displacement += spring_preload
	if body == element_a.body:
		displacement *= -1
		return displacement * spring_constant * element_a.get_alignment()
	else:
		return displacement * spring_constant * element_b.get_alignment()

func assemble_ui():
	ui = VBoxContainer.new()
	ui.name = "spring"
	ui.add_child(Label.new())
	ui.get_child(0).text = name
	ui.add_child(GridContainer.new())
	
	var line = []
	line.append(Label.new())
	line.append(LineEdit.new())
	line[0].text = "Preload"
	line[1].text = str(spring_preload)
	line[1].text_submitted.connect(_on_spring_preload_changed)
	
	ui.get_child(1).add_child(line[0])
	ui.get_child(1).add_child(line[1])

func _on_spring_preload_changed(text):
	print(text)
	spring_preload = float(text)

func _on_ui_requested():
	get_parent().provide_ui(ui)











