use bevy_ecs::prelude::*;
mod components;
mod systems;

use components::*;
use macroquad::prelude::*;
use systems::*;
#[derive(Component)]
struct Player {}
#[macroquad::main("MyGame")]
async fn main() {
    let mut world = World::new();

    let mut schedule = Schedule::default();
    let dirt: Texture2D = load_texture("assets/dirt.png").await.unwrap();
    schedule.add_systems((movement, collision, render, spriterender, input));

    world.spawn((
        Position { x: 10.0, y: 500.0 },
        Velocity { x: 0.0, y: 0.0 },
        Rectangle {
            x: 32.0,
            y: 32.0,
            color: RED,
        },
        Player {},
        Collider { x: 32.0, y: 32.0 },
    ));

    world.spawn((
        Position { x: 10.0, y: 200.0 },
        Velocity { x: 0.0, y: 0.0 },
        Sprite {
            x: 32.0,
            y: 32.0,
            texture: dirt,
            color: WHITE,
        },
        Collider { x: 32.0, y: 32.0 },
    ));

    loop {
        clear_background(BLACK);
        schedule.run(&mut world);
        next_frame().await;
    }
}
fn input(query: Query<(&mut Velocity, &Player)>) {
    for (mut velocity, sprite) in query {
        if is_key_pressed(KeyCode::Up) {
            velocity.y += -1.0;
        }
    }
}
