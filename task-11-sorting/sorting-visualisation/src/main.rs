/**
 * Sorting Visualisation
 * - Example Task 11
 * 
 * Partial example on selection sort.
 * Note that a complete example would also include
 * - insertion sort,
 * - merge sort, and
 * - an additional of your choice.
 * 
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2022-11-25
 */

use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};

use std::path;

use rand::seq::SliceRandom;
use rand::thread_rng;

const ALG_SPAN: usize = 20;

const GRID_WIDTH: usize = ALG_SPAN;
const GRID_HEIGHT: usize = 50;
const GRID_CELL_SIZE: usize = 10;

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_WIDTH as f32 * GRID_CELL_SIZE as f32,
    GRID_HEIGHT as f32 * GRID_CELL_SIZE as f32,
);

trait Sorter {
    fn new(list: &[Vec<usize>]) -> Self;
    fn sort(&mut self) -> &[Vec<usize>];
}

/// GUI logic and event implementation structure. 
struct AppState {
    colours: Vec<Color>,
    grid: Vec<Vec<usize>>,
    selection_step: usize,
    finished: bool,
    inner_step: usize,
    outer_step: usize
}

impl AppState {
    /// Initialise new shuffled grid.
    fn new() -> GameResult<AppState> {
        let grid = Self::generate_grid();

        let state = AppState {
            grid,
            colours: (0..GRID_HEIGHT).map(|_i| {
                let h: f32 = 360.0 * (_i as f32)/(GRID_HEIGHT as f32);
                let h_ = if h == 360.0 { 0.0 } else { h / 60.0 };
                let x = h_ - h_.floor();

                let q = 1.0 - x;
                let t = 1.0 - q;

                let (r, g, b) = match h_.floor() as usize {
                    0 => (1.0, t, 0.0),
                    1 => (q, 1.0, 0.0),
                    2 => (0.0, 1.0, t),
                    3 => (0.0, q, 1.0),
                    4 => (t, 0.0, 1.0),
                    5 => (1.0, 0.0, q),
                    _ => (0.0, 0.0, 0.0)
                };
                
                Color::new(r, g, b, 1.0)
            }).collect(),
            selection_step: 0,
            finished: false,
            inner_step: 0,
            outer_step: 0
        };

        Ok(state)
    }

    /// Get grid of format `vec![vec![usize; GRID_HEIGHT]; GRID_WIDTH]` representing a colour spectrum.
    fn generate_grid() -> Vec<Vec<usize>> {
        let mut random_gen = thread_rng();

        (0..GRID_WIDTH).map(|_| {
            let mut colours: Vec<usize> = (0..GRID_HEIGHT).collect();
            colours.shuffle(&mut random_gen);
            colours
        }).collect()
    }

    /// Perform a algorithm step.
    fn sort(&mut self) -> bool {
        let mut is_finished = false;
        
        // Selection sort behaviour.
        for _col in 0..self.grid.len() {
            if self.grid[_col][self.inner_step + 1] < self.grid[_col][self.inner_step] {
                self.grid[_col].swap(self.inner_step + 1, self.inner_step);
            }
        }

        // Increment step counters.
        if self.inner_step + 2 == GRID_HEIGHT - self.outer_step {
            self.outer_step += 1;
            self.inner_step = 0;

            if self.outer_step == GRID_HEIGHT {
                is_finished = true;
            }
        } else {
            self.inner_step += 1;
        }

        is_finished
    }
}

/// Implement each stage of the application event loop. 
impl event::EventHandler for AppState {

    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.sort() {
            self.grid = Self::generate_grid();
        }
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear interface with gray background colour.
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // Draw tiles.
        for _row in 0..GRID_SIZE.1 {
            for _col in 0..GRID_SIZE.0 {
                let rectangle = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new_i32(
                        ((_col) as i32) * (GRID_CELL_SIZE as i32),
                        ((_row) as i32) * (GRID_CELL_SIZE as i32),
                        GRID_CELL_SIZE as i32,
                        GRID_CELL_SIZE as i32,
                    ), self.colours[self.grid[_col][_row]])?;
                graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 }, ));
            }
        }

        // Render updated graphics.
        graphics::present(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    // Create configuration for GUI application.
    let context_builder = ggez::ContextBuilder::new("sorting-visualisation", "violaso")
        .window_setup(
            ggez::conf::WindowSetup::default()  
                .title("Sorting Visualisation")
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
                .resizable(false)
        );

    // Initialise configuration.
    let (context, event_loop) = &mut context_builder.build()?;

    // Create sorting state.
    let state = &mut AppState::new()?;

    // Run event loop.
    event::run(context, event_loop, state)
}