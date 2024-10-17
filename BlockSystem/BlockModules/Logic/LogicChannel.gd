extends Object

class_name LogicChannel

enum ChannelType {BOOL, FLOAT_RAW, FLOAT_NORM, SERIAL}
# BOOL: boolean, multiple inputs combined by OR
# FLOAT_RAW: unbounded float, multiple inputs combined by addition
# FLOAT_NORM: normalized float, bounded to -1..1, multiple inputs combined by mean
# SERIAL: serial bus, instantaneous arbitrary data packets, value is unused

signal value_updated

var type :ChannelType
var inputs = []
var outputs = []

var value

func _init(new_type):
	type = new_type

func add_input(input):
	if input.type == type:
		inputs.append(input)
		input.updated.connect(on_input_updated)
	else:
		print("Attempted to add input of incompatible type")

func remove_input(input):
	input.updated.disconnect(on_input_updated)
	inputs.erase(input)

func add_output(output):
	if output.type == type:
		outputs.append(output)
		value_updated.connect(output.on_value_updated)
	else:
		print("Attempted to add output of incompatible type")

func remove_output(output):
	value_updated.disconnect(output.on_value_updated)
	outputs.erase(output)

func on_input_updated(packet = null):
	match type:
		ChannelType.BOOL:
			value = false
			for input in inputs:
				if input.value:
					value = true
					break
			value_updated.emit(value)
		
		ChannelType.FLOAT_RAW:
			value = 0
			for input in inputs:
				value += input.value
			value_updated.emit(value)
		
		ChannelType.FLOAT_NORM:
			value = 0
			for input in inputs:
				value += input.value
			value /= inputs.size()
			value_updated.emit(value)
		
		ChannelType.SERIAL:
			value_updated.emit(packet)
	
#	if type == ChannelType.SERIAL:
#		print(packet)
#	else:
#		print(value)
