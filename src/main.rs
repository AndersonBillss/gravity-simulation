use macroquad::prelude::*;

static CIRCLE_RADIUS: f32 = 8.0;
static CIRCLE_COLOR: Color = ORANGE;
struct Circle {
    position: (f32, f32),
    velocity: (f32, f32),
}
#[macroquad::main("Gravity Simulation")]
async fn main() {
    let mut circles: Vec<Circle> = (0..4000).map(|_| random_circle()).collect();
    let circle_texture = create_soft_circle_texture();
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
            } else if is_mouse_button_down(MouseButton::Right) {
                let g = calculate_gravity(&c.position, &mouse_position(), 5.0);
                c.velocity.0 -= g.0 * dt;
                c.velocity.1 -= g.1 * dt;
            }
            let friction = 1.0 - 0.1 * dt;
            c.velocity.0 *= friction;
            c.velocity.1 *= friction;
            draw_faded_circle(&c.position, &circle_texture);
            if circle_out_of_bounds(&c.position) {
                c.position = reflect(&c.position);
            }
        }
        next_frame().await
    }
}

fn reflect(coords: &(f32, f32)) -> (f32, f32) {
    let screen_dimensions = (screen_width(), screen_height());
    let screen_mid = (screen_dimensions.0 / 2.0, screen_dimensions.1 / 2.0);
    let adjusted_coords = (coords.0 - screen_mid.0, coords.1 - screen_mid.1);
    let flipped_coords = (-adjusted_coords.0, -adjusted_coords.1);

    let slope_point_to_center = flipped_coords.1 / flipped_coords.0;
    let slope_corner_to_corner = screen_dimensions.1 / screen_dimensions.0;

    let in_bounds_coords: (f32, f32);
    if slope_point_to_center.abs() > slope_corner_to_corner.abs() {
        let mut y = screen_mid.1 + (CIRCLE_RADIUS - 1.0);
        if flipped_coords.1 < 0.0 {
            y = -y;
        }
        let scale_factor = y / flipped_coords.1;
        in_bounds_coords = (flipped_coords.0 * scale_factor, y);
    } else {
        let mut x = screen_mid.0 + (CIRCLE_RADIUS - 1.0);
        if flipped_coords.0 < 0.0 {
            x = -x;
        }
        let scale_factor = x / flipped_coords.0;
        in_bounds_coords = (x, flipped_coords.1 * scale_factor);
    }

    return (
        in_bounds_coords.0 + screen_mid.0,
        in_bounds_coords.1 + screen_mid.1,
    );
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

fn draw_faded_circle(circle_position: &(f32, f32), texture: &Texture2D) {
    draw_texture_ex(
        &texture,
        circle_position.0 - CIRCLE_RADIUS,
        circle_position.1 - CIRCLE_RADIUS,
        CIRCLE_COLOR,
        DrawTextureParams {
            dest_size: None,
            source: None,
            rotation: 0.0,
            flip_x: false,
            flip_y: false,
            pivot: None,
        },
    );
}

fn create_soft_circle_texture() -> Texture2D {
    let image_dimensions: (u16, u16) = ((CIRCLE_RADIUS as u16) * 2, (CIRCLE_RADIUS as u16) * 2);
    let mut image = Image::gen_image_color(
        image_dimensions.0,
        image_dimensions.1,
        Color { a: 0.0, ..WHITE },
    );
    for i in 0..image_dimensions.0 {
        for j in 0..image_dimensions.1 {
            let i_adjusted = i as f32 - CIRCLE_RADIUS;
            let j_adjusted = j as f32 - CIRCLE_RADIUS;
            let distance_from_center = (i_adjusted * i_adjusted + j_adjusted * j_adjusted).sqrt();
            image.set_pixel(
                i as u32,
                j as u32,
                Color {
                    a: 1.0 - ((distance_from_center * 2.0) / image_dimensions.0 as f32),
                    ..WHITE
                },
            );
        }
    }
    let tex = Texture2D::from_image(&image);
    tex.set_filter(FilterMode::Linear);
    tex
}
