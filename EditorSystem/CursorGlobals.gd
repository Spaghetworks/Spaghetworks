extends Node

# Holds cursor states.

signal selected_block_changed(block_name)

var selected_block_name

func change_selected_block(block_name):
	
	# change selected path
	selected_block_name = block_name
	
	# emit signal
	selected_block_changed.emit(selected_block_name)
