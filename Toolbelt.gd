extends Node3D

var equipped_tool

func _ready():
	equipped_tool = get_child(0)
	print("equipped tool: " + equipped_tool.name)

func _process(_delta):
	if equipped_tool == null:
		return
	
	if Input.is_action_just_pressed("CharacterToolPrimary"):
		equipped_tool.action_primary(true)
	if Input.is_action_just_released("CharacterToolPrimary"):
		equipped_tool.action_primary(false)
	
	if Input.is_action_just_pressed("CharacterToolSecondary"):
		equipped_tool.action_secondary(true)
	if Input.is_action_just_released("CharacterToolSecondary"):
		equipped_tool.action_secondary(false)
	
	if Input.is_action_just_pressed("CharacterToolTertiary"):
		equipped_tool.action_tertiary(true)
	if Input.is_action_just_released("CharacterToolTertiary"):
		equipped_tool.action_tertiary(false)
	
	if Input.is_action_just_pressed("CharacterToolUp"):
		equipped_tool.action_up()
	
	if Input.is_action_just_pressed("CharacterToolDown"):
		equipped_tool.action_down()
