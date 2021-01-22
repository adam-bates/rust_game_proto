use super::{
    components::{Player, CurrentPosition, TargetPosition},
    resources::Camera,
};
use specs::Join;

pub struct PrintSystem;

impl<'a> specs::System<'a> for PrintSystem {
    type SystemData = (
        Option<specs::Read<'a, Camera>>,
        specs::ReadStorage<'a, Player>,
        specs::ReadStorage<'a, CurrentPosition>,
        specs::ReadStorage<'a, TargetPosition>,
    );

    fn run(&mut self, (camera, player, real_position, target_position): Self::SystemData) {
        if let Some(camera) = camera {
            println!("Camera: [{}, {}]", camera.x, camera.y);
        }

        for (_, real_position, target_position) in
            (&player, &real_position, &target_position).join()
        {
            println!(
                "Player {{ real_position: [{}, {}], target_position: [{}, {}] }}",
                real_position.x, real_position.y, target_position.x, target_position.y
            );
        }

        for (_, real_position, target_position) in
            (!&player, &real_position, &target_position).join()
        {
            println!(
                "Not Player {{ real_position: [{}, {}], target_position: [{}, {}] }}",
                real_position.x, real_position.y, target_position.x, target_position.y
            );
        }
    }
}
