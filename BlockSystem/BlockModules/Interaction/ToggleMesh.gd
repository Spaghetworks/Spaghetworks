extends MeshInstance3D

@export var true_transform :Transform3D
@export var false_transform :Transform3D
@export var source_area_name :String

var source_area
var state = false

func initialize(params):
	var file_mesh = load("res://BlockSystem/" + params["mesh_name"])
	set_mesh(file_mesh)
	true_transform = parse_transform(params["true_transform"])
	false_transform = parse_transform(params["false_transform"])
	transform = false_transform
	get_parent().add_mesh([file_mesh, position])
	source_area_name = params["source_area"]

func _enter_tree():
	if source_area:
		return
	source_area = get_node("../" + source_area_name)
	source_area.interacted.connect(on_source_interacted)

func on_source_interacted(new_state):
	state = new_state
	transform = true_transform if state else false_transform

func array_to_vector(arr):
	return Vector3(arr[0], arr[1], arr[2])

func parse_transform(data):
	return Transform3D(array_to_vector(data["x"]), array_to_vector(data["y"]), array_to_vector(data["z"]), array_to_vector(data["origin"]))
