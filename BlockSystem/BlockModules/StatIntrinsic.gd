extends Node

@export var stat_name:String
@export var stat_value:int

func initialize(params):
	stat_name = params["stat_name"]
	stat_value = params["stat_value"]

func _enter_tree():
	BlockStatistics.update_statistics({stat_name:stat_value})

func _exit_tree():
	BlockStatistics.update_statistics({stat_name:-stat_value})
