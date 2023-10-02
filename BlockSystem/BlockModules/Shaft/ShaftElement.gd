extends Node

signal state_updated

@export var moment:float
var socket_a
var socket_b
var shaft_system
var body
var offset = 0

@export var socket_names:Array
@export var axis:Vector3

var connected_element_a
var connected_element_b
var springs = []
var constraints = []

var rebuild_this_frame

func initialize(params):
	moment = params["moment"]
	socket_names = params["sockets"]
	name = params["element_name"]
	axis = Vector3(params["axis"][0],params["axis"][1],params["axis"][2])

func _enter_tree():
	rebuild_this_frame = true
	if shaft_system:
		return
	shaft_system = get_node("../../ShaftSystem")
	
	if socket_names.size() > 0:
		# bind to socket_a
		socket_a = get_node("../" + socket_names[0])
		socket_a.element = self
		socket_a.socket_connected.connect(on_socket_a_connected)
		socket_a.socket_disconnected.connect(on_socket_a_disconnected)
	
	if socket_names.size() > 1:
		# bind to socket_b
		socket_b = get_node("../" + socket_names[1])
		socket_b.element = self
		socket_b.socket_connected.connect(on_socket_b_connected)
		socket_b.socket_disconnected.connect(on_socket_b_disconnected)

func _physics_process(_delta):
	if rebuild_this_frame:
		# Submit a rebuild request
		shaft_system.request_rebuild(self)
		rebuild_this_frame = false

func attach(shaft_body):
	if body:
		# Disconnect signals
		body.state_updated.disconnect(on_state_updated)
	body = shaft_body
	# Connect signals
	body.state_updated.connect(on_state_updated)

func add_spring(spring):
	springs.append(spring)
	print("spring!" + str(springs.size()))

func add_constraint(constraint):
	constraints.append(constraint)

func get_position():
	return body.position * (get_parent().global_transform.basis * axis).dot(get_node("../..").global_transform.basis * body.principal_axis) + offset

func get_velocity():
	return body.velocity * (get_parent().global_transform.basis * axis).dot(get_node("../..").global_transform.basis * body.principal_axis)

func on_state_updated(pos, vel):
#	print((get_parent().global_transform * axis).dot(body.principal_axis))
	state_updated.emit(get_position(), get_velocity())

func on_socket_a_connected(_local, remote):
	connected_element_a = remote.element
	rebuild_this_frame = true
	print("Connected " + self.to_string() + " to " + connected_element_a.to_string())

func on_socket_a_disconnected(_local, remote):
	print("Disconnected " + self.to_string() + " from " + remote.to_string())
	connected_element_a = null
	rebuild_this_frame = true

func on_socket_b_connected(_local, remote):
	connected_element_b = remote.element
	rebuild_this_frame = true
	print("Connected " + self.to_string() + " to " + connected_element_b.to_string())

func on_socket_b_disconnected(_local, remote):
	print("Disconnected " + self.to_string() + " from " + remote.to_string())
	connected_element_b = null
	rebuild_this_frame = true
