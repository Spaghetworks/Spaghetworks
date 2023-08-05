extends Node

enum scenes {SCENE_NONE, SCENE_MENU, SCENE_EDITOR, SCENE_SIMULATION}

var scene_editor = preload("res://EditorSystem/Editor.tscn")
var scene_simulation = preload("res://SimulationSystem/Simulation.tscn")

var active_scene = scenes.SCENE_NONE
#var deferred_scene = scenes.SCENE_NONE
var active_scene_node

func _enter_tree():
	switch_scene(scenes.SCENE_EDITOR) 
	# Normally this would load the menu scene first, 
	# but as the menu doesn't yet exist it just goes straight to the editor.

func switch_scene(new_scene, payload = null):
	save_scene_state(active_scene)
	free_scene()
	
	load_scene(new_scene)
	load_scene_state(new_scene)
	if payload != null:
		active_scene_node.recieve(payload)
	
func save_scene_state(_scene):
	pass

func free_scene():
	if active_scene_node:
		active_scene_node.queue_free()

func load_scene(scene):
	var new_scene
	match scene:
#		scenes.SCENE_MENU:
		scenes.SCENE_EDITOR:
			new_scene = scene_editor.instantiate()
		scenes.SCENE_SIMULATION:
			new_scene = scene_simulation.instantiate()
			pass
	add_child(new_scene)
	active_scene_node = new_scene

func load_scene_state(_scene):
	pass
