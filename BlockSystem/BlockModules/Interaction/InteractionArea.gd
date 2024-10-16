extends Area3D
class_name InteractionArea

signal interacted

func initialize(params):
	name = params["area_name"]
	collision_mask = 2
	collision_layer = 2
	var collision = CollisionShape3D.new()
	var shape = BoxShape3D.new()
	var dimensions = params["dimensions"]
	dimensions = Vector3(dimensions[0],dimensions[1],dimensions[2])
	dimensions /= 10
	shape.set_size(dimensions)
	collision.shape = shape
	add_child(collision)
	if params.has("offset"):
		var offset = params["offset"]
		offset = Vector3(offset[0],offset[1],offset[2])
		collision.position = offset

func interact():
	interacted.emit()
