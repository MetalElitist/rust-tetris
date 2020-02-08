use ggez::nalgebra as na;
use rand::RngCore;

pub struct Tetromino {
	pub pos: na::Point2<i32>,
	pub blocks: [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4],
	pub rotation: usize,
}

impl Tetromino {
	pub fn fall(&mut self, grid: &mut [[i32; crate::grid_rows]; crate::grid_cols]) -> bool { // has fallen?
		if self.can_fall(grid) {
			self.pos.y += 1;
			false
		} else {
			self.place_to_grid(grid);
			self.reset();
			true
		}
	}

	pub fn move_tetromino(&mut self, grid: &mut [[i32; crate::grid_rows]; crate::grid_cols], dir: i8) {
		if self.can_move(grid, dir) {
			self.pos.x += dir as i32;
		}
	}

	pub fn rotate(&mut self, grid: &[[i32; crate::grid_rows]; crate::grid_cols], dir: i8) {
		let next_rotation = rotate_value(self.rotation as i8 + dir, 0, 3);
		if self.can_rotate(grid, next_rotation as usize) {
			self.rotation = next_rotation as usize;
		}
	}

	pub fn new() -> Self {
		Self {
			pos: na::Point2::<i32>::new(0,0),
			blocks: Self::randomized_blocks(),
			rotation: 0,
		}
	}

	pub fn reset(&mut self) {
		self.blocks = Self::randomized_blocks();
		self.rotation = 0;
		self.pos.y = 0;
		self.pos.x = (crate::grid_cols/2 - 1) as i32;
	}

	pub fn randomized_blocks() ->  [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4] {
		let random = rand::thread_rng().next_u32() % 6;
		match random {
			0 => Self::create_L1(),
			1 => Self::create_L2(),
			2 => Self::create_T(),
			3 => Self::create_Z1(),
			4 => Self::create_Z2(),
			5 => Self::create_I(),
			x => [[[1; crate::tetromino_width]; crate::tetromino_height]; 4],
		}
	}

	pub fn can_fall(&self, grid: &[[i32; crate::grid_rows]; crate::grid_cols]) -> bool {
		for x in 0..crate::tetromino_width {
			for y in 0..crate::tetromino_height {
				if self.blocks[self.rotation][y][x] == 1 {
					let block_pos = (self.pos.x + x as i32,self.pos.y + y as i32);
					let below_pos = (block_pos.0, block_pos.1+1);
					if below_pos.1 >= crate::grid_rows as i32 || grid[below_pos.0 as usize][below_pos.1 as usize] == 1 {
						return false
					}
				}
			}
		}
		true
	}

	pub fn can_move(&self, grid: &[[i32; crate::grid_rows]; crate::grid_cols], dir: i8) -> bool {
		for x in 0..crate::tetromino_width {
			for y in 0..crate::tetromino_height {
				if self.blocks[self.rotation][y][x] == 1 {
					let check_block_pos = (self.pos.x as i8 + x as i8 + dir,self.pos.y as i8 + y as i8);
					if check_block_pos.0 < 0 || check_block_pos.0 >= crate::grid_cols as i8 || grid[check_block_pos.0 as usize][check_block_pos.1 as usize] == 1 {
						return false
					}
				}
			}
		}
		true
	}

	pub fn can_rotate(&self, grid: &[[i32; crate::grid_rows]; crate::grid_cols], rotation: usize) -> bool {
		for x in 0..crate::tetromino_width {
			for y in 0..crate::tetromino_height {
				if self.blocks[rotation][y][x] == 1 {
					let block_pos = (self.pos.x as i8 + x as i8, self.pos.y as i8 + y as i8);
					if block_pos.0 < 0 || block_pos.0 >= crate::grid_cols as i8 || block_pos.1 >= crate::grid_rows as i8 || 
						grid[block_pos.0 as usize][block_pos.1 as usize] == 1 {
						return false
					}
				}
			}
		}
		true
	}

	pub fn place_to_grid(&self, grid: &mut [[i32; crate::grid_rows]; crate::grid_cols]) {
		for x in 0..crate::tetromino_width {
			for y in 0..crate::tetromino_height {
				if self.blocks[self.rotation][y][x] == 1 {
					grid[(self.pos.x + x as i32) as usize][(self.pos.y + y as i32) as usize] = 1;
				}
			}
		}
	}

	pub fn create_L1() -> [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4] {
		[
			[
				[0,1,1,1],
				[0,1,0,0],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,1,1,0],
				[0,0,1,0],
				[0,0,1,0],
				[0,0,0,0],
			],
			[
				[0,0,0,1],
				[0,1,1,1],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[0,1,0,0],
				[0,1,1,0],
				[0,0,0,0],
			],
		]
	}

	pub fn create_L2() -> [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4] {
		[
			[
				[0,1,0,0],
				[0,1,1,1],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,1,1,0],
				[0,1,0,0],
				[0,1,0,0],
				[0,0,0,0],
			],
			[
				[0,1,1,1],
				[0,0,0,1],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,0,1,0],
				[0,0,1,0],
				[0,1,1,0],
				[0,0,0,0],
			],
		]
	}

	pub fn create_T() -> [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4] {
		[
			[
				[0,1,0,0],
				[1,1,1,0],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[0,1,1,0],
				[0,1,0,0],
				[0,0,0,0],
			],
			[
				[1,1,1,0],
				[0,1,0,0],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[1,1,0,0],
				[0,1,0,0],
				[0,0,0,0],
			],
		]
	}

	pub fn create_Z1() -> [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4] {
		[
			[
				[0,0,0,0],
				[0,0,1,1],
				[0,1,1,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[0,1,1,0],
				[0,0,1,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[0,1,1,0],
				[0,0,1,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[0,1,1,0],
				[0,0,1,0],
				[0,0,0,0],
			],
		]
	}

	pub fn create_Z2() -> [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4] {
		[
			[
				[0,0,0,0],
				[0,1,1,0],
				[0,0,1,1],
				[0,0,0,0],
			],
			[
				[0,0,1,0],
				[0,1,1,0],
				[0,1,0,0],
				[0,0,0,0],
			],
			[
				[0,0,0,0],
				[0,1,1,0],
				[0,0,1,1],
				[0,0,0,0],
			],
			[
				[0,0,1,0],
				[0,1,1,0],
				[0,1,0,0],
				[0,0,0,0],
			],
		]
	}

	pub fn create_I() -> [[[i32; crate::tetromino_width]; crate::tetromino_height]; 4] {
		[
			[
				[0,0,0,0],
				[1,1,1,1],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[0,1,0,0],
				[0,1,0,0],
				[0,1,0,0],
			],
			[
				[0,0,0,0],
				[1,1,1,1],
				[0,0,0,0],
				[0,0,0,0],
			],
			[
				[0,1,0,0],
				[0,1,0,0],
				[0,1,0,0],
				[0,1,0,0],
			],
		]
	}

}

pub fn rotate_value<T: PartialOrd>(value: T, min: T, max: T) -> T {
	if value < min {
		return max
	} else if value > max {
		return min
	}
	value
}