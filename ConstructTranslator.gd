extends Node

static var root

func _ready():
	root = get_node("/root")

static func to_world(proto_construct):
	var com = Vector3.ZERO
	var mass = 0
	var moment = Vector3.ZERO
	# Convert the root Node3D to a RigidBody3D
	var construct = RigidBody3D.new()
	construct.name = "Construct_Root"
	proto_construct.replace_by(construct, true)
	# Iterate over all blocks:
	var block_mass
	var block_moment
	for block in construct.get_children():
		if !block is MeshInstance3D: # not actually a block, it's a construct module
			block.to_simulation()
			continue
		# Call the block's to_world to signal modules to enter world mode
		block.to_simulation()
		# Add the block to CoM and moment of inertia
		block_mass = block.get_meta("mass")
#		com = (com * mass + block.position * block_mass) / (mass + block_mass)
#
#		block_moment = Vector3(
#			block_mass * (block.position.y*block.position.y + block.position.z*block.position.z),
#			block_mass * (block.position.x*block.position.x + block.position.z*block.position.z),
#			block_mass * (block.position.x*block.position.x + block.position.y*block.position.y)
#			)
#		moment += block_moment
		mass += block_mass
		# Move the block's CollisionShape3D to the RigidBody preserving transform
		for node in block.get_node("Area3D").get_children():
			node.reparent(construct, true)
		# Free the Area3D
		block.get_node("Area3D").queue_free()
	# Correct the moment of inertia by parallel axis theorem
	moment -= Vector3(
		mass * (com.y*com.y + com.z*com.z),
		mass * (com.x*com.x + com.z*com.z),
		mass * (com.x*com.x + com.y*com.y)
		)
#	construct.set_center_of_mass_mode(RigidBody3D.CENTER_OF_MASS_MODE_CUSTOM)
#	construct.set_inertia(moment)
	construct.set_mass(mass)
#	construct.set_center_of_mass(com)
	proto_construct.queue_free()
	
	return construct

static func to_editor(live_construct):
	var construct_root = Node3D.new()
	for child in live_construct.get_children():
		if child is MeshInstance3D: # it's a block
			var block = root.get_node("/root/BlockLoader").blocks[child.get_meta("name")].duplicate(7)
			construct_root.add_child(block)
			block.transform = child.transform
			block.deserialize_modules(child.serialize_modules())
		elif child is CollisionShape3D: # it's a collision shape
			continue
		elif child is Node3D: # it's a construct module
			var module = child.new()
			construct_root.add_child(module)
			module.name = child.name
	return construct_root

static func to_file(proto_construct):
	var save_data = []
	for child in proto_construct.get_children():
		if !child is MeshInstance3D:
			# Skip things that aren't blocks (construct modules)
			continue
		var block_data = {}
		block_data["transform"] = serialize_transform(child.transform)
		block_data["block_name"] = child.get_meta("name")
		block_data["modules"] = child.serialize_modules()
		save_data.append(block_data)
	return JSON.stringify(save_data," ")

static func from_file(save_data):
	var construct_root = Node3D.new()
	construct_root.name = "Construct_Root"
	for block_data in save_data:
		# Place each block
		var block = root.get_node("/root/BlockLoader").blocks[block_data["block_name"]].duplicate(7)
		construct_root.add_child(block)
		block.transform = parse_transform(block_data["transform"])
		block.deserialize_modules(block_data["modules"])
	return construct_root
	

static func serialize_transform(xform):
	return {
		"x" : vector_to_array(xform.basis.x),
		"y" : vector_to_array(xform.basis.y),
		"z" : vector_to_array(xform.basis.z),
		"origin" : vector_to_array(xform.origin)
	}

static func vector_to_array(vec):
	return [vec.x, vec.y, vec.z]

static func array_to_vector(arr):
	return Vector3(arr[0], arr[1], arr[2])

static func parse_transform(data):
	return Transform3D(array_to_vector(data["x"]), array_to_vector(data["y"]), array_to_vector(data["z"]), array_to_vector(data["origin"]))
