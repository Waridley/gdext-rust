extends Node3D

const GdClass = preload("res://GdClass.gd")

@onready var rust_test := $RustTest as RustTest

var c
var d
var e

func _init():
	print("[GDScript] _init")

	connect("unload", self._unload, CONNECT_DEFERRED)
	
	print("[GDScript] end _init")


func _ready():
	print("[GDScript] _ready")

	print()
	rust_test.print_input_map_actions()
	print()

	rust_test.name = "rust_test"

	print("self._to_string(): ", self)

	var msg := rust_test.test_method(12, "hello from GDScript")
	print(msg)

	var node = Node3D.new()
	#print("Correct to_global(): ", node.to_global(Vector3(2.0, 3.0, 4.0)))
	print("  instance_id (gd): ", node.get_instance_id())

	print("<--")

	rust_test.call_node_method(node)
	node.free()
	#rust_test.call_base_method()
	print("-->")

#	get_tree().quit()
#	return

	var res := rust_test.add(4, 6, Vector2(3, 5))
	print(res)
	var res_vec := rust_test.vec_add(Vector2(1, 2), Vector2(3, 4))
	print(res_vec)

	print_instance_id(self, "self")
	var gd_class = GdClass.new()
	print_instance_id(gd_class, "new")
	gd_class.unreference()
	print_instance_id($MeshInstance3D, "mesh")
	print_instance_id($WorldEnvironment, "env")
	print_instance_id($DirectionalLight3D, "light")


	print()
	print("----------------------------")
	if true: #scope
		var manual_mem = RustTest.new()
		var id = manual_mem.get_instance_id()
		manual_mem.free()
		#manual_mem.return_obj()
		var restored = instance_from_id(id)
		print("restored:  ", restored)
		print("is null:   ", restored == null)
		print("typeof:    ", typeof(restored))
		print("valid:     ", is_instance_valid(restored))
		print("id valid:  ", is_instance_id_valid(id))
		#print("get_class: ", restored.get_class()) # fails
		#restored.return_obj()
	print("----------------------------")


	var obj = rust_test.return_obj()
	print_instance_id(obj, "entity")
	rust_test.accept_obj(obj)

	if true: #scope
		var obj2 = rust_test.find_obj(obj.get_instance_id())
		print_instance_id(obj, "entity (again)")
		print_instance_id(obj2, "entity (via get_instance_id)")
		# note: end of scope doesn't unreference

	#print("Create some more refs...") # note: adding variables doesn't call reference()
	#c = obj
	#rust_test.accept_obj(obj)
	print()

	print("[GDScript] end _ready")
#	get_tree().quit()
#	print("[GDScript] after quit")

signal unload
var loaded = true

func _unload():
	var status
	if loaded:
		print("unloading...")
		rust_test.free() # safety: only calling deferred
		status = NativeExtensionManager.unload_extension("res://gdext-rust-test.gdextension")
		loaded = false
	else:
		print("loading...")
		status = NativeExtensionManager.load_extension("res://gdext-rust-test.gdextension")
		rust_test = RustTest.new()
		add_child(rust_test)
		loaded = true
	
	print(status)

func _input(event):
	if event.is_action_pressed("ui_accept"):
		emit_signal("unload")
		

func print_instance_id(obj, msg=null):
	var full = obj.get_instance_id()
#	var low = full & 0xFFFFFFFF
#	var high = (full / 4294967296) & 0xFFFFFFFF # Not '>> 32' because GDScript disallows shift of negative numbers

#	if msg == null:
#		print("instance id: %x%x" % [high, low])
#	else:
#		print("instance id: %x%x  (%s)" % [high, low, msg])

	if msg == null:
		print("instance id: ", full)
	else:
		print("instance id: ", full, " (", msg, ")")
	print("  _to_string(): ", obj)
