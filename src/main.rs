use bevy_ecs::prelude::*;
mod components;
mod systems;

use components::*;
use macroquad::prelude::{collections::storage::get_mut, *};
use systems::*;
#[derive(Component)]
struct Player {}
#[macroquad::main("MyGame")]
async fn main() {
    let mut world = World::new();

    world.add_observer(|ev: On<CollisionEvent>, mut query: Query<&mut Velocity>| {
        if let Ok(mut vel) = query.get_mut(ev.a) {
            vel.x *= -1.0;
            vel.y *= -1.0;
        }

        if let Ok(mut vel) = query.get_mut(ev.b) {
            vel.x *= -1.0;
            vel.y *= -1.0;
        }
    });
    let mut schedule = Schedule::default();
    let dirt: Texture2D = load_texture("assets/dirt.png").await.unwrap();
    dirt.set_filter(FilterMode::Nearest);
    schedule.add_systems((
        collision_detection,
        collision_resolution,
        movement,
        render,
        spriterender,
        input,
        debug,
    ));

    world.spawn((
        Position { x: 10.0, y: 500.0 },
        Velocity { x: 0.0, y: -10.0 },
        Rectangle {
            x: 32.0,
            y: 32.0,
            color: BLUE,
        },
        Collider { x: 32.0, y: 32.0 },
    ));

    world.spawn((
        Position { x: 10.0, y: 200.0 },
        Velocity { x: 0.0, y: 0.0 },
        Sprite {
            x: 64.0,
            y: 64.0,
            texture: dirt.clone(),
            color: WHITE,
        },
        Solid,
        Collider { x: 64.0, y: 64.0 },
    ));
    world.spawn((
        Position { x: 10.0, y: 800.0 },
        Velocity { x: 0.0, y: 0.0 },
        Sprite {
            x: 64.0,
            y: 64.0,
            texture: dirt.clone(),
            color: WHITE,
        },
        Solid,
        Collider { x: 64.0, y: 64.0 },
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
pub fn debug(query: Query<(&Position, &Collider)>) {
    for (position, collider) in &query {
        draw_rectangle_lines(position.x, position.y, collider.x, collider.y, 2.0, RED);
    }
}
