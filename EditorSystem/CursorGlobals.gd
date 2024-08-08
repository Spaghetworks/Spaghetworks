extends Node

# Holds cursor states.

signal selected_block_changed(block_name, unset_ui)

var selected_block_name

func change_selected_block(block_name, unset_ui):
	# change selected path
	selected_block_name = block_name
	
	# emit signal
	selected_block_changed.emit(selected_block_name, unset_ui)
