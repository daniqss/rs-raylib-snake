use std::{error::Error, time::Instant};

use rs_raylib_snake::{
    game::Game,
    input::Input,
    math::Vector2i,
    raylib::{self, Color},
    window::Window,
};

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;
const GRID_SIZE: i32 = 40;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Snake!");

    let window = Window::new(SCREEN_WIDTH, SCREEN_HEIGHT, "Rust Raylib Snake");

    let mut game = Game::new(Vector2i::new(SCREEN_WIDTH, SCREEN_HEIGHT) / GRID_SIZE);

    // Timings
    const FRAME_TIME: i64 = 10_000_000 / 60;
    let start_time = Instant::now();
    let mut previous_time = 0i64;
    let mut elapsed_time;
    let mut lag = 0i64;

    let mut game_over_countdown = 5.0f32;

    while !window.should_close() {
        if !game.game_over {
            Input::update();

            let current_time = Instant::now().duration_since(start_time).as_micros() as i64;
            elapsed_time = current_time - previous_time;

            previous_time = current_time;

            lag += elapsed_time;
            while lag >= FRAME_TIME {
                game.update();
                Input::clear();
                lag -= FRAME_TIME;
            }
        } else {
            game_over_countdown -= raylib::get_frame_time();
            if game_over_countdown <= 0.0 {
                break;
            }
        }

        raylib::begin_drawing();

        raylib::clear_background(Color::BLACK);

        game.render(GRID_SIZE);

        if game.game_over {
            raylib::draw_centered_text(
                "Game Over!",
                SCREEN_WIDTH / 2,
                (SCREEN_HEIGHT as f32 * 0.35f32) as i32,
                48,
                Color::WHITE,
            );
            raylib::draw_centered_text(
                format!("Final score: {}", game.score),
                SCREEN_WIDTH / 2,
                (SCREEN_HEIGHT as f32 * 0.35f32) as i32 + 64,
                28,
                Color::WHITE,
            );

            raylib::draw_centered_text(
                format!(
                    "Shutting down in {} seconds",
                    game_over_countdown.round() as i32
                ),
                SCREEN_WIDTH / 2,
                SCREEN_HEIGHT - 50,
                16,
                Color::new_rgb(80, 80, 80),
            );
        }

        raylib::end_drawing();
    }

    Ok(())
}
