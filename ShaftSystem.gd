extends Node

var shaft_body = preload("res://ShaftBody.gd")
var rebuild_request_set = {}
var skip_rebuild_request_set = {}
var physics = false
var physics_delay
var substeps = 10
var constraint_matrix
var constraints

# DEBUG
var error = 0

func _ready():
	var test = MatrixSolver.create(2, PackedFloat64Array([2,-1,-1,2]))
	print(test.solve(PackedFloat64Array([-20,0])))
	print(test.solve_elided(PackedFloat64Array([-20,10]), PackedInt64Array([0])))

func request_rebuild(element):
	rebuild_request_set[element] = true
	print("Requesting rebuild of " + element.to_string())

func to_simulation():
	print("Enabling shaft physics")
	physics = true
	physics_delay = 3

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
	print("Rebuilding constraint matrix")
	constraints = []
	for body in get_children():
		if !is_instance_valid(body) || body.is_queued_for_deletion():
			continue
		constraints.append_array(body.a_constraints)
	var size = constraints.size()
	print(" Found ", size, " constraint(s)")
	if size != 0:
		constraint_matrix = PackedFloat64Array()
		constraint_matrix.resize(size*size)
		
		var index = 0
		for constraint in constraints:
			match constraint.type:
				"proportional":
					for subconstraint in constraint.element_a.body.a_constraints:
						constraint_matrix[index * size + constraints.find(subconstraint)] +=  \
							1.0 / constraint.element_a.body.moment * constraint.ratio[0] * subconstraint.ratio[0] * constraint.element_a.get_alignment() * subconstraint.element_a.get_alignment()
					for subconstraint in constraint.element_a.body.b_constraints:
						constraint_matrix[index * size + constraints.find(subconstraint)] -=  \
							1.0 / constraint.element_a.body.moment * constraint.ratio[0] * subconstraint.ratio[1] * constraint.element_a.get_alignment() * subconstraint.element_b.get_alignment()
					for subconstraint in constraint.element_b.body.a_constraints:
						constraint_matrix[index * size + constraints.find(subconstraint)] -=  \
							1.0 / constraint.element_b.body.moment * constraint.ratio[1] * subconstraint.ratio[0] * constraint.element_b.get_alignment() * subconstraint.element_a.get_alignment()
					for subconstraint in constraint.element_b.body.b_constraints:
						constraint_matrix[index * size + constraints.find(subconstraint)] +=  \
							1.0 / constraint.element_b.body.moment * constraint.ratio[1] * subconstraint.ratio[1] * constraint.element_b.get_alignment() * subconstraint.element_b.get_alignment()
				"constantvel":
					for subconstraint in constraint.element_a.body.a_constraints:
						constraint_matrix[index * size + constraints.find(subconstraint)] += 1.0 / constraint.element_a.body.moment
					for subconstraint in constraint.element_a.body.b_constraints:
						constraint_matrix[index * size + constraints.find(subconstraint)] -= 1.0 / constraint.element_a.body.moment
			
			index += 1
		print(constraint_matrix)
		constraint_matrix = MatrixSolver.create(size, constraint_matrix)
	print("End rebuilds")

func _physics_process(delta):
	# Process rebuilds
	if not rebuild_request_set.is_empty():
		flush_rebuild_requests()
	
	for body in get_children():
		if body.elements == 0:
			print("freeing body")
			body.queue_free()
	
	if get_children().size() == 0:
		return
	
	# Physics sim
	if physics:
		if physics_delay == 0:
			step(delta)
		else:
			physics_delay -= 1

func step(delta):
	delta /= substeps
	var children = get_children()
	
#	for substep in range(0,1):
	for substep in range(0,substeps):
		for body in children:
			body.sub_pos = body.position + body.velocity * delta + body.acceleration * delta * delta / 2
		for body in children:
			# Accumulate torque
			for spring in body.springs:
				body.add_torque(spring.get_sub_torque(body))
			body.sub_acc = body.accumulated_torque / body.moment
		
		# Solve linear system
		if constraints.size() != 0:
			var b_vec = PackedFloat64Array()
			b_vec.resize(constraints.size())
			var index = 0
			for constraint in constraints:
				match constraint.type:
					"proportional":
						b_vec[index] = -constraint.element_a.free_sub_vel(delta) * constraint.ratio[0] + constraint.element_b.free_sub_vel(delta) * constraint.ratio[1]
					"constantvel":
						b_vec[index] = -constraint.element_a.free_sub_vel(delta) + 2 * constraint.velocity / delta
				index += 1
			var x_vec = constraint_matrix.solve(b_vec)
			
			# Solve slip
			var elided = []
			while true:
				var broken = false
				index = 0
				for constraint in constraints:
					if elided.has(index):
						index += 1
						continue
					match constraint.type:
						"proportional":
							if abs(x_vec[index]) > constraint.breaking_force:
								elided.append(index)
								broken = true
								x_vec[index] = clamp(x_vec[index], -constraint.breaking_force, constraint.breaking_force)
								constraint.element_a.add_torque( x_vec[index] * constraint.ratio[0])
								constraint.element_b.add_torque(-x_vec[index] * constraint.ratio[1])
					index += 1
				
				if !broken:
					break
				
				if elided.size() == constraints.size():
					break
				
				for body in children:
					body.sub_acc = body.accumulated_torque / body.moment
				
				index = 0
				for constraint in constraints:
					match constraint.type:
						"proportional":
							b_vec[index] = -constraint.element_a.free_sub_vel(delta) * constraint.ratio[0] + constraint.element_b.free_sub_vel(delta) * constraint.ratio[1]
						"constantvel":
							b_vec[index] = -constraint.element_a.free_sub_vel(delta) + 2 * constraint.velocity / delta
					index += 1
				
				elided.sort()
				x_vec = constraint_matrix.solve_elided(b_vec, PackedInt64Array(elided))
			
			index = 0
			for constraint in constraints:
				match constraint.type:
					"proportional":
						if abs(x_vec[index]) > constraint.breaking_force:
							print("Excess force!")
							print(index, " ", x_vec[index])
						constraint.element_a.add_torque( x_vec[index] * constraint.ratio[0])
						constraint.element_b.add_torque(-x_vec[index] * constraint.ratio[1])
					"constantvel":
						constraint.element_a.add_torque(x_vec[index])
				index += 1
		
		for body in children:
			body.sub_acc = body.accumulated_torque / body.moment
			body.sub_vel = body.velocity + (body.acceleration + body.sub_acc) * (delta / 2)
		for body in children:
			body.position = body.sub_pos
			body.accumulated_torque = 0
			body.acceleration = body.sub_acc
			body.velocity = body.sub_vel
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
	new_body.a_constraints.append_array(element.a_constraints)
	new_body.b_constraints.append_array(element.b_constraints)
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
		new_body.a_constraints.append_array(element[0].a_constraints)
		new_body.b_constraints.append_array(element[0].b_constraints)
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
	print(" A-Constraints: " + str(new_body.a_constraints))
	print(" B-Constraints: " + str(new_body.b_constraints))
	add_child(new_body)
