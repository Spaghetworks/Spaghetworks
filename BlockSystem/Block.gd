extends MeshInstance3D

signal on_to_editor
signal on_to_simulation
signal ui_requested
signal ui_provided

@export var all_meshes:Array
@export var all_collisions:Array

func add_mesh(new_mesh):
	all_meshes.append(new_mesh)

func get_all_meshes():
	return all_meshes

func add_collision(new_collision):
	all_collisions.append(new_collision)

func get_all_collisions():
	return all_collisions

func to_editor():
	on_to_editor.emit()

func to_simulation():
	on_to_simulation.emit()

func request_ui():
	ui_requested.emit()

func provide_ui(ui):
	ui_provided.emit(ui)
