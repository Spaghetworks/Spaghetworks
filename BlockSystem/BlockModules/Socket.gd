extends Area3D

signal socket_connected
signal socket_disconnected

var socket_type # Only sockets with matching types will connect
var exact_orientation # If true, only sockets with matching global rotations will connect
var connected # True if the socket is connected
var connected_to # The socket to which this socket is connected, or else null
var element # The system element to which this socket belongs. Used for graph traversal.

func _ready():
	area_entered.connect(on_area_enter)
	area_exited.connect(on_area_exit)

func initialize(params):
	set_collision_mask(1<<31)
	set_collision_layer(1<<31)
	
	var shape = CollisionShape3D.new()
	shape.set_shape(BoxShape3D.new())
	shape.get_shape().set_size(Vector3(.01,.01,.01))
	add_child(shape)
	
	name = params["socket_name"]
	
	var pos = params["position"]
	position = Vector3(pos[0], pos[1], pos[2])
	
	exact_orientation = params.get("exact", "false") == "true"
	if exact_orientation:
		var rot = params["rotation"]
		look_at(Vector3(rot[0],rot[1],rot[2]))
	
	socket_type = params["socket_type"]

func on_area_enter(area):
	if !connected:
		if socket_type == area.socket_type:
			if !exact_orientation || get_rotation() == area.get_rotation():
				connected = true
				connected_to = area
#				print("Connected " + self.to_string() + " to " + area.to_string())
				socket_connected.emit(self, area)

func on_area_exit(area):
	if connected:
		if area == connected_to:
			# Disconnect
			connected = false
			connected_to = null
#			print("Disconnected " + self.to_string() + " from " + area.to_string())
			socket_disconnected.emit(self, area)
