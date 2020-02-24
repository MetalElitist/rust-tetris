use ggez;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::mint::{Point2};
use ggez::{Context, GameResult};
use ggez::input::keyboard;
use ggez::event::{KeyCode, KeyMods};
use ggez::event;
use ggez::timer;

use std::time;
use std::time::SystemTime;

mod tetromino;
mod das;
use tetromino::Tetromino;
use das::DAS;

pub const grid_cols : usize = 12;
pub const grid_rows : usize = 30;
pub const cellsize : f32 = 25f32;
pub const tetromino_width : usize = 4;
pub const tetromino_height : usize = 4;

struct MainState {
	grid: [[i32; grid_rows]; grid_cols],
	block_mesh: graphics::Mesh,
	block_mesh2: graphics::Mesh,
	clear_mesh: graphics::Mesh,
	tetr: Tetromino,
	das: DAS,
	level: u16,
	lines: u16,
	tetromino_fall_delay: u32,
	tetromino_fall_delay_devider: f32,
	tetromino_fall_delay_normal_devider: f32,
	tetromino_descreasing_fall_delay_devider: f32,
	tetromino_normal_fall_delay: u32,
	last_update_time: u128,
	need_redraw_all: bool,
	pressed_down: bool,
	left_pressed: bool,
	right_pressed: bool,
}

impl MainState {
	fn new(ctx: &mut Context) -> GameResult<MainState> {
		Ok(MainState{
			grid: [[0;grid_rows];grid_cols],
			block_mesh: graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(), 
				graphics::Rect{x:2f32,y:2f32,w:cellsize as f32 - 2.0, h:cellsize as f32 - 2.0},
				graphics::Color::from((12, 123, 213))
			).unwrap(),
			block_mesh2: graphics::Mesh::new_circle(
				ctx, 
				graphics::DrawMode::fill(), 
				graphics::mint::Point2{x:5f32,y:5f32},
				2f32,
				1f32,
				graphics::Color::from((255, 255, 254))
			).unwrap(),
			clear_mesh: graphics::Mesh::new_rectangle(
				ctx, 
				graphics::DrawMode::fill(), 
				graphics::Rect{x:0.,y:0.,w:cellsize as f32, h:cellsize as f32},
				graphics::BLACK
			).unwrap(),
			tetr: Tetromino::new(),
			das: DAS::new(),
			level: 1,
			lines: 0,
			tetromino_fall_delay: 60000,
			tetromino_fall_delay_devider: 8.0,
			tetromino_fall_delay_normal_devider: 8.0,
			tetromino_descreasing_fall_delay_devider: 0.3,
			tetromino_normal_fall_delay: 60000,
			last_update_time: 0,
			need_redraw_all: true,
			pressed_down: false,
			left_pressed: false,
			right_pressed: false,
		})
	}
}

impl MainState {
	fn check_rows(&self) -> [(usize, bool);4] { // Первый элемент - строка, второй - заполненная она или нет
		let mut rowsinfo: [(usize, bool);4] = [(0,false);4];
		let mut num_filled_rows = 0;
		for y in 0..grid_rows {
			rowsinfo[num_filled_rows].0 = y;
			rowsinfo[num_filled_rows].1 = true;
			for x in 0..grid_cols {
				if self.grid[x][y] == 0 {
					rowsinfo[num_filled_rows].1 = false;
					break;
				}
			}
			if rowsinfo[num_filled_rows].1 {
				num_filled_rows+=1;
			}
		}
		rowsinfo
	}

	fn clear_row(&mut self, row: usize) {
		for y in 0..grid_cols {
			self.grid[y][row] = 0;
		}
		self.lines += 1;
		println!("lines: {}", self.lines);
		if (self.lines/2 >= self.level && self.lines % 2 == 0) {
			self.level += 1;
			println!("new level {}", self.level);
		}
	}

	fn lower_above(&mut self, row: usize) { // Опускает все вышестоящие строки начиная с row
		// println!("{}", row);
		for y in (1..row+1).rev() {
			// println!("row {}", y);
			for x in 0..grid_cols {
				// print!("c: {}, v: {} ", x, self.grid[x as usize][y as usize]);
				self.grid[x as usize][y as usize] = self.grid[x as usize][(y - 1) as usize];
			}
		}
	}
}

impl event::EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {

		self.das.tick();
		if self.das.need_move {
			self.das.moved = self.tetr.move_tetromino(&mut self.grid, self.das.side);
			if self.das.moved {
				self.das.need_move = false;
			}
		}
		// println!("{}", self.das.new_tetromino);

		let fall_delay = (self.tetromino_fall_delay as f32/(self.level as f32/self.tetromino_fall_delay_devider)) as i128;

