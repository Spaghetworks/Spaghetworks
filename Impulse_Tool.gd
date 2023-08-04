extends Node3D

var first_frame = true
var raycast
@onready var magnitude_label = $Label

var magnitude = 1.0

func _process(_delta):
	if !first_frame:
		return
	magnitude_label.text = str(magnitude)
	raycast = get_node("../..").get_camera().get_child(0)
	first_frame = false

func action_primary(pressed):
	if raycast == null:
		return
	
#	if pressed:
#		print("primary pressed")
#	else:
#		print("primary released")
	
	if !pressed:
		# Raycast
		if !raycast.is_colliding():
			return
		var target = raycast.get_collider()
		# Cannot apply impulse to a CharacterBody or StaticBody
		if target is CharacterBody3D || target is StaticBody3D:
			return
		
		var target_point = raycast.get_collision_point() - target.global_position
		var target_vector = (raycast.to_global(raycast.get_target_position()) - raycast.global_position).normalized()
		# Apply impulse
		target.apply_impulse((1 * target_vector * magnitude), target_point)

func action_secondary(pressed):
	if raycast == null:
		return
	
	if pressed:
		print("secondary pressed")
	else:
		print("secondary released")

func action_tertiary(pressed):
	if pressed:
		print("tertiary pressed")
	else:
		print("tertiary released")

func action_up():
	print("magnitude up")
	magnitude *= 10
	magnitude_label.text = str(magnitude)

func action_down():
	print("magnitude down")
	magnitude /= 10
	magnitude_label.text = str(magnitude)
