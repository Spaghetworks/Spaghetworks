extends Node

# Loads and assembles a block from a config file.

var systemBaseDirectory = "res://BlockSystem"
var blocks = {}
var loaded = false
signal block_registered

func _ready():
	print("Begin loading")
	var blocks_dir = DirAccess.open(systemBaseDirectory)
	var files = []
	
	
	var dirs = ["Blocks"]
	var dir = dirs.pop_front()

	while dir != null:
		blocks_dir.change_dir(dir) 
		print(blocks_dir.get_current_dir())
		# Search first directory in list
		for file in blocks_dir.get_files():
			if file.ends_with(".json"):
				files.append(blocks_dir.get_current_dir() + "/" + file)
		for newdir in blocks_dir.get_directories():
			dirs.append(blocks_dir.get_current_dir() + "/" + newdir)
		# Remove first directory from list
		dir = dirs.pop_front()

	print("Files: ", files)
	
	# Process each file into a block
	var block_template = load("res://BlockSystem/Block.gd")
	var block_data
	for file in files:
		# Read the file into structured data
		var block_file = FileAccess.open(file, FileAccess.READ)
		block_data = JSON.parse_string(block_file.get_as_text())
		print("Assembling block: " + block_data["name"])
		# Assemble the scene
		#  Create a new scene with a MeshInstance, Area3D, and CollisionShape.
		var block = block_template.new()
		block.add_child(Area3D.new())
		block.get_child(0).add_child(CollisionShape3D.new())
		block.name = "MeshInstance3D"
		block.get_child(0).name = "Area3D"
		block.get_child(0).get_child(0).name = "CollisionShape3D"
		#  Configure MeshInstance and CollisionShape
		if typeof(block_data["visual_mesh"]) == TYPE_STRING:
			# Asset path
			var mesh = load(systemBaseDirectory + "/" + block_data["visual_mesh"])
			print(mesh)
			block.set_mesh(mesh)
			block.add_mesh(mesh)
		else:
			# Generator
			print(block_data["visual_mesh"]["name"])
			match block_data["visual_mesh"]["name"]:
				"generator_cuboid":
					var box_shape = BoxMesh.new()
					var dimensions = block_data["visual_mesh"]["dimensions"]
					dimensions = Vector3(dimensions[0],dimensions[1],dimensions[2])
					dimensions /= 10
					box_shape.set_size(dimensions)
					block.set_mesh(box_shape)
					block.add_mesh(box_shape)
				"generator_none":
					pass
		
		if typeof(block_data["collision_mesh"]) == TYPE_STRING:
			# Asset path
			print("COLLISION ASSETS NOT YET HANDLED")
		else:
			# Generator
			print(block_data["collision_mesh"]["name"])
			match block_data["collision_mesh"]["name"]:
				"generator_cuboid":
					var box_shape = BoxShape3D.new()
					var dimensions = block_data["collision_mesh"]["dimensions"]
					dimensions = Vector3(dimensions[0],dimensions[1],dimensions[2])
					dimensions /= 10
					box_shape.set_size(dimensions)
					block.get_child(0).get_child(0).set_shape(box_shape)
		#  Add all modules
		if block_data.has("modules"):
			print("Adding modules")
			for module in block_data["modules"]:
				print(module["name"])
				var new_module = load(systemBaseDirectory +  "/BlockModules/" + module["name"] + ".gd").new()
				block.add_child(new_module)
				new_module.initialize(module["params"])
		else:
			print("No modules")
		#  Write metadata
		block.set_meta("name", block_data["name"])
		block.set_meta("display_name", block_data["display_name"])
		block.set_meta("category", block_data["category"])
		block.set_meta("mass", block_data["mass"])
		print("ADDITIONAL METADATA NOT YET HANDLED")
		# Add node to collection
		print(block)
		blocks[block_data["name"]] = block
		
		# Emit signal
		print("Block registered: " + block_data["name"])
		block_registered.emit(block_data["name"], block_data["display_name"])
	print("Loading done.")
	loaded = true
