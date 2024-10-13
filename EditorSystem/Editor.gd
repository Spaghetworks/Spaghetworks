extends Node3D

signal construct_root_replaced

@onready var construct_root = $Construct_Root

func recieve(payload): # Recieves extra information immediately after scene change
	if payload.has("editable_construct"):
		var root_xform = construct_root.transform
		construct_root.name = "Construct_Root_Old"
		construct_root.queue_free()
		construct_root = payload["editable_construct"]
		construct_root.name = "Construct_Root"
		add_child(construct_root)
		construct_root.transform = root_xform
		construct_root_replaced.emit(construct_root)
