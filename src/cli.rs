use clap::Parser;
use super::*;

/// A unique take on the classic game of Langtons Ant
#[derive(Parser)]
pub struct Settings{
  /// The width of the grid
  #[arg(short, long, default_value_t = DEFAULT_GRID_WIDTH)]
  pub width: usize,
  /// The height of the grid
  #[arg(short, long, default_value_t = DEFAULT_GRID_HEIGHT)]
  pub height: usize,
  /// The amount of iterations per frame
  #[arg(short, long, default_value_t = ITER_PER_FRAME)]
  pub iter: usize,
  /// The amount of states an ant can have
  #[arg(short, long, default_value_t = DEFAULT_STATE_COUNT)]
  pub state_count: u8,
}