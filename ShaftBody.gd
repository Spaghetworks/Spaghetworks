extends Node

signal state_updated

var moment
var principal_axis

var position = 0
var velocity = 0
var acceleration = 0

var accumulated_torque = 0

var sub_pos = 0
var sub_vel = 0
var sub_acc = 0

var springs = []
var constraints = []

func update_state():
	state_updated.emit(position, velocity)

func add_torque(torque):
	accumulated_torque += torque
