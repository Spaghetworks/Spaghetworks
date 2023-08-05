extends Node3D

func recieve(payload): # Recieves extra information immediately after a scene change
	if payload.has("construct"):
		spawn_construct(payload["construct"])

func spawn_construct(construct):
	print("Spawning construct")
	construct.reparent(self, true)
