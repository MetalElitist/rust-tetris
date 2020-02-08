// Delayed auto shift

use ggez::graphics;

pub struct DAS {
	pub active: bool,
	pub activating: bool,
	pub moving: bool,
	pub moved: bool,
	pub ticks_to_activate: i32,
	pub ticks_left_to_activate: i32,
	pub ticks_to_move: i32,
	pub ticks_left_to_move: i32,
	pub need_move: bool,
	pub side: i8,
	pub new_tetromino: bool,
}

impl DAS {
	pub fn new() -> DAS {
		DAS {
			active: false,
			activating: false,
			moving: false,
			moved: false,
			ticks_to_activate: 40,
			ticks_left_to_activate:40,
			ticks_to_move: 9,
			ticks_left_to_move: 9,
			need_move: false,
			side: 0,
			new_tetromino: false,
		}
	}

	pub fn tick(&mut self) {
		if self.activating {
			self.ticks_left_to_activate -= 1;
			if self.ticks_left_to_activate < 0 {
				self.active = true;
				self.activating = false;
				self.ticks_left_to_move = self.ticks_to_move;
			}
		}
		if self.active && self.moving {
			// self.need_move = false;
			self.ticks_left_to_move -= 1;
			if self.ticks_left_to_move < 0 {
				self.need_move = true;
				self.ticks_left_to_move = self.ticks_to_move;
			}
		}
	}

	pub fn start_moving(&mut self, side: i8) {
		if (self.side != side && !self.new_tetromino) || !self.new_tetromino {
			self.active = false;
			self.ticks_left_to_activate = self.ticks_to_activate;
			self.ticks_left_to_move = self.ticks_left_to_move;
		}
		self.side = side;
		if !self.active {
			self.activating = true;
		}
		self.moving = true;
		self.new_tetromino = false;
	}

	pub fn stop_moving(&mut self) {
		self.activating = false;
		self.moving = false;
		self.need_move = false;
		// if self.moved {
		// 	self.active = false;
		// 	self.ticks_left_to_move = self.ticks_left_to_move;
		// 	self.ticks_left_to_activate = self.ticks_to_activate
		// }
	}

	pub fn new_tetromino(&mut self) {
		self.new_tetromino = true;
	}

}

pub struct DAS_DEBUG {

}

impl DAS_DEBUG {
	pub fn debug_mesh(ctx: &mut ggez::Context, das: &DAS, cellsize: f32) -> ggez::graphics::Mesh {
		let mut red = 355 - das.ticks_left_to_activate as i32;
		if red < 100 {
			red = 100;
		}
		let mut green = 0;
		if das.active {
			green = 255;
		}
		let mut blue = 0;
		if das.activating {
			blue = 255;
		}

		graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect{x:0.,y:0.,w:cellsize,h:cellsize}, graphics::Color::from((red as u8,green as u8,blue as u8))).unwrap()
	}
}