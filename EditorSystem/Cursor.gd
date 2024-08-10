extends Node3D

var world
var hand # block(s) to be placed
var place_area # collision shape should correspond to the collision shape of the object in hand
var delete_area # collision shape should always remain a simple 1-cube
				#TODO: _should_ it? why?
var cursor_mesh # mesh which previews the object in hand
var camera_focus # focus xform for the camera
var editor_ui # root UI object

var collider_dirty = true # check obstruction on next frame
var rotate_mode = false # should directional inputs be translation or rotation?
var interacting # block which interaction menu is active
var automove = true # advance the cursor by the size of the thing being placed
var place_aabb # dimensions of the object in hand
var selecting = false # select mode
var selection_aabb # selected AABB
var selection_cube # renders selection volume
var selection_dirty = 0 # check selection area next frame
var selected_blocks = Array() # list of blocks in the selection area
var clipboard = Node3D.new() # copied/cut group of blocks, which can be sent to the hand
var pasting = false # whether the thing to be placed is pasting from the clipboard, as opposed to the block in hand

func _process(_delta):
	if(collider_dirty):
		# check obstruction to draw the cursor preview
		obstructed()

func _physics_process(_delta):
	if selection_dirty == 1:
		print(selection_cube.get_node("Area3D").get_overlapping_areas().size())
		selected_blocks = Array()
		for block_area in selection_cube.get_node("Area3D").get_overlapping_areas():
			selected_blocks.append(block_area.get_parent())
	if selection_dirty > 0:
		selection_dirty -= 1

func _enter_tree():
	get_node("/root/CursorGlobals").selected_block_changed.connect(_on_selected_block_changed)
	world = get_parent().get_node("Construct_Root")
	editor_ui = get_node("../EditorUI")
	place_area = $Place_Area
	delete_area = $Delete_Area
	cursor_mesh = $Cursor_Mesh
	camera_focus = $Camera_Focus
	selection_cube = $"../Selection_Cube"

func _unhandled_input(event):
	if(Input.is_action_just_pressed("EditorRotateMode")):
		rotate_mode = true
		print("Rotation")
	elif(Input.is_action_just_released("EditorRotateMode")):
		rotate_mode = false
		print("Translation")
	
	if(event.is_action_pressed("EditorPlace")):
		if selecting: #do selection stuff
			if selection_aabb == null:
				selection_aabb = AABB(position, Vector3.ZERO)
			else:
				selection_aabb = selection_aabb.expand(position)
			update_selection_cube()
		else:
			if !pasting && hand == null:
				print("no block selected")
				return
			if(place_area.has_overlapping_areas()):
				# Fail to place the thing
				print("placement obstructed")
			else:
				if pasting:
					# Place the thing in the clipboard
					print("pasting from clipboard")
					for block in clipboard.get_children():
						var new_block = block.duplicate(7)
						world.add_child(new_block)
						new_block.global_transform = cursor_mesh.global_transform
						new_block.transform *= block.transform
						new_block.owner = world
				else:
					# Place the thing in hand
					print("placing item")
					var new_block = hand.duplicate(7)
					world.add_child(new_block)
					new_block.global_transform = cursor_mesh.global_transform
					new_block.owner = world
					
					if automove:
						var move = Vector3()
						move[abs(camera_focus.global_transform.basis.z).max_axis_index()] = -sign(camera_focus.global_transform.basis.z[abs(camera_focus.global_transform.basis.z).max_axis_index()])
			#			move *= camera_focus.global_transform * (place_aabb.size)
						move *= (place_area.transform * place_aabb).size.snapped(Vector3(0.1,0.1,0.1))
						transform.origin += move
	
	elif(event.is_action_pressed("EditorDelete")):
		if(delete_area.has_overlapping_areas()):
			# Delete the thing under the cursor
			print("deleting item")
			var block = delete_area.get_overlapping_areas()[0].get_parent()
			collider_dirty = true
			hide_interaction()
			
			var move = Vector3()
			move[abs(camera_focus.global_transform.basis.z).max_axis_index()] = -sign(camera_focus.global_transform.basis.z[abs(camera_focus.global_transform.basis.z).max_axis_index()])
#			move *= camera_focus.global_transform * (place_aabb.size)
			move *= (place_area.transform * block.aabb).size.snapped(Vector3(0.1,0.1,0.1))
			print(move)
			transform.origin -= move
			block.queue_free()
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
			var control_rotate = Vector3(
				Input.get_axis("EditorCursorForward","EditorCursorBack"),
				Input.get_axis("EditorCursorLeft","EditorCursorRight"),
				Input.get_axis("EditorCursorDown","EditorCursorUp")
			)
			# Xform the rotation per the camera's look direction
			control_rotate = control_rotate.rotated(Vector3.UP, round(camera_focus.rotation.y / (PI/2)) * PI/2 * sign(camera_focus.basis.y.dot(Vector3.UP)))
			cursor_mesh.transform = cursor_mesh.transform.rotated(control_rotate, PI/2)
			place_area.transform = place_area.transform.rotated(control_rotate, PI/2)
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

