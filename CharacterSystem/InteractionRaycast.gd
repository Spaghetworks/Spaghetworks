extends RayCast3D

func _unhandled_input(event):
	if event.is_action("CharacterInteract") && event.is_pressed(	):
		if !is_colliding():
			return
		var collided = get_collider()
		if collided is Area3D:
			print("Interaction hit ", collided)
			collided.interact()
