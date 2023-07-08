extends Label

@export var base_string = ""
@export var stat_name:String

func _enter_tree():
	get_node("/root/BlockStatistics").statistic_updated.connect(on_updated)
	on_updated()

func on_updated():
	text = base_string + str(BlockStatistics.statistics.get(stat_name, "INVALID"))
