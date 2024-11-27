use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::InputEvent;
use godot::classes::InputEventMouseButton;
use godot::global::MouseButton;
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    speed: f64,
    destination: Option<Vector2>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Hello, world!");

        Self {
            speed: 400.0,
            destination: None,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseButton>() {
            if event.is_pressed() && event.get_button_index() == MouseButton::LEFT {
                self.destination = Some(event.get_position());
                godot_print!("Got left mouse button press {}, {}", self.destination.unwrap().x, self.destination.unwrap().y);
            }
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if let Some(destination) = self.destination {
            match self.base().get_position().distance_to(destination) {
                d if d > 10.0 => {
                    let velocity = self.base().get_position().direction_to(destination) * self.speed as f32;
                    self.base_mut().set_velocity(velocity);
                    self.base_mut().move_and_slide();
                }
                d if d > 0.0 => {
                    self.base_mut().set_position(destination);
                }
                _ => {
                    self.destination = None;
                }
            }
        }
    }
}