extends TabContainer

var categories = {}
var draggable = preload("res://Draggable.gd")

func _ready():
	# Build the UI tree
	if !get_node("/root/BlockLoader").loaded:
		print("Blocks not yet loaded!")
		return
	for block in get_node("/root/BlockLoader").blocks.values():
		if !categories.has(block.get_meta("category")):
			add_category(block.get_meta("category"))
		var button = draggable.new()
		button.payload = {
			"type":"block",
			"name":block.get_meta("name"),
			"display_name":block.get_meta("display_name")
		}
		button.text = block.get_meta("display_name")
		button.focus_mode = Control.FOCUS_NONE
		button.button_up.connect(_on_block_selected.bind(block.get_meta("name")))
		categories[block.get_meta("category")].add_child(button)

func add_category(category_name):
	var category = Panel.new()
	category.add_child(HFlowContainer.new())
	category.name = category_name
	category.get_child(0).custom_minimum_size.x = 1000
	add_child(category)
	categories[category_name] = category.get_child(0)

func _on_block_selected(block_name):
	get_node("/root/CursorGlobals").change_selected_block(block_name, true)
