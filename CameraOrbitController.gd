extends Node3D

var mode = 0 # 0 is unlocked cursor, 1 is locked cursor, and 2 is held-unlocked.
var lock_held = 0
var camera

var hold_time = .25
var sensitivity = .0125
var zoom_step = 1.1

func _enter_tree():
	camera = get_child(0)

func _process(delta):
	if(mode == 1 && Input.is_action_pressed("EditorCameraLock")):
		# count time the lock key is pressed and transition to held-unlocked mode
		lock_held += delta
		if(lock_held > hold_time):
			lock_held = 0
			set_mode(2)

func _unhandled_input(event):
	if(event is InputEventMouseMotion):
		if((mode == 0 && Input.is_action_pressed("EditorCameraOrbit"))||(mode == 1 && !Input.is_action_pressed("EditorCameraLock"))):
			# do some orbiting
			rotation.x -= event.relative.y * sensitivity
			rotation.y -= event.relative.x * sensitivity * sign(basis.y.dot(Vector3.UP))
	
	if(event is InputEventKey):
		# mode state machine
		if event.is_action_released("EditorCameraLock"):
			match mode:
				0: # unlocked
						set_mode(1)
				1: # locked
						set_mode(0)
				2: # held-unlocked
						set_mode(1)
	if(event is InputEventMouseButton):
		if event.is_action_pressed("EditorCameraZoomIn"):
			# zoom in
			camera.transform.origin.z /= zoom_step
		if event.is_action_pressed("EditorCameraZoomOut"):
			# zoom out
			camera.transform.origin.z *= zoom_step

func set_mode(newMode):
	match newMode:
		0: # unlocked
			Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
		1: # locked
			Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
		2: # held-unlocked
			Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
	mode = newMode
