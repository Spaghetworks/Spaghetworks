extends Control

var statemachine
@onready var escape_menu = $EscapeMenu

func _ready():
	statemachine = get_node("/root/SceneStateMachine")

func _unhandled_key_input(event):
	if event.is_action_pressed("EscapeMenu"):
		escape_menu.visible = !escape_menu.visible
		if escape_menu.visible:
			Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
		else:
			Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func _on_to_editor_pressed():
	statemachine.switch_scene(statemachine.scenes.SCENE_EDITOR, {"editable_construct" : ConstructTranslator.to_editor(get_node("../Construct_Root"))})
