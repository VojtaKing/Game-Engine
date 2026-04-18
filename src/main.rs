use bevy_ecs::prelude::*;
use macroquad::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Rectangle {
    x: f32,
    y: f32,
    color: Color,
}
#[derive(Component)]
struct Sprite {
    x: f32,
    y: f32,
    texture: Texture2D,
    color: Color,
}

#[derive(Component)]
struct Collider {
    x: f32,
    y: f32,
}
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

fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

fn render(query: Query<(&Position, &Rectangle)>) {
    for (position, rect) in &query {
        draw_rectangle(position.x, position.y, rect.x, rect.y, rect.color);
    }
}
fn spriterender(query: Query<(&Position, &Sprite)>) {
    for (position, sprite) in &query {
        draw_texture(&sprite.texture, position.x, position.y, sprite.color);
    }
}
fn input(query: Query<(&mut Velocity, &Player)>) {
    for (mut velocity, sprite) in query {
        if is_key_pressed(KeyCode::Up) {
            velocity.y += -1.0;
        }
    }
}
fn collision(mut query: Query<(&mut Position, &mut Velocity, &Collider)>) {
    for _ in 0..2 {
        let mut combinations = query.iter_combinations_mut();

        while let Some([(mut pos_a, mut vel_a, col_a), (mut pos_b, mut vel_b, col_b)]) =
            combinations.fetch_next()
        {
            let rect_a = Rect::new(pos_a.x, pos_a.y, col_a.x, col_a.y);
            let rect_b = Rect::new(pos_b.x, pos_b.y, col_b.x, col_b.y);

            if rect_a.overlaps(&rect_b) {
                let dx = (rect_a.x + rect_a.w * 0.5) - (rect_b.x + rect_b.w * 0.5);

                let px = (rect_a.w * 0.5 + rect_b.w * 0.5) - dx.abs();

                let dy = (rect_a.y + rect_a.h * 0.5) - (rect_b.y + rect_b.h * 0.5);
                let py = (rect_a.h * 0.5 + rect_b.h * 0.5) - dy.abs();

                let move_a = vel_a.x.abs() + vel_a.y.abs() > vel_b.x.abs() + vel_b.y.abs();

                if px < py {
                    if dx > 0.0 {
                        if move_a {
                            pos_a.x += px;
                            vel_a.x = 0.0;
                        } else {
                            pos_b.x -= px;
                            vel_b.x = 0.0;
                        }
                    } else {
                        if move_a {
                            pos_a.x -= px;
                            vel_a.x = 0.0;
                        } else {
                            pos_b.x += px;
                            vel_b.x = 0.0;
                        }
                    }
                } else {
                    if dy > 0.0 {
                        if move_a {
                            pos_a.y += py;
                            vel_a.y = 0.0;
                        } else {
                            pos_b.y -= py;
                            vel_b.y = 0.0;
                        }
                    } else {
                        if move_a {
                            pos_a.y -= py;
                            vel_a.y = 0.0;
                        } else {
                            pos_b.y += py;
                            vel_b.y = 0.0;
                        }
                    }
                }
            }
        }
    }
}
