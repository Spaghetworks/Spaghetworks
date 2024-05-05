extends VBoxContainer

@export var control_keys:Dictionary = {
	"EditorCameraLock":"Lock/unlock cursor",
	"EditorCameraOrbit":"Orbit camera",
	"EditorPlace":"Place block",
	"EditorDelete":"Delete block",
	"EditorRotateMode":"Hold to rotate",
	"EditorInteract":"Configure block"
	}

func _ready():
	for key in control_keys:
		var label = Label.new()
		label.text = control_keys[key] + ": " + InputMap.action_get_events(key)[0].as_text()
		add_child(label)

