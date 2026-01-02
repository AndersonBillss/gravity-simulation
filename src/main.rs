use macroquad::prelude::*;

struct Circle {
    position: (f32, f32),
    velocity: (f32, f32),
}
#[macroquad::main("Gravity Simulation")]
async fn main() {
    println!("Hello, gravity simulation");
    let mut circles: Vec<Circle> = vec![
        Circle {
            position: (30.0, 30.0),
            velocity: (30.0, 30.0),
        },
        Circle {
            position: (400.0, 400.0),
            velocity: (30.0, -30.0),
        }
    ];
    loop {
        clear_background(BLACK);
        let dt = get_frame_time();
        for c in &mut circles {
            c.position.0 += c.velocity.0 * dt;
            c.position.1 += c.velocity.1 * dt;
            draw_faded_circle(&c.position);
        }
        next_frame().await
    }
}

fn draw_faded_circle(circle_position: &(f32, f32)) {
    let orange_1 = ORANGE;
    let orange_2: Color = Color { a: 0.8, ..ORANGE };
    let orange_3: Color = Color { a: 0.4, ..ORANGE };
    let orange_4: Color = Color { a: 0.2, ..ORANGE };
    let orange_5: Color = Color { a: 0.1, ..ORANGE };
    draw_circle(circle_position.0, circle_position.1, 1.0, orange_1);
    draw_circle(circle_position.0, circle_position.1, 2.0, orange_2);
    draw_circle(circle_position.0, circle_position.1, 4.0, orange_3);
    draw_circle(circle_position.0, circle_position.1, 6.0, orange_4);
    draw_circle(circle_position.0, circle_position.1, 8.0, orange_5);
}
