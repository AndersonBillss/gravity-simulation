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
            position: (200.0, 200.0),
            velocity: (30.0, 200.0),
        },
        Circle {
            position: (300.0, 300.0),
            velocity: (-30.0, -200.0),
        },
        Circle {
            position: (1000.0, 800.0),
            velocity: (0.0, 0.0),
        },
        Circle {
            position: (1500.0, 900.0),
            velocity: (0.0, 0.0),
        },
    ];
    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        let mut forces = vec![(0.0, 0.0); circles.len()];
        for i in 0..circles.len() {
            for j in 0..circles.len() {
                if i != j {
                    let gravity = calculate_gravity(&circles[i].position, &circles[j].position);
                    forces[i].0 += gravity.0;
                    forces[i].1 += gravity.1;
                }
            }
        }
        for (i, c) in &mut circles.iter_mut().enumerate() {
            c.position.0 += c.velocity.0 * dt;
            c.position.1 += c.velocity.1 * dt;
            c.velocity.0 += forces[i].0;
            c.velocity.1 += forces[i].1;
            draw_faded_circle(&c.position);
        }
        next_frame().await
    }
}

fn calculate_gravity(a: &(f32, f32), b: &(f32, f32)) -> (f32, f32) {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    let r = (dist_x * dist_x + dist_y * dist_y).sqrt();
    let scaled_r = r / 1000.0;
    let strength = 1.0 / scaled_r * scaled_r;
    let x_normalized = dist_x / r;
    let y_normalized = dist_y / r;
    let f_x = -strength * x_normalized;
    let f_y = -strength * y_normalized;
    return (f_x, f_y);
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
