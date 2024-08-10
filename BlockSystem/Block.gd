extends MeshInstance3D

signal on_to_editor
signal on_to_simulation
signal ui_requested
signal ui_provided

@export var all_meshes:Array
@export var all_collisions:Array
@export var aabb:AABB

func merge_aabb(new_aabb):
	if aabb == null:
		aabb = AABB()
	aabb = aabb.merge(new_aabb)
#	print(aabb)

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

func serialize_modules():
	var data = []
	for module in get_children():
		if module.has_method("serialize"):
			data.append(module.serialize())
	return data

func deserialize_modules(data):
	if data.size() > 0:
		for module in data:
			get_module(module["name"]).deserialize(module)

func get_module(module_name):
	for module in get_children():
		if module.name == module_name:
			return module
	return null
