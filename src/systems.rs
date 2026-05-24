use crate::components::*;
use bevy_ecs::prelude::*;
use macroquad::prelude::*;
pub fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

pub fn render(query: Query<(&Position, &Rectangle)>) {
    for (position, rect) in &query {
        draw_rectangle(position.x, position.y, rect.x, rect.y, rect.color);
    }
}

pub fn spriterender(query: Query<(&Position, &Sprite)>) {
    for (position, sprite) in &query {
        draw_texture_ex(
            &sprite.texture,
            position.x,
            position.y,
            sprite.color,
            DrawTextureParams {
                dest_size: (Some((vec2(sprite.x, sprite.y)))),
                ..Default::default()
            },
        );
    }
}
pub fn collision_detection(query: Query<(Entity, &Position, &Collider)>, mut commands: Commands) {
    let mut combinations = query.iter_combinations();

    while let Some([(e1, pos_a, col_a), (e2, pos_b, col_b)]) = combinations.fetch_next() {
        let rect_a = Rect::new(pos_a.x, pos_a.y, col_a.x, col_a.y);
        let rect_b = Rect::new(pos_b.x, pos_b.y, col_b.x, col_b.y);

        if rect_a.overlaps(&rect_b) {
            commands.trigger(CollisionEvent { a: e1, b: e2 });
        }
    }
}
pub fn collision_resolution(
    mut query: Query<(
        Entity,
        &mut Position,
        &mut Velocity,
        &Collider,
        Option<&Solid>,
    )>,
) {
    let mut combinations = query.iter_combinations_mut();

    while let Some(
        [
            (e1, mut pos_a, mut vel_a, col_a, solid_a),
            (e2, mut pos_b, mut vel_b, col_b, solid_b),
        ],
    ) = combinations.fetch_next()
    {
        if solid_a.is_none() || solid_b.is_none() {
            continue;
        }

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