		let delta = (SystemTime::now() - timer::time_since_start(ctx)).elapsed().unwrap().as_micros() as i128 - self.last_update_time as i128;
		if delta > fall_delay {
			self.last_update_time = (SystemTime::now() - timer::time_since_start(ctx)).elapsed().unwrap().as_micros();// + (delta as u128 - self.tetromino_fall_delay as u128);
			
			if self.tetr.fall(&mut self.grid) {
				self.das.new_tetromino();
				self.need_redraw_all = true;
			} else {
				self.das.fall();
			}
			let rowsinfo = self.check_rows();
			for row in rowsinfo.iter() {
				if row.1 {
					self.clear_row(row.0);
					self.lower_above(row.0);
				}
			}
		}

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut draw_region = (self.tetr.pos.x - 1, self.tetr.pos.y - 1, self.tetr.pos.x + 5, self.tetr.pos.y+4);
		if self.need_redraw_all {
			draw_region = (0, 0, grid_cols as i32, grid_rows as i32);
			self.need_redraw_all = false;
		}
		if self.pressed_down {
			draw_region.1 = 0;
		}
		
		for x in draw_region.0..draw_region.2 {
			for y in draw_region.1..draw_region.3 {
				if x as usize >= grid_cols || y as usize >= grid_rows || x < 0 || y < 0 {
					continue;
				}
				if self.grid[x as usize][y as usize] == 1 {
					graphics::draw(ctx, &self.block_mesh, (na::Point2::<f32>::new(x as f32 * cellsize as f32, y as f32 * cellsize as f32),));
					graphics::draw(ctx, &self.block_mesh2,(na::Point2::<f32>::new(x as f32 * cellsize as f32, y as f32 * cellsize as f32),));
				} else {
					graphics::draw(ctx, &self.clear_mesh, (na::Point2::<f32>::new(x as f32 * cellsize as f32, y as f32 * cellsize as f32),));
				}
			}
		}

		let mut tetromino_mesh = &self.block_mesh;
		// let debug_mesh = das::DAS_DEBUG::debug_mesh(ctx, &self.das, cellsize as f32);
		// tetromino_mesh = &debug_mesh;

		for x in 0..tetromino_width {
			for y in 0..tetromino_height {
				if self.tetr.blocks[self.tetr.rotation][y][x] == 1 {
					let blockX = (self.tetr.pos.x + x as i32) as f32 * cellsize as f32;
					let blockY = (self.tetr.pos.y + y as i32) as f32 * cellsize as f32;
					graphics::draw(ctx, tetromino_mesh, (na::Point2::<f32>::new(blockX, blockY),));
					graphics::draw(ctx, &self.block_mesh2, (na::Point2::<f32>::new(blockX, blockY),));
				}
			}
		}

		graphics::present(ctx);
		timer::yield_now();

		Ok(())
	}

	fn key_down_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
		match key {
			KeyCode::Left => {
				if !self.das.moving {
					self.das.need_move = true;
				}
				if !self.left_pressed {
					self.das.start_moving(-1);
				}
				self.left_pressed = true;
			},
			KeyCode::Right => {
				if !self.das.moving {
					self.das.need_move = true;
				}
				if !self.right_pressed {
					self.das.start_moving(1);
				}
				self.right_pressed = true;
			},
			KeyCode::A => self.tetr.rotate(&self.grid, -1),
			KeyCode::S => self.tetr.rotate(&self.grid, 1),
			KeyCode::Down => {
				// self.tetromino_fall_delay = self.tetromino_normal_fall_delay - self.tetromino_decreasing_fall_delay;
				self.tetromino_fall_delay_devider = self.tetromino_descreasing_fall_delay_devider;
				self.pressed_down = true;
			},
			_ => (),
		};
	}

	fn key_up_event(&mut self, ctx: &mut Context, key: KeyCode, mods: KeyMods) {
		match key {
			KeyCode::Left => {
				self.das.stop_moving();
				self.left_pressed = false;
			},
			KeyCode::Right => {
				self.das.stop_moving();
				self.right_pressed = false;
			},
			KeyCode::Down => {
				// self.tetromino_fall_delay = self.tetromino_normal_fall_delay;
				self.tetromino_fall_delay_devider = self.tetromino_fall_delay_normal_devider;
				self.pressed_down = false;
			},
			_ => (),
		};
	}

	
}

fn main() -> GameResult {
	let window = ggez::conf::WindowSetup {
		title: "Tetris".to_owned(),
		samples: ggez::conf::NumSamples::One,
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
	for x in 0..grid_cols - 1 {
		state.grid[x][grid_rows-1] = 1;
	}
	println!("{}", SystemTime::now().elapsed().unwrap().as_micros());
	event::run(ctx, event_loop, state)
}
