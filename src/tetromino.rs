use ggez::nalgebra as na;

pub struct Tetromino {
	pub pos: na::Point2<i32>,
	pub blocks: [[i32; crate::tetromino_width]; crate::tetromino_height],
}

impl Tetromino {
	pub fn fall(&mut self, grid: &[[i32; crate::grid_rows]; crate::grid_cols]) {
		if self.can_fall(grid) {
			self.pos.y += 1;
		}
	}

	pub fn new() -> Self {
		Self {
			pos: na::Point2::<i32>::new(0,0),
			blocks: [
				[1,1,1,0],
				[1,0,0,0],
				[0,0,0,0],
				[0,0,0,0],
			]
		}
	}

	pub fn can_fall(&self, grid: &[[i32; crate::grid_rows]; crate::grid_cols]) -> bool {
		for x in 0..crate::tetromino_height {
			for y in 0..crate::tetromino_width {
				if self.blocks[y][x] == 1 {
					let block_pos = (self.pos.x as usize + x,self.pos.y as usize + y);
					let below_pos = (block_pos.0, block_pos.1+1);
					if below_pos.1 >= crate::grid_rows || grid[below_pos.0][below_pos.1] == 1 {
						return false
					}
				}
			}
		}
		true
	}
}