use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct Path {
    base: Base<Node2D>,
}

#[godot_api]
impl Path {

    #[func]
    fn on_player_reached_destination(&mut self) {
        godot_print!("player reached destination");
        self.base_mut().queue_free();
    }
}
