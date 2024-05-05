extends TabContainer

var categories = {}

func _ready():
	# Build the UI tree
	if !get_node("/root/BlockLoader").loaded:
		print("Blocks not yet loaded!")
		return
	for block in get_node("/root/BlockLoader").blocks.values():
		if !categories.has(block.get_meta("category")):
			add_category(block.get_meta("category"))
		var button = Button.new()
		button.text = block.get_meta("display_name")
		button.focus_mode = Control.FOCUS_NONE
		button.button_down.connect(_on_block_selected.bind(block.get_meta("name")))
		categories[block.get_meta("category")].add_child(button)

func add_category(category_name):
	var category = Panel.new()
	category.add_child(HBoxContainer.new())
	category.name = category_name
	add_child(category)
	categories[category_name] = category.get_child(0)

func _on_block_selected(block_name):
	get_node("/root/CursorGlobals").change_selected_block(block_name)
