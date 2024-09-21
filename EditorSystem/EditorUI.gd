extends Control

signal automove_toggled
signal select_toggled
signal copy_requested
signal cut_requested
signal paste_requested

var statemachine
@onready var interaction = $VBoxContainer2/InteractionMenu
@onready var interaction_list = $VBoxContainer2/InteractionMenu/VBoxContainer/VBoxContainer
@onready var file_dialog = $FileDialog
@onready var palette = $Palette
const construct_dir = "user://Constructs"
var file_dialog_init = false

func _ready():
	statemachine = get_node("/root/SceneStateMachine")

func _unhandled_input(event):
	if Input.is_action_just_pressed("EditorPalette"):
		palette.visible = !palette.visible

func _on_reload_pressed(): # Reload the editor by invoking a scene state transition
	statemachine.switch_scene(statemachine.scenes.SCENE_EDITOR)


func _on_to_sim_pressed():
	statemachine.switch_scene(statemachine.scenes.SCENE_SIMULATION, {"construct" : ConstructTranslator.to_world(get_node("../Construct_Root"))})

func show_interaction():
	interaction.visible = true

func hide_interaction():
	interaction.visible = false

func add_interaction(ui):
	if interaction_list.get_children().size() != 0:
		interaction_list.add_child(HSeparator.new())
	interaction_list.add_child(ui)

func clear_interaction():
	print("clearing interaction menu")
	for node in interaction_list.get_children():
		interaction_list.remove_child(node)

func _on_automove_toggled(state):
	automove_toggled.emit(state)

func _on_select_toggled(state):
	select_toggled.emit(state)
	$VBoxContainer2/Panel/HFlowContainer/Copy .disabled = !state
	$VBoxContainer2/Panel/HFlowContainer/Cut .disabled = !state


func _on_copy_pressed():
	copy_requested.emit()

func _on_cut_pressed():
	cut_requested.emit()

func _on_paste_pressed():
	paste_requested.emit()

func force_UI(param, state):
	match param:
		"select":
			$VBoxContainer2/Panel/HFlowContainer/Select .button_pressed = state
			$VBoxContainer2/Panel/HFlowContainer/Select .toggled.emit(state)
			pass
		"automove":
			pass
	pass

func init_file_dialog():
	# Check for save dir exists
	if !DirAccess.dir_exists_absolute(construct_dir):
		DirAccess.make_dir_absolute(construct_dir)
	file_dialog.root_subfolder = construct_dir

func _on_save_pressed():
	if !file_dialog_init:
		init_file_dialog()
	# Open dialog in save mode
	file_dialog.file_mode = file_dialog.FILE_MODE_SAVE_FILE
	file_dialog.visible = true

func _on_load_pressed():
	if !file_dialog_init:
		init_file_dialog()
	# Open dialog in load mode
	file_dialog.file_mode = file_dialog.FILE_MODE_OPEN_FILE
	file_dialog.visible = true

func _on_file_dialog_file_selected(path):
	var construct_root = get_node("../Construct_Root")
	match file_dialog.file_mode:
		FileDialog.FILE_MODE_OPEN_FILE:
			# Load the file
			print("Loading from " + path)
			var save_file = FileAccess.open(path, FileAccess.READ)
			var save_data = JSON.parse_string(save_file.get_as_text())
			var proto_construct = get_node("/root/ConstructTranslator").from_file(save_data)
			add_child(proto_construct)
			for child in proto_construct.get_children():
				child.reparent(construct_root, false)
			proto_construct.queue_free()
		FileDialog.FILE_MODE_SAVE_FILE:
			if !path.ends_with(".json"):
				path += ".json"
			# Save the file
			print("Saving to " + path)
			var save_file = FileAccess.open(path, FileAccess.WRITE)
			save_file.store_line(get_node("/root/ConstructTranslator").to_file(construct_root))


func _on_toolbar_selected_string(block_name):
	get_node("/root/CursorGlobals").change_selected_block(block_name, true)
