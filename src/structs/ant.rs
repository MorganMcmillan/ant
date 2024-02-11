use crate::cli;

use super::*;
use direction::Direction;
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}

impl Ant {
    pub fn update(&mut self, grid: &mut Grid, state: &GridState, settings: &cli::Settings) {
        if self.x > usize::MAX >> 1 {
            self.x = settings.width - 1
        }
        if self.y > usize::MAX >> 1 {
            self.y = settings.height - 1
        }
        self.x %= settings.width;
        self.y %= settings.height;
        let cell = &mut grid[self.x][self.y];
        match state.rules[*cell as usize] {
            Rule::TurnLeft=>self.direction.turn_left(),
            Rule::TurnRight=>self.direction.turn_right(),
            Rule::TurnBack=>self.direction.turn_back(),
            Rule::Forward=>{}
        }
        *cell = (*cell + 1) % state.state_count;
        use Direction::*;
        match self.direction {
            Up => self.y -= 1,
            Right => self.x += 1,
            Down => self.y += 1,
            Left => self.x -= 1,
        }
    }
}
