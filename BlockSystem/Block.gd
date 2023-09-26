extends MeshInstance3D

@export var all_meshes:Array

func add_mesh(new_mesh):
	all_meshes.append(new_mesh)

func get_all_meshes():
	return all_meshes
