use macroquad::prelude::*;

#[macroquad::main("my-rust-pong")]
async fn main() {

    // Common Paddle variables
    let paddle_height = 100.0;
    let paddle_width = 20.0;
    let paddle_speed = 10.0;

    // Left Paddle
    let paddle1_x = 20.0;
    let mut paddle1_y = (screen_height() / 2.0) - 50.0;

    // Right Paddle
    let paddle2_x = screen_width() - 40.0;
    let mut paddle2_y = (screen_height() / 2.0) - 50.0;

    // Ball
    let mut ball_x = screen_width() / 2.0;
    let mut ball_y = screen_height() / 2.0;
    let diameter = 30.0;
    let radius = diameter / 2.0;
    let mut ball_speed_x = 2.5;
    let mut ball_speed_y = 2.5;

    loop {
        // Paddle Movement
        // Upper and Lower Bound Constraints

        // Right Paddle
        if paddle2_y <= screen_height() - paddle_height {
            if is_key_down(KeyCode::Down) {
                paddle2_y += paddle_speed;
            }
        }
        if paddle2_y >= 0.0 {
            if is_key_down(KeyCode::Up) {
                paddle2_y -= paddle_speed;
            }
        }

        // Left Paddle
        if paddle1_y <= screen_height() - paddle_height {
            if is_key_down(KeyCode::A) {
                paddle1_y += paddle_speed;
            }
        }
        if paddle1_y >= 0.0 {
            if is_key_down(KeyCode::Q) {
                paddle1_y -= paddle_speed;
            }
        }

        // Ball Movement
        ball_x -= ball_speed_x;
        ball_y -= ball_speed_y;

        // Ball Collisions
        // Left Paddle Collision
        if ball_x - radius > paddle1_x && ball_x - radius < paddle1_x + paddle_width {
            if ball_y > paddle1_y && ball_y < paddle1_y + paddle_height {
                ball_speed_x *= -1.0;
            }
        }

        // Right Paddle Collision
        if ball_x + radius > paddle2_x && ball_x + radius > paddle2_x - paddle_width {
            if ball_y > paddle2_y && ball_y < paddle2_y + paddle_height {
                ball_speed_x *= -1.0;
            }
        }

        // Floor Collision
        if ball_y + radius == screen_height() {
            ball_speed_y *= -1.0;
        }

        // Ceiling Collision
        if ball_y - radius == 0.0 {
            ball_speed_y *= -1.0;
        }

        // Background color
        clear_background(BLACK);

        // Draw Paddles
        draw_rectangle(paddle1_x, paddle1_y, paddle_width, paddle_height, WHITE);
        draw_rectangle(paddle2_x, paddle2_y, paddle_width, paddle_height, WHITE);

        // Draw Ball
        draw_circle(ball_x, ball_y , radius, WHITE);

        // Draw Stadium
        draw_rectangle(screen_width() / 2.0, 0.0, 1.0, screen_height(), WHITE);
        draw_circle_lines(screen_width() / 2.0, screen_height() / 2.0, 75.0, 1.0, WHITE);

        next_frame().await
    }
}
