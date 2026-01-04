use macroquad::prelude::*;

static CIRCLE_RADIUS: f32 = 8.0;
struct Circle {
    position: (f32, f32),
    velocity: (f32, f32),
}
#[macroquad::main("Gravity Simulation")]
async fn main() {
    println!("Hello, gravity simulation");
    let mut circles: Vec<Circle> = (0..1000).map(|_| random_circle()).collect();
    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        for c in &mut circles {
            c.position.0 += c.velocity.0 * dt;
            c.position.1 += c.velocity.1 * dt;
            if is_mouse_button_down(MouseButton::Left) {
                let g = calculate_gravity(&c.position, &mouse_position(), 5.0);
                c.velocity.0 += g.0 * dt;
                c.velocity.1 += g.1 * dt;
            }
            let friction = 1.0 - 0.1 * dt;
            c.velocity.0 *= friction;
            c.velocity.1 *= friction;
            draw_faded_circle(&c.position);
            if circle_out_of_bounds(&c.position) {
                *c = random_circle();
            }
        }
        next_frame().await
    }
}

fn calculate_gravity(a: &(f32, f32), b: &(f32, f32), weight: f32) -> (f32, f32) {
    let dist_x = a.0 - b.0;
    let dist_y = a.1 - b.1;
    let r = (dist_x * dist_x + dist_y * dist_y).sqrt();
    let scaled_r = r / 1000.0;
    let strength = 50.0 / scaled_r * scaled_r;
    let x_normalized = dist_x / r;
    let y_normalized = dist_y / r;
    let f_x = -strength * x_normalized * weight;
    let f_y = -strength * y_normalized * weight;
    return (f_x, f_y);
}

fn circle_out_of_bounds(circle_position: &(f32, f32)) -> bool {
    let screen_dimensions = (screen_width(), screen_height());
    return (circle_position.0 + CIRCLE_RADIUS < 0.0)
        || (circle_position.1 + CIRCLE_RADIUS < 0.0)
        || (circle_position.0 - CIRCLE_RADIUS > screen_dimensions.0)
        || (circle_position.1 - CIRCLE_RADIUS > screen_dimensions.1);
}

fn random_circle() -> Circle {
    let screen_dimensions = (screen_width(), screen_height());
    let rand_screen_position: f32 = rand::gen_range(0.0, screen_dimensions.0 + screen_dimensions.1);
    let mut circle = Circle {
        position: (0.0, 0.0),
        velocity: (0.0, 0.0),
    };
    if rand_screen_position > screen_dimensions.0 {
        circle.position.1 = rand_screen_position - screen_dimensions.0 - CIRCLE_RADIUS;
        circle.position.0 = -CIRCLE_RADIUS;
    } else {
        circle.position.0 = rand_screen_position - CIRCLE_RADIUS;
        circle.position.1 = -CIRCLE_RADIUS;
    }
    let min_velocity = 20.0;
    let max_velocity = 100.0;
    let velocity = rand::gen_range(min_velocity, max_velocity);
    circle.velocity.0 = velocity;
    circle.velocity.1 = velocity;
    return circle;
}

fn draw_faded_circle(circle_position: &(f32, f32)) {
    let orange_1 = ORANGE;
    let orange_2: Color = Color { a: 0.8, ..ORANGE };
    let orange_3: Color = Color { a: 0.4, ..ORANGE };
    let orange_4: Color = Color { a: 0.2, ..ORANGE };
    let orange_5: Color = Color { a: 0.1, ..ORANGE };
    draw_circle(
        circle_position.0,
        circle_position.1,
        CIRCLE_RADIUS * 0.125,
        orange_1,
    );
    draw_circle(
        circle_position.0,
        circle_position.1,
        CIRCLE_RADIUS * 0.25,
        orange_2,
    );
    draw_circle(
        circle_position.0,
        circle_position.1,
        CIRCLE_RADIUS * 0.5,
        orange_3,
    );
    draw_circle(
        circle_position.0,
        circle_position.1,
        CIRCLE_RADIUS * 0.75,
        orange_4,
    );
    draw_circle(
        circle_position.0,
        circle_position.1,
        CIRCLE_RADIUS,
        orange_5,
    );
}
