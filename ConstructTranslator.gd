extends Node

static func to_world(proto_construct):
	var com = Vector3.ZERO
	var mass = 0
	var moment = Vector3.ZERO
	# Convert the root Node3D to a RigidBody3D
	var construct = RigidBody3D.new()
	proto_construct.replace_by(construct, true)
	# Iterate over all blocks:
	var block_mass
	var block_moment
	for block in construct.get_children():
		if !block is MeshInstance3D:
			continue
#		# Call the block's to_world to signal modules to enter world mode
#		block.to_world()
		# Add the block to CoM and moment of inertia
		block_mass = block.get_meta("mass")
		com = (com * mass + block.position * block_mass) / (mass + block_mass)
		block_moment = Vector3(
			block_mass * (block.position.y*block.position.y + block.position.z*block.position.z),
			block_mass * (block.position.x*block.position.x + block.position.z*block.position.z),
			block_mass * (block.position.x*block.position.x + block.position.y*block.position.y)
			)
		moment += block_moment
		mass += block_mass
		# Move the block's CollisionShape3D to the RigidBody preserving transform
		block.get_node("Area3D/CollisionShape3D").reparent(construct, true)
		# Free the Area3D
		block.get_node("Area3D").queue_free()
	# Correct the moment of inertia by parallel axis theorem
	moment -= Vector3(
		mass * (com.y*com.y + com.z*com.z),
		mass * (com.x*com.x + com.z*com.z),
		mass * (com.x*com.x + com.y*com.y)
		)
	construct.set_center_of_mass_mode(RigidBody3D.CENTER_OF_MASS_MODE_CUSTOM)
	construct.set_center_of_mass(com)
	construct.set_mass(mass)
	construct.set_inertia(moment)
	proto_construct.queue_free()
	
	return construct

static func to_editor(construct):
	
	pass
