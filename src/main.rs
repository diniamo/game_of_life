#![allow(clippy::absurd_extreme_comparisons)]

use game_of_life::{parse_goln, random_grid, res, step, EdgeCaseMethod};
use grid::Grid;
use raylib::{
    color::Color, consts::KeyboardKey, drawing::RaylibTextureModeExt, ffi::TraceLogLevel,
    prelude::RaylibDraw, texture::RenderTexture2D, RaylibHandle, RaylibThread,
};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

const SLEEP_DURATION: Duration = Duration::from_nanos(1_000_000_000 / 30);
const UPDATE_INTERVAL: Duration = Duration::from_millis(100);

const RANDOM_GRID: bool = false;
const RG_WIDTH: usize = 80;
const RG_HEIGHT: usize = 60;

const BORDER_SIZE: i32 = 1;
const PIXEL_SIZE: i32 = 10;

const EDGE_CASE_METHOD: EdgeCaseMethod = EdgeCaseMethod::AssumeDead;

fn main() {
    let mut grid = if RANDOM_GRID {
        random_grid(RG_WIDTH, RG_HEIGHT)
    } else {
        parse_goln(res::GOSPER_GLIDER_GUN)
    };
    let mut last_update = Instant::now();

    let cols = grid.cols() as i32;
    let rows = grid.rows() as i32;
    let width = cols * PIXEL_SIZE + cols * BORDER_SIZE + BORDER_SIZE;
    let height = rows * PIXEL_SIZE + rows * BORDER_SIZE + BORDER_SIZE;

    let (mut rl, thread) = raylib::init()
        // What? The window is exactly 37 pixels taller than it should be for some reason.
        // This doesn't happen on the SDL backend, but the Rust bindings don't seem to support that(?).
        .size(width, height - 37)
        .log_level(TraceLogLevel::LOG_ERROR)
        .title("Game of Life")
        .build();

    let border_buffer = render_border(&mut rl, &thread, width, height, cols + 1, rows + 1);

    let mut paused = false;
    while !rl.window_should_close() {
        rl.poll_input_events();

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }

        if !paused && last_update.elapsed() >= UPDATE_INTERVAL && step(&mut grid, &EDGE_CASE_METHOD)
        {
            do_draw(&mut rl, &thread, &border_buffer, &grid);
            last_update = Instant::now();
        }

        sleep(SLEEP_DURATION);
    }
}

fn do_draw(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    border_buffer: &Option<RenderTexture2D>,
    grid: &Grid<bool>,
) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::WHITE);

    if let Some(border_buffer) = border_buffer {
        d.draw_texture(border_buffer, 0, 0, Color::WHITE);
    }

    for ((y, x), cell) in grid.indexed_iter() {
        let x = x as i32;
        let y = y as i32;

        if *cell {
            d.draw_rectangle(
                x * PIXEL_SIZE + x * BORDER_SIZE + BORDER_SIZE,
                y * PIXEL_SIZE + y * BORDER_SIZE + BORDER_SIZE,
                PIXEL_SIZE,
                PIXEL_SIZE,
                Color::BLACK,
            );
        }
    }
}

fn render_border(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    width: i32,
    height: i32,
    vertical_lines: i32,
    horizontal_lines: i32,
) -> Option<RenderTexture2D> {
    if BORDER_SIZE > 0 {
        let mut buffer = rl
            .load_render_texture(thread, width as u32, height as u32)
            .ok()?;

        let mut d = rl.begin_drawing(thread);
        let mut d = d.begin_texture_mode(thread, &mut buffer);

        for i in 0..vertical_lines {
            d.draw_rectangle(
                i * (BORDER_SIZE + PIXEL_SIZE),
                0,
                BORDER_SIZE,
                height,
                Color::BLACK,
            )
        }

        for i in 0..horizontal_lines {
            d.draw_rectangle(
                0,
                i * (BORDER_SIZE + PIXEL_SIZE),
                width,
                BORDER_SIZE,
                Color::BLACK,
            );
        }

        drop(d);

        Some(buffer)
    } else {
        None
    }
}
