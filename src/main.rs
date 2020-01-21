use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::mint::{Point2};
use ggez::{Context, GameResult};
use ggez::input::keyboard;
use ggez::event::{KeyCode, KeyMods};
use ggez::event;

mod tetromino;
use tetromino::Tetromino;

pub const grid_cols : usize = 20;
pub const grid_rows : usize = 30;
pub const cellsize : f32 = 25f32;
pub const tetromino_width : usize = 4;
pub const tetromino_height : usize = 4;

struct MainState {
	grid: [[i32; grid_rows]; grid_cols],
	block_mesh: graphics::Mesh,
	tetr: Tetromino,
	tetromino_fall_delay: i32,
	time: i32,
}

impl MainState {
	fn new(ctx: &mut Context) -> GameResult<MainState> {
		Ok(MainState{
			grid: [[0;grid_rows];grid_cols],
			block_mesh: graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(), 
				graphics::Rect{x:0f32,y:0f32,w:cellsize as f32, h:cellsize as f32},
				graphics::Color::from((12, 123, 213))
			).unwrap(),
			tetr: Tetromino::new(),
			tetromino_fall_delay: 20,
			time: 0,
		})
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		if self.time % self.tetromino_fall_delay == 0 {
			self.tetr.fall(&self.grid);
		}
		self.time+=1;
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		graphics::clear(ctx, graphics::WHITE);
		
		for x in 0..grid_cols {
			for y in 0..grid_rows {
				if self.grid[x][y] == 1 {
					graphics::draw(ctx, &self.block_mesh, (na::Point2::<f32>::new(x as f32 * cellsize as f32, y as f32 * cellsize as f32),));
				}
			}
		}

		for x in 0..tetromino_width {
			for y in 0..tetromino_height {
				if self.tetr.blocks[y][x] == 1 {
					let blockX = (self.tetr.pos.x + x as i32) as f32 * cellsize as f32;
					let blockY = (self.tetr.pos.y + y as i32) as f32 * cellsize as f32;
					graphics::draw(ctx, &self.block_mesh, (na::Point2::<f32>::new(blockX, blockY),));
				}
			}
		}


		graphics::present(ctx);

		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {

	}
}

fn main() -> GameResult {
	let window = ggez::conf::WindowSetup {
		title: "Tetris".to_owned(),
		samples: ggez::conf::NumSamples::Zero,
		vsync: false,
		icon: "".to_owned(),
		srgb: true,
	};

	let windowmode = ggez::conf::WindowMode {
		width: grid_cols as f32*cellsize,
		height: grid_rows as f32*cellsize,
		maximized: false,
		fullscreen_type: ggez::conf::FullscreenType::Windowed,
		borderless: false,
		min_width: 0.0,
		max_width: 0.0,
		min_height: 0.0,
		max_height: 0.0,
		resizable: false,
	};

	let conf = ggez::conf::Conf {
		window_mode: windowmode,
		window_setup: window,
		backend: ggez::conf::Backend::default(),
		modules: ggez::conf::ModuleConf::default(),
	};

	let cb = ggez::ContextBuilder::new("super_simple", "ggez").conf(conf);
	let (ref mut ctx, event_loop) = &mut cb.build()?;
	let state = &mut MainState::new(ctx)?;
	event::run(ctx, event_loop, state)
}
