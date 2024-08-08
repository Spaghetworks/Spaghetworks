extends ColorRect

var dirty

var color_true = Color("0046AA")
var color_false = Color("D22300")

func _ready():
	dirty = true

func _process(delta):
	if dirty:
		if get_parent().button_pressed:
			color = color_true
		else:
			color = color_false


func _on_button_toggled(button_pressed):
	print(button_pressed)
	if button_pressed:
		color = color_true
	else:
		color = color_false
