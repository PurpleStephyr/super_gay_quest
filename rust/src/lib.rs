use godot::{prelude::*};
use path::Path;
use player::Player;

mod player;
mod path;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    path_scene: Gd<PackedScene>,
    base: Base<Node>,
}

#[godot_api]
impl Main {

    #[func]
    fn on_set_destination(&mut self, destination: Vector2) {
        godot_print!("Set destination {}", destination);

        let mut path_scene = self.path_scene.instantiate_as::<Node2D>();
        path_scene.set_position(destination);
        self.base_mut().add_child(&path_scene);

        let path = path_scene.cast::<Path>();

        let mut player = self.base().get_node_as::<Player>("Player");
        player.connect("reached_destination", &path.callable("on_player_reached_destination"));
    }
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        godot_print!("constructing Main");
        Main {
            path_scene: PackedScene::new_gd(),
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("main ready");
        self.path_scene = load("res://path.tscn");
    }
}