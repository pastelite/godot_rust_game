extends Label


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_player_change_mode(mode):
	print("change mode to",mode)
	match mode:
		0: text = "--none--"
		1: text = "build"
	pass # Replace with function body.
