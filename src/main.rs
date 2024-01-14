#![allow(clippy::absurd_extreme_comparisons)]

use std::time::{Duration, Instant};

use game_of_life::{parse_goln, random_grid, res, step};
use grid::Grid;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::{event::Event, pixels::Color};

const RANDOM_GRID: bool = true;
const UPDATE_INTERVAL: Duration = Duration::from_millis(100);

const BORDER_SIZE: u32 = 1;
const BORDER_SIZE_I: i32 = BORDER_SIZE as i32;
const PIXEL_SIZE: u32 = 10;
const PIXEL_SIZE_I: i32 = PIXEL_SIZE as i32;

fn main() {
    let mut grid = if RANDOM_GRID {
        random_grid(80, 60)
    } else {
        parse_goln(res::GLIDER)
    };
    let mut last_update = Instant::now();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let cols = grid.cols() as u32;
    let rows = grid.rows() as u32;
    let width = cols as u32 * PIXEL_SIZE + cols * BORDER_SIZE + BORDER_SIZE;
    let height = rows as u32 * PIXEL_SIZE + rows * BORDER_SIZE + BORDER_SIZE;
    let window = video_subsystem
        .window("Game of Life", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut border_texture = texture_creator
        .create_texture_target(None, width, height)
        .unwrap();
    draw_border(&mut canvas, &mut border_texture, cols + 1, rows + 1);

    canvas.set_draw_color(Color::WHITE);
    do_draw(&mut canvas, &border_texture, &grid).expect("Failed first draw, terminating");

    let mut paused = false;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape | Keycode::Q),
                    ..
                } => break 'game_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => paused = !paused,
                _ => {}
            }
        }

        if !paused && last_update.elapsed() >= UPDATE_INTERVAL {
            grid = step(grid);

            if let Err(e) = do_draw(&mut canvas, &border_texture, &grid) {
                println!("Failed to draw, terminating: {e}");
                break 'game_loop;
            }

            last_update = Instant::now();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30))
    }
}

fn do_draw(
    canvas: &mut Canvas<Window>,
    border_texture: &Texture,
    grid: &Grid<bool>,
) -> Result<(), String> {
    if BORDER_SIZE > 0 {
        canvas.copy(border_texture, None, None)?;
    } else {
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
    }

    canvas.set_draw_color(Color::BLACK);
    for ((y, x), cell) in grid.indexed_iter() {
        let x = x as i32;
        let y = y as i32;

        if *cell {
            canvas.fill_rect(Rect::new(
                x * PIXEL_SIZE_I + x * BORDER_SIZE_I + BORDER_SIZE_I,
                y * PIXEL_SIZE_I + y * BORDER_SIZE_I + BORDER_SIZE_I,
                PIXEL_SIZE,
                PIXEL_SIZE,
            ))?;
        }
    }

    canvas.set_draw_color(Color::WHITE);
    canvas.present();

    Ok(())
}

fn draw_border(
    canvas: &mut Canvas<Window>,
    texture: &mut Texture,
    vertical_lines: u32,
    horizontal_lines: u32,
) {
    if BORDER_SIZE > 0 {
        let (w, h) = canvas.window().size();

        canvas
            .with_texture_canvas(texture, |texture_canvas| {
                texture_canvas.set_draw_color(Color::WHITE);
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::GRAY);

                for i in 0..vertical_lines {
                    texture_canvas
                        .fill_rect(Rect::new(
                            i as i32 * (BORDER_SIZE + PIXEL_SIZE) as i32,
                            0,
                            BORDER_SIZE,
                            h,
                        ))
                        .unwrap();
                }

                for i in 0..horizontal_lines {
                    texture_canvas
                        .fill_rect(Rect::new(
                            0,
                            i as i32 * (BORDER_SIZE + PIXEL_SIZE) as i32,
                            w,
                            BORDER_SIZE,
                        ))
                        .unwrap();
                }
            })
            .unwrap();
    }
}
