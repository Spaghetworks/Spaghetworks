extends Node3D


func recieve(payload): # Recieves extra information immediately after scene change
	if payload.has("editable_construct"):
		var construct_root = $Construct_Root
		var proto_construct = payload["editable_construct"]
		add_child(proto_construct)
		for child in payload["editable_construct"].get_children():
			child.reparent(construct_root, false)
		proto_construct.queue_free()
		pass
