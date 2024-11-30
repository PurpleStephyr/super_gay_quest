use godot::classes::CharacterBody2D;
use godot::classes::ICharacterBody2D;
use godot::classes::InputEvent;
use godot::classes::InputEventMouseButton;
use godot::global::MouseButton;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    speed: f64,
    destination: Option<Vector2>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
    #[signal]
    fn set_destination(destination: Vector2);

    #[signal]
    fn reached_destination();
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
                if self.destination.is_none() {
                    let mut destination = event.get_position();
                    destination.x = (destination.x / 64.0).floor() * 64.0 + 32.0;
                    destination.y = (destination.y / 64.0).floor() * 64.0 + 32.0;
                    self.destination = Some(destination);
                    godot_print!("Got left mouse button press {}, {}", destination.x, destination.y);
                    self.base_mut().emit_signal("set_destination", &[destination.to_variant()]);
                }
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
                    self.base_mut().emit_signal("reached_destination", &[]);
                }
            }
        }
    }
}