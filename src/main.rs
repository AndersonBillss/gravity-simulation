use macroquad::prelude::*;

struct Position {
    x: f32,
    y: f32,
}
#[macroquad::main("Gravity Simulation")]
async fn main() {
    println!("Hello, gravity simulation");
    let mut circle1_position: Position = Position { x: 30.0, y: 30.0 };
    loop {
        circle1_position.x += 1.0;
        circle1_position.y += 1.0;
        clear_background(BLACK);
        draw_faded_circle(&circle1_position);
        next_frame().await
    }
}

fn draw_faded_circle(circle_position: &Position) {
    let orange_1 = ORANGE;
    let orange_2: Color = Color { a: 0.8, ..ORANGE };
    let orange_3: Color = Color { a: 0.4, ..ORANGE };
    let orange_4: Color = Color { a: 0.2, ..ORANGE };
    let orange_5: Color = Color { a: 0.1, ..ORANGE };
    draw_circle(circle_position.x, circle_position.y, 1.0, orange_1);
    draw_circle(circle_position.x, circle_position.y, 2.0, orange_2);
    draw_circle(circle_position.x, circle_position.y, 4.0, orange_3);
    draw_circle(circle_position.x, circle_position.y, 6.0, orange_4);
    draw_circle(circle_position.x, circle_position.y, 8.0, orange_5);
}
