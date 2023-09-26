extends Node

# Aggregates global statistics (e.g. block counts)

signal statistic_updated

var stat_names = PackedStringArray(["shaft_count"])

var statistics

func _enter_tree():
	statistics = {}
	for stat in stat_names:
		statistics[stat] = 0

func update_statistics(delta_values):
	for delta in delta_values:
		if statistics.has(delta):
			statistics[delta] += delta_values[delta]
		else:
			print("attempted to update nonexistent statistic ", delta)
	statistic_updated.emit()

func overwrite_statistics(new_values):
	for overwrite in new_values:
		if statistics.has(overwrite):
			statistics[overwrite] = new_values[overwrite]
		else:
			print("attempted to overwrite nonexistent statistic ", overwrite)
	statistic_updated.emit()
