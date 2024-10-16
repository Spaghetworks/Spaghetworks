extends InteractionArea

@export var state :bool = false

func interact():
	state = !state
	interacted.emit(state)
