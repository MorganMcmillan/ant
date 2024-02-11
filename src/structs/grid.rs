use super::*;
pub type Grid = Vec<Vec<u8>>;

pub struct GridState {
    /// The color of a cell
    pub colors: Vec<Color>,
    /// If an ant should turn left or right
    pub rules: Vec<Rule>,
    /// How many states a cell can have
    pub state_count: u8,
}

impl GridState {
    pub fn new(rng: &mut ThreadRng, state_count: u8) -> GridState {
        GridState {
            rules: (0..state_count).map(|_| rng.gen()).collect(),
            colors: generate_colors(rng, state_count),
            state_count,
        }
    }
}