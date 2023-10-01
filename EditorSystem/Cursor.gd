extends Node3D

var world
var hand #= preload("res://Blocks/Generic/BlockCube.tscn")
var place_area # collision shape should correspond to the collision shape of the object in hand
var delete_area # collision shape should always remain a simple 1-cube
				#TODO: _should_ it? why?
var cursor_mesh # mesh which previews the object in hand
var camera_focus # focus xform for the camera
var editor_ui # root UI object

var collider_dirty = true # signals to check obstruction on next frame
var rotate_mode = false # should directional inputs be translation or rotation?
var interacting # block which interaction menu is active

func _process(_delta):
	if(collider_dirty):
		# check obstruction to draw the cursor preview
		obstructed()

func _enter_tree():
	get_node("/root/CursorGlobals").selected_block_changed.connect(_on_selected_block_changed)
	world = get_parent().get_node("Construct_Root")
	editor_ui = get_node("../EditorUI")
	place_area = $Place_Area
	delete_area = $Delete_Area
	cursor_mesh = $Cursor_Mesh
	camera_focus = $Camera_Focus

func _unhandled_input(event):
	if(Input.is_action_just_pressed("EditorRotateMode")):
		rotate_mode = true
		print("Rotation")
	elif(Input.is_action_just_released("EditorRotateMode")):
		rotate_mode = false
		print("Translation")
	
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
			new_block.global_transform = cursor_mesh.global_transform
	
	elif(event.is_action_pressed("EditorDelete")):
		if(delete_area.has_overlapping_areas()):
			# Delete the thing under the cursor
			print("deleting item")
			delete_area.get_overlapping_areas()[0].get_parent().queue_free()
			collider_dirty = true
			hide_interaction()
		else:
			print("nothing to delete")
	
	elif (event.is_action_pressed("EditorInteract")):
		if delete_area.has_overlapping_areas():
		# Open and attach the interaction window
			interacting = delete_area.get_overlapping_areas()[0].get_parent()
			interacting.ui_provided.connect(_on_inter_ui_recieved)
			editor_ui.clear_interaction()
			editor_ui.show_interaction()
			interacting.request_ui()
		else:
			hide_interaction()
	
	elif  (event.is_action_pressed("EditorCursorBack")
		|| event.is_action_pressed("EditorCursorForward")
		|| event.is_action_pressed("EditorCursorUp")
		|| event.is_action_pressed("EditorCursorDown")
		|| event.is_action_pressed("EditorCursorRight")
		|| event.is_action_pressed("EditorCursorLeft")):
		if(rotate_mode):
			# Rotate the block in hand
			var rotate = Vector3(
				Input.get_axis("EditorCursorForward","EditorCursorBack"),
				Input.get_axis("EditorCursorLeft","EditorCursorRight"),
				Input.get_axis("EditorCursorDown","EditorCursorUp")
			)
			# Xform the rotation per the camera's look direction
			rotate = rotate.rotated(Vector3.UP, round(camera_focus.rotation.y / (PI/2)) * PI/2 * sign(camera_focus.basis.y.dot(Vector3.UP)))
			cursor_mesh.transform = cursor_mesh.transform.rotated_local(rotate, PI/2)
			place_area.transform = place_area.transform.rotated_local(rotate, PI/2)
		else:
			# Move the cursor
			var cursor_move = Vector3(
				Input.get_axis("EditorCursorLeft","EditorCursorRight"),
				Input.get_axis("EditorCursorDown","EditorCursorUp"),
				Input.get_axis("EditorCursorForward","EditorCursorBack")
			)
			# Xform the cursor move direction per the camera's look direction
			cursor_move = cursor_move.rotated(Vector3.UP, round(camera_focus.rotation.y / (PI/2)) * PI/2 * sign(camera_focus.basis.y.dot(Vector3.UP))) 
			transform.origin += 0.1 * cursor_move
			
			# Detach and close the interaction menu
			hide_interaction()

func hide_interaction():
	if interacting:
		interacting.ui_provided.disconnect(_on_inter_ui_recieved)
		editor_ui.hide_interaction()
		interacting = null
	pass

func _on_inter_ui_recieved(ui):
	editor_ui.add_interaction(ui)

func obstructed():
	if(place_area.has_overlapping_areas()):
		for mesh in cursor_mesh.get_children():
			mesh.get_surface_override_material(0).albedo_color = Color(1, .5, .5, .75)
	else:
		for mesh in cursor_mesh.get_children():
			mesh.get_surface_override_material(0).albedo_color = Color(.1, .5, 1, .75)

func _on_place_area_area_entered(_area):
	obstructed()

func _on_place_area_area_exited(_area):
	obstructed()

func _on_selected_block_changed(block_name):
	print("block changed to " + block_name)
	hand = get_node("/root/BlockLoader").blocks[block_name]
#	print(hand.get_node("Area3D/CollisionShape3D"))
	place_area.get_child(0).set_shape(hand.get_node("Area3D/CollisionShape3D").get_shape())
	for node in cursor_mesh.get_children():
		node.queue_free()
	for mesh in hand.get_all_meshes():
		var instance = MeshInstance3D.new()
		instance.set_mesh(mesh)
		var mat = StandardMaterial3D.new()
		mat.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
		instance.set_surface_override_material(0, mat)
		cursor_mesh.add_child(instance)
#	cursor_mesh.set_mesh(hand.get_all_meshes())
