extends Camera2D

const speed = 400
const zoom_speed = 10

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var velocity = Vector2()

	if Input.is_action_pressed("ui_left"):  # A key
		velocity.x -= speed
	if Input.is_action_pressed("ui_right"):  # D key
		velocity.x += speed
	if Input.is_action_pressed("ui_up"):  # W key
		velocity.y -= speed
	if Input.is_action_pressed("ui_down"):  # S key
		velocity.y += speed
		
	var zoom = self.zoom.x
	var real_zoom_speed = 1 + zoom_speed * delta
	if Input.is_action_just_released("ui_zoom_in"):
		print("zoom_in")
		zoom *= real_zoom_speed
	if Input.is_action_just_released("ui_zoom_out"):
		print("zoom_out")
		zoom /= real_zoom_speed
	zoom = clamp(zoom,0,10)

	translate(velocity * delta)  # Apply movement
	self.zoom = Vector2(zoom,zoom)
	
	pass
