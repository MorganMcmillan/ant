use raylib::prelude::*;
use rand::{rngs::ThreadRng, Rng};


pub mod ant;
pub mod rule;
use rule::Rule;
pub mod grid;
use grid::{Grid,GridState};
pub mod direction;

pub fn generate_colors(rng: &mut ThreadRng, state_count: u8) -> Vec<Color> {
    let mut colors = Vec::with_capacity(state_count as usize);
    colors.push(rcolor(
        rng.gen_range(0..16),
        rng.gen_range(0..16),
        rng.gen_range(0..16),
        255,
    ));
    colors.extend((0..state_count - 1).map(|_| rcolor(rng.gen(), rng.gen(), rng.gen(), 255)));
    return colors;
}
