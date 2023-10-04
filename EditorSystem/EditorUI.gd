extends Control

var statemachine
@onready var interaction = $InteractionMenu
@onready var interaction_list = $InteractionMenu/VBoxContainer/VBoxContainer

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
	interaction_list.add_child(ui)

func clear_interaction():
	print("clearing interaction menu")
	for node in interaction_list.get_children():
		interaction_list.remove_child(node)
