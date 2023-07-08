extends Node3D

var world
var hand #= preload("res://Blocks/Generic/BlockCube.tscn")
var place_area # collision shape should correspond to the collision shape of the object in hand
var delete_area # collision shape should always remain a simple 1-cube
				#TODO: _should_ it? why?
var cursor_mesh # mesh which previews the object in hand
var camera_focus # focus xform for the camera

var collider_dirty = true # signals to check obstruction on next frame

func _process(_delta):
	if(collider_dirty):
		# check obstruction to draw the cursor preview
		obstructed()

func _enter_tree():
	get_node("/root/CursorGlobals").selected_block_changed.connect(_on_selected_block_changed)
	world = get_parent().get_node("Construct_Root")
	place_area = $Place_Area
	delete_area = $Delete_Area
	cursor_mesh = $Cursor_Mesh
	camera_focus = $Camera_Focus

func _unhandled_input(event):
	if(event.is_action_pressed("EditorPlace")):
		if hand == null:
			print("no block selected")
			return
		if(place_area.has_overlapping_areas()):
			# Fail to place the thing
			print("placement obstructed")
		else:
			# Place the thing in hand
			print("placing item")
			var new_block = hand.duplicate(7)
			world.add_child(new_block)
			new_block.global_position = global_position
	
	elif(event.is_action_pressed("EditorDelete")):
		if(delete_area.has_overlapping_areas()):
			# Delete the thing under the cursor
			print("deleting item")
			delete_area.get_overlapping_areas()[0].get_parent().queue_free()
			collider_dirty = true
		else:
			print("nothing to delete")
	
	elif  (event.is_action_pressed("EditorCursorBack")
		|| event.is_action_pressed("EditorCursorForward")
		|| event.is_action_pressed("EditorCursorUp")
		|| event.is_action_pressed("EditorCursorDown")
		|| event.is_action_pressed("EditorCursorRight")
		|| event.is_action_pressed("EditorCursorLeft")):
		# Move the cursor
		var cursor_move = Vector3(0,0,0)
		if(event.is_action_pressed("EditorCursorBack")):
			cursor_move.z = 1
		if(event.is_action_pressed("EditorCursorForward")):
			cursor_move.z = -1
		if(event.is_action_pressed("EditorCursorUp")):
			cursor_move.y = 1
		if(event.is_action_pressed("EditorCursorDown")):
			cursor_move.y = -1
		if(event.is_action_pressed("EditorCursorRight")):
			cursor_move.x = 1
		if(event.is_action_pressed("EditorCursorLeft")):
			cursor_move.x = -1
		# Xform the cursor move direction per the camera's look direction
		cursor_move = cursor_move.rotated(Vector3.UP, round(camera_focus.rotation.y / (PI/2)) * PI/2 * sign(camera_focus.basis.y.dot(Vector3.UP))) 
		transform.origin += 0.1 * cursor_move

func obstructed():
	if(place_area.has_overlapping_areas()):
		cursor_mesh.get_surface_override_material(0).albedo_color = Color(1, .5, .5, .75)
	else:
		cursor_mesh.get_surface_override_material(0).albedo_color = Color(.1, .5, 1, .75)

func _on_place_area_area_entered(_area):
	obstructed()

func _on_place_area_area_exited(_area):
	obstructed()

func _on_selected_block_changed(block_name):
	print("block changed to " + block_name)
	hand = get_node("/root/BlockLoader").blocks[block_name]
#	print(hand.get_node("Area3D/CollisionShape3D"))
	place_area.get_child(0).set_shape(hand.get_node("Area3D/CollisionShape3D").get_shape())
	cursor_mesh.set_mesh(hand.get_mesh())
