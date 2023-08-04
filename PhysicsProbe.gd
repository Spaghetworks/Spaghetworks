extends RigidBody3D

var frame = 0

# Called when the node enters the scene tree for the first time.
func _physics_process(_delta):
	if frame > 2:
		return
	if frame == 1:
		print("test cuboid inertia:")
		print(PhysicsServer3D.body_get_direct_state(get_rid()).inverse_inertia.inverse())
	frame += 1
