extends Node

var shaft_body = preload("res://ShaftBody.gd")
var rebuild_request_set = {}
var skip_rebuild_request_set = {}
var physics = false
var substeps = 10

func request_rebuild(element):
	rebuild_request_set[element] = true
	print("Requesting rebuild of " + element.to_string())

func to_simulation():
	print("Enabling shaft physics")
	physics = true

func flush_rebuild_requests():
	print("Begin rebuilds")
	for rebuild_request in rebuild_request_set:
		# Pop a request and rebuild it
		if rebuild_request not in skip_rebuild_request_set:
			rebuild(rebuild_request)
		else:
			print("Skip rebuild of " + rebuild_request.to_string())
	rebuild_request_set.clear()
	skip_rebuild_request_set.clear()
	print("End rebuilds")

func _physics_process(delta):
	# Process rebuilds
	if not rebuild_request_set.is_empty():
		flush_rebuild_requests()
	
#	# Placeholder constant velocity rotation
#	var constant_velocity = .001
#	for body in get_children():
#		body.set_state(body.position + body.velocity, constant_velocity)
	
	for body in get_children():
		if body.elements == 0:
			print("freeing body")
			body.queue_free()
	
	# Physics sim
	if physics:
#		print("physics step")
		step(delta)

func step(delta):
	delta /= substeps
	var children = get_children()
	
	for substep in range(0,substeps):
		for body in children:
			body.sub_pos = body.position + body.velocity * delta + body.acceleration * delta * delta / 2
			body.position = body.sub_pos
		for body in children:
			# Accumulate torque
			for spring in body.springs:
				body.add_torque(spring.get_torque(body))
			body.sub_acc = body.accumulated_torque / body.moment
		for body in children:
			body.sub_vel = body.velocity + (body.acceleration + body.sub_acc) * (delta / 2)
			body.velocity = body.sub_vel
		for body in children:
			body.acceleration = body.sub_acc
			body.accumulated_torque = 0
		for body in children:
			body.update_state()

func rebuild(element):
	# Detach old shaft body (if extant) and add no-dupe to old body collection
	var old_bodies = []
	if element.body:
		old_bodies.append(element.body)
	# Create and attach new shaft body
	var new_body = shaft_body.new()
	element.attach(new_body)
	# Set up based on the starting element
	new_body.moment = element.moment
	new_body.elements = 1
	new_body.principal_axis = element.get_parent().global_transform.basis * element.axis
	# Springs and constraints
	new_body.springs.append_array(element.springs)
	new_body.constraints.append_array(element.constraints)
	# Enqueue both ends (if applicable) and from-element
	var element_queue = []
	if element.connected_element_a:
		element_queue.append([element.connected_element_a,element])
	if element.connected_element_b:
		element_queue.append([element.connected_element_b,element])
	# Repeat until empty:
	while element_queue.size() > 0:
		# Pop next element
		element = element_queue.pop_back()
		# Erase from rebuild_requests to avoid duplicate rebuilds
		skip_rebuild_request_set[element[0]] = true
		# Detach old shaft body and add no-dupe to old body collection
		if element[0].body && !old_bodies.has(element[0].body):
			old_bodies.append(element[0].body)
		# Attach new shaft body
		element[0].attach(new_body)
		# Accumulate element into shaft body
		new_body.moment += element[0].moment
		new_body.elements += 1
		# Rebuild constraints/springs
		new_body.springs.append_array(element[0].springs)
		new_body.constraints.append_array(element[0].constraints)
		# Enqueue next neighbor
		if element[0].connected_element_a != element[1]: # check if this is neighbor A or B
			if element[0].connected_element_a: # null check
				element_queue.append([element[0].connected_element_a, element[0]])
		else:
			if element[0].connected_element_b: # null check
				element_queue.append([element[0].connected_element_b, element[0]])
	# Clean up
	for body in old_bodies:
		if is_instance_valid(body):
			body.queue_free()
	print("Rebuild completed: " + new_body.to_string())
	print(" Moment: " + str(new_body.moment))
	print(" Elements: " + str(new_body.elements))
	add_child(new_body)
