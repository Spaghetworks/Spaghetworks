extends Node

var channels :Dictionary = {}

func initialize(params):
	name = params["interface_name"]

func add_input(channel_number, input):
	if !channels.has(channel_number): #channel doesn't exist yet, make a new channel first
		channels[channel_number] = LogicChannel.new(input.type)
	if !channels[channel_number].inputs.has(input):
		channels[channel_number].add_input(input)

func remove_input(channel_number, input):
	channels[channel_number].remove_input(input)

func add_output(channel_number, output):
	if !channels.has(channel_number): #channel doesn't exist yet, make a new channel first
		channels[channel_number] = LogicChannel.new(output.type)
	if !channels[channel_number].outputs.has(output):
		channels[channel_number].add_output(output)

func remove_output(channel_number, output):
	channels[channel_number].remove_output(output)
