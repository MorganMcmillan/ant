/* keybindings:
    s: take screenshot
    r: reset the grid with new rules
    e: exterminates all ants
    c: change colors
    d: spawn ant in the middle of the screen
    w: write the state to the console
    o: outputs the state into the clipboard
    left click: spawn several ants at the cursor's position
    right click: spawn only one ant at the cursor's position
*/

mod structs;
use structs::*;
mod cli;
use ant::Ant;
use grid::{Grid, GridState};
use direction::Direction;

use rand::Rng;
use raylib::prelude::*;
use KeyboardKey::*;
use clap::*;

// constants
const DEFAULT_GRID_WIDTH: usize = 512;
const DEFAULT_GRID_HEIGHT: usize = 288;
const ITER_PER_FRAME: usize = 1000;
const DEFAULT_STATE_COUNT: u8 = 8;
const STARTING_DIRECTION: Direction = Direction::Up;

fn main() {

    let settings = cli::Settings::parse();

    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("Langton's Ant")
        .resizable()
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut rng = rand::thread_rng();

    let mut ants = vec![Ant {
        x: settings.width >> 1,
        y: settings.height >> 1,
        direction: Direction::Up,
    }];
    let mut grid: Grid = vec![vec![0; settings.height]; settings.width];
    let mut state = GridState::new(&mut rng, DEFAULT_STATE_COUNT);

    let mut is_paused = false;

    while !rl.window_should_close() {
        let size_x = rl.get_screen_width() as f32 / settings.width as f32;
        let size_y = rl.get_screen_height() as f32 / settings.height as f32;

        if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
            ants.push(Ant {
                x: (rl.get_mouse_x() as f32 / size_x) as usize,
                y: (rl.get_mouse_y() as f32 / size_y) as usize,
                direction: STARTING_DIRECTION,
            })
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_RIGHT_BUTTON) {
            ants.push(Ant {
                x: (rl.get_mouse_x() as f32 / size_x) as usize,
                y: (rl.get_mouse_y() as f32 / size_y) as usize,
                direction: STARTING_DIRECTION,
            })
        }

        // Fullscreen
        if rl.is_key_pressed(KEY_F11) {
            rl.toggle_fullscreen()
        }

        // RESET
        if rl.is_key_pressed(KEY_R) {
            ants.clear();
            grid = vec![vec![0; settings.height]; settings.width];
            //state.rules = (0..state.state_count).map(|_| rng.gen()).collect();
            state = GridState::new(&mut rng, DEFAULT_STATE_COUNT);
        }

        // Spawn ant in default position
        if rl.is_key_pressed(KEY_D) {
            ants.push(Ant {
                x: settings.width >> 1,
                y: settings.height >> 1,
                direction: Direction::Up,
            })
        }

        // change colors
        if rl.is_key_pressed(KEY_C) {
            state.colors = generate_colors(&mut rng, state.state_count)
        }

        // Exterminate all ants
        if rl.is_key_pressed(KEY_E){
            ants.clear();
            grid = vec![vec![0; settings.height]; settings.width];
        }

        is_paused ^= rl.is_key_pressed(KEY_P);

        if !is_paused {
            for _ in 0..ITER_PER_FRAME {
                for ant in ants.iter_mut() {
                    ant.update(&mut grid, &state, &settings);
                }
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(rcolor(0, 127, 127, 0));

        // Draw the grid
        for x in 0..settings.width {
            for y in 0..settings.height {
                d.draw_rectangle_v(
                    rvec2(x as f32 * size_x, y as f32 * size_y),
                    rvec2(size_x, size_y),
                    state.colors[grid[x][y] as usize],
                );
            }
        }

        // Draw ant
        for ant in ants.iter() {
            d.draw_rectangle_v(
                rvec2(ant.x as f32 * size_x, ant.y as f32 * size_y),
                rvec2(size_x, size_y),
                Color::RED,
            );
        }

        drop(d);

        // Take screenshot
        if rl.is_key_pressed(KEY_S) {
            rl.take_screenshot(&thread, &format!("sc_{}.png", rng.gen::<u32>()))
        }
    }
}