func update_selection_cube():
	if !selecting || selection_aabb == null:
		selection_cube.visible = false
	else:
		selection_cube.visible = true
		selection_cube.size = selection_aabb.size + Vector3.ONE * 0.1
		selection_cube.position = selection_aabb.get_center()
		selection_cube.get_node("Area3D/CollisionShape3D").shape.size = selection_cube.size - Vector3.ONE * 0.01
		selection_dirty = 2

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

func _on_selected_block_changed(block_name, unset_ui):
	print("block changed to " + block_name)
	hand = get_node("/root/BlockLoader").blocks[block_name]
	for node in place_area.get_children():
		node.queue_free()
	for collision in hand.get_all_collisions():
		place_area.add_child(collision.duplicate())
	for node in cursor_mesh.get_children():
		node.queue_free()
	for mesh in hand.get_all_meshes():
		var instance = MeshInstance3D.new()
		instance.set_mesh(mesh[0])
		var mat = StandardMaterial3D.new()
		mat.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
		instance.set_surface_override_material(0, mat)
		cursor_mesh.add_child(instance)
		instance.position = mesh[1]
	place_aabb = hand.aabb
	pasting = false
	if unset_ui:
		editor_ui.force_UI("select", false)


func _on_automove_toggled(state):
	automove = state


func _on_select_toggled(state):
	selecting = state
	selection_aabb = null
	update_selection_cube()
	if state:
		_on_selected_block_changed("block_cube", false)

func _on_copy_requested():
	for child in clipboard.get_children():
		child.queue_free()
	for block in selected_blocks:
		var block_copy = block.duplicate(7)
		clipboard.add_child(block_copy)
		block_copy.position = block.global_position - (selection_cube.global_position * 10).floor() / 10

func _on_cut_requested():
	_on_copy_requested()
	for block in selected_blocks:
		block.queue_free()

func _on_paste_requested():
	if clipboard.get_child_count() > 0:
		# clear hand
		for node in place_area.get_children():
			node.queue_free()
		for node in cursor_mesh.get_children():
			node.queue_free()
		# replace with clipboard
		place_aabb = clipboard.get_child(0).aabb * clipboard.get_child(0).transform
		for block in clipboard.get_children():
			for collision in block.get_all_collisions():
				var dupe = collision.duplicate()
				place_area.add_child(dupe)
				dupe.transform *= block.transform
			for mesh in block.get_all_meshes():
				var instance = MeshInstance3D.new()
				instance.set_mesh(mesh[0])
				var mat = StandardMaterial3D.new()
				mat.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA
				instance.set_surface_override_material(0, mat)
				cursor_mesh.add_child(instance)
				instance.position = mesh[1]
				instance.transform *= block.transform
			place_aabb = place_aabb.merge(block.aabb * block.transform)
			print(place_aabb)
		pasting = true
		editor_ui.force_UI("select", false)

func _on_save_or_load_requested(path, mode):
	# Actually save or load the construct
	match mode:
		FileDialog.FILE_MODE_SAVE_FILE:
			if !path.ends_with(".json"):
				path += ".json"
			# Save the file
			print("Saving to " + path)
			var save_file = FileAccess.open(path, FileAccess.WRITE)
			var save_data = []
			for child in world.get_children():
				if !child is MeshInstance3D:
					# Skip things that aren't blocks (construct modules)
					continue
				var block_data = {}
				block_data["transform"] = serialize_transform(child.transform)
				block_data["block_name"] = child.get_meta("name")
				block_data["modules"] = child.serialize_modules()
				save_data.append(block_data)
			var save_string = JSON.stringify(save_data," ")
			save_file.store_line(save_string)
	#		var construct = PackedScene.new()
	#		var error = construct.pack(world)
	#		if error == OK:
	#			error = ResourceSaver.save(construct, path)
	#			if error == OK:
	#				print("Save done")
	#			else:
	#				print("Save failed, code " + str(error))
	#		else:
	#			print("Pack failed, code " + str(error))
		FileDialog.FILE_MODE_OPEN_FILE:
			# Load the file
			print("Loading from " + path)
			var save_file = FileAccess.open(path, FileAccess.READ)
			for child in world.get_children():
				# Delete all the existing blocks
				if child is MeshInstance3D:
					child.queue_free()
			var save_data = JSON.parse_string(save_file.get_as_text())
			for block_data in save_data:
				# Place each block
				var block = get_node("/root/BlockLoader").blocks[block_data["block_name"]].duplicate(7)
				world.add_child(block)
				block.transform = parse_transform(block_data["transform"])
				block.deserialize_modules(block_data["modules"])
			
#			var new_construct = load(path)
#			world.name = "Old_Construct"
#			world.queue_free()
#			world = new_construct.instantiate()
#			get_parent().add_child(world)
#			world.name = "Construct_Root"

func serialize_transform(xform):
	return {
		"x" : vector_to_array(xform.basis.x),
		"y" : vector_to_array(xform.basis.y),
		"z" : vector_to_array(xform.basis.z),
		"origin" : vector_to_array(xform.origin)
	}

func vector_to_array(vec):
	return [vec.x, vec.y, vec.z]

func array_to_vector(arr):
	return Vector3(arr[0], arr[1], arr[2])

func parse_transform(data):
	return Transform3D(array_to_vector(data["x"]), array_to_vector(data["y"]), array_to_vector(data["z"]), array_to_vector(data["origin"]))








