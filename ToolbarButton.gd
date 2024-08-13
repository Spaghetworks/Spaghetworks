extends Label

signal selected
signal updated

var number = 1

func initialize(num):
	number = num % 10
	var event = InputEventKey.new()
	if num > 5:
		num -= 5
		event.shift_pressed = true
	event.set_keycode(OS.find_keycode_from_string(str(num)))
	
	var shortcut = Shortcut.new()
	shortcut.events.append(event)
	get_child(0).shortcut = shortcut


func _on_button_pressed():
	selected.emit(number)

# Droppable data must be a dictionary with a valid "type" key

func _can_drop_data(at_position, data):
	print("drop queried")
	if data is Dictionary && data.has("type"):
		if data["type"] == "block":
			return true
	return false

func _drop_data(at_position, data):
	match data["type"]:
		"block":
			updated.emit(number, data["display_name"], data["name"])
