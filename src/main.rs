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

        for  c in &mut circles {
            c.position.0 += c.velocity.0 * dt;
            c.position.1 += c.velocity.1 * dt;
            if is_mouse_button_down(MouseButton::Left) {
                let g = calculate_gravity(&c.position, &mouse_position(), 5.0);
                c.velocity.0 += g.0;
                c.velocity.1 += g.1;
            }
            draw_faded_circle(&c.position);
        }
        next_frame().await
    }
}

fn calculate_gravity(a: &(f32, f32), b: &(f32, f32), weight: f32) -> (f32, f32) {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    let r = (dist_x * dist_x + dist_y * dist_y).sqrt();
    let scaled_r = r / 1000.0;
    let strength = 1.0 / scaled_r * scaled_r;
    let x_normalized = dist_x / r;
    let y_normalized = dist_y / r;
    let f_x = -strength * x_normalized * weight;
    let f_y = -strength * y_normalized * weight;
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
