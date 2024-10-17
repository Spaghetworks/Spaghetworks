extends InteractionArea

@export var state :bool = false

func _enter_tree():
	interacted.emit(state)

func interact():
	state = !state
	interacted.emit(state)
