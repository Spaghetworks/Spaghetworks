extends Control

signal automove_toggled
signal select_toggled
signal copy_requested
signal cut_requested
signal paste_requested

var statemachine
@onready var interaction = $VBoxContainer2/InteractionMenu
@onready var interaction_list = $VBoxContainer2/InteractionMenu/VBoxContainer/VBoxContainer

func _ready():
	statemachine = get_node("/root/SceneStateMachine")

func _on_reload_pressed(): # Reload the editor by invoking a scene state transition
	statemachine.switch_scene(statemachine.scenes.SCENE_EDITOR)


func _on_to_sim_pressed():
	statemachine.switch_scene(statemachine.scenes.SCENE_SIMULATION, {"construct" : ConstructTranslator.to_world(get_node("../Construct_Root"))})

func show_interaction():
	interaction.visible = true

func hide_interaction():
	interaction.visible = false

func add_interaction(ui):
	if interaction_list.get_children().size() != 0:
		interaction_list.add_child(HSeparator.new())
	interaction_list.add_child(ui)

func clear_interaction():
	print("clearing interaction menu")
	for node in interaction_list.get_children():
		interaction_list.remove_child(node)

func _on_automove_toggled(state):
	automove_toggled.emit(state)

func _on_select_toggled(state):
	select_toggled.emit(state)
	$VBoxContainer2/Panel/HFlowContainer/Copy .disabled = !state
	$VBoxContainer2/Panel/HFlowContainer/Cut .disabled = !state


func _on_copy_pressed():
	copy_requested.emit()

func _on_cut_pressed():
	cut_requested.emit()

func _on_paste_pressed():
	paste_requested.emit()

func force_UI(param, state):
	match param:
		"select":
			$VBoxContainer2/Panel/HFlowContainer/Select .button_pressed = state
			$VBoxContainer2/Panel/HFlowContainer/Select .toggled.emit(state)
			pass
		"automove":
			pass
	pass
