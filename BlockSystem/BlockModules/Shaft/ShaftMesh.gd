extends MeshInstance3D

@export var element_name:String
var element
var last_position = 0
var last_velocity = 0

# DEBUG
var first_time

func initialize(params):
	var file_mesh = load("res://BlockSystem/" + params["mesh_name"])
	set_mesh(file_mesh)
	if params.has("offset"):
		position = Vector3(params["offset"][0],params["offset"][1],params["offset"][2])
	get_parent().add_mesh([file_mesh, position])
	element_name = params["element_name"]

func _enter_tree():
	first_time = Time.get_unix_time_from_system()
	if element:
		return
	element = get_node("../" + element_name)
	element.state_updated.connect(on_state_updated)

func on_state_updated(pos, vel):
#	if sign(vel) != sign(last_velocity) && vel < 0:
#		print(str(Time.get_unix_time_from_system() - first_time) + " " + str(pos))
	transform = transform.rotated_local(element.axis, pos - last_position)
	last_position = pos
	last_velocity = vel
