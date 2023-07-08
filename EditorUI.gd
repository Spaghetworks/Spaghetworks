extends Control

var statemachine

func _ready():
	statemachine = get_node("/root/SceneStateMachine")

func _on_reload_pressed(): # Reload the editor by invoking a scene state transition
	statemachine.switch_scene(statemachine.scenes.SCENE_EDITOR)


func _on_to_sim_pressed():
	statemachine.switch_scene(statemachine.scenes.SCENE_SIMULATION, {"construct" : ConstructTranslator.to_world(get_node("../Construct_Root"))})
