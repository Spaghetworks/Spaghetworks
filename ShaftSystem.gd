extends Node

var shaft_body = preload("res://ShaftBody.gd")
var rebuild_requests = []
var physics = false
var substeps = 100

func request_rebuild(element):
	rebuild_requests.append(element)
	print("Requesting rebuild of " + element.to_string())

func to_simulation():
	print("Enabling shaft physics")
	physics = true

func _physics_process(delta):
	# Process rebuilds
	while rebuild_requests.size() > 0:
		# Pop a request and rebuild it
		rebuild(rebuild_requests.pop_front())
	
#	# Placeholder constant velocity rotation
#	var constant_velocity = .001
#	for body in get_children():
#		body.set_state(body.position + body.velocity, constant_velocity)
	
	# Physics sim
	if physics:
#		print("physics step")
		step(delta)

func step(delta):
#	delta /= 10
	for substep in range(0,substeps):
		for body in get_children():
			# Accumulate torque
			for spring in body.springs:
				body.add_torque(spring.get_torque(body))
			# Update pos and vel by velocity verlet
			body.sub_pos = body.position + body.velocity * delta / substeps + body.acceleration * delta * delta / (substeps * substeps)
			body.sub_acc = body.accumulated_torque / body.moment
			body.sub_vel = body.velocity + (body.acceleration + body.sub_acc) * (delta / 2 / substeps)
		
		for body in get_children():
			body.position = body.sub_pos
			body.velocity = body.sub_vel
			body.acceleration = body.sub_acc
			body.accumulated_torque = 0
	for body in get_children():
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
		pass
		# Pop next element
		element = element_queue.pop_front()
		# Erase from rebuild_requests to avoid duplicate rebuilds
		rebuild_requests.erase(element[0])
		# Detach old shaft body and add no-dupe to old body collection
		if element[0].body && !old_bodies.has(element[0].body):
			old_bodies.append(element[0].body)
		# Attach new shaft body
		element[0].attach(new_body)
		# Accumulate element into shaft body
		new_body.moment += element[0].moment
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
		body.queue_free()
	print("Rebuild completed: " + new_body.to_string())
	print(" Moment: " + str(new_body.moment))
	add_child(new_body)
