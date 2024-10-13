extends HBoxContainer

signal selected_number
signal selected_string

var button_strings

func _enter_tree():
	if get_child_count() > 1:
		return
	var button = get_child(0)
	for i in range(2,11):
		if i == 6:
			add_child(VSeparator.new())
		var new_button = button.duplicate(7)
		new_button.initialize(i)
		add_child(new_button)
	button_strings = Array()
	for i in range (0,10):
		button_strings.append("")

# The first button has number = 1
# The tenth button has number = 0
# button_strings is indexed by number, so the tenth button comes first in the array

func set_label(number, text, internal_text = ""):
	if internal_text != "":
		button_strings[number] = internal_text
	if number == 0:
		number = 10 # the tenth button has number = 0
	if number <= 5:
		number -= 1 # skip the separator
	get_child(number).text = text

func _on_selected(number):
	print("selected " + str(number))
	selected_number.emit(number)
	if button_strings[number] != "":
		print(button_strings[number])
		selected_string.emit(button_strings[number])


func _on_updated(number, display, internal):
	set_label(number, display, internal)
