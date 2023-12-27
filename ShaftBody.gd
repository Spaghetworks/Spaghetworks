extends Node

signal state_updated

var moment
var principal_axis
var elements = 0

var position = 0
var velocity = 0
var acceleration = 0

var accumulated_torque = 0

var sub_pos = 0
var sub_vel = 0
var sub_acc = 0

var springs = []
var a_constraints = []
var b_constraints = []

func update_state():
	state_updated.emit(position, velocity)

func add_torque(torque):
	accumulated_torque += torque
