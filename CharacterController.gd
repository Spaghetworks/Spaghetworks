extends CharacterBody3D


const SPEED = 5.0
const JUMP_VELOCITY = 4.5
var sensitivity = .002
#var spring = 1
#var crouching = false
#var head_set_height = .6
@onready var camera = $HeadShape/Camera3D
@onready var joint = $SliderJoint3D
@onready var head_shape = $HeadShape
@onready var head_driver = $HeadDriver

# Get the gravity from the project settings to be synced with RigidBody nodes.
var gravity = ProjectSettings.get_setting("physics/3d/default_gravity")

func _ready():
	Input.mouse_mode = Input.MOUSE_MODE_CAPTURED
#	head_driver.set_constant_force(Vector3.UP*10)

func _physics_process(delta):
	# Add the gravity.
	if not is_on_floor():
		velocity.y -= gravity * delta

	# Handle Jump.
	if Input.is_action_just_pressed("CharacterJump") and is_on_floor():
		velocity.y = JUMP_VELOCITY
	
#	# Handle crouch
#	if Input.is_action_just_pressed("CharacterCrouch"):
#		crouching = true
#		head_set_height = -.3
#	if Input.is_action_just_released("CharacterCrouch"):
#		crouching = false
#		head_set_height = .6
#
#	head_driver.apply_force(Vector3(0,(head_driver.position.y - head_set_height)*spring,0))
#	head_shape.position.y = head_driver.position.y
	
	# Get the input direction and handle the movement/deceleration.
	var input_dir = Input.get_vector("CharacterMoveLeft", "CharacterMoveRight", "CharacterMoveForward", "CharacterMoveBack")
	var direction = (transform.basis * Vector3(input_dir.x, 0, input_dir.y)).normalized()
	if direction:
		velocity.x = direction.x * SPEED
		velocity.z = direction.z * SPEED
	else:
		velocity.x = move_toward(velocity.x, 0, SPEED)
		velocity.z = move_toward(velocity.z, 0, SPEED)

	move_and_slide()

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		rotation.y -= event.relative.x * sensitivity
		camera.rotation.x -= event.relative.y * sensitivity
