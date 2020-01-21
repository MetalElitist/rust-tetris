use ggez::nalgebra as na;

pub struct Tetromino {
	pub pos: na::Point2<i32>,
	pub blocks: [[i32; crate::tetromino_width]; crate::tetromino_height],
}

impl Tetromino {
	pub fn fall(&mut self, grid: &mut [[i32; crate::grid_rows]; crate::grid_cols]) {
		if self.can_fall(grid) {
			self.pos.y += 1;
		} else {
			grid[self.pos.x as usize][self.pos.y as usize] = 1;
			self.place_to_grid(grid);
			self.pos.y = 0;
		}
	}

	pub fn move_tetromino(&mut self, grid: &mut [[i32; crate::grid_rows]; crate::grid_cols], dir: i8) {
		if self.can_move(grid, dir) {
			self.pos.x += dir as i32;
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
		for x in 0..crate::tetromino_width {
			for y in 0..crate::tetromino_height {
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

	pub fn can_move(&self, grid: &[[i32; crate::grid_rows]; crate::grid_cols], dir: i8) -> bool {
		println!("can move");
		for x in 0..crate::tetromino_width {
			for y in 0..crate::tetromino_height {
				if self.blocks[y][x] == 1 {
					let check_block_pos = (self.pos.x as i8 + x as i8 + dir,self.pos.y as i8 + y as i8);
					println!("{:?}", check_block_pos);
					if check_block_pos.0 < 0 || check_block_pos.0 >= crate::grid_cols as i8 || grid[check_block_pos.0 as usize][check_block_pos.1 as usize] == 1 {
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
				if self.blocks[y][x] == 1 {
					grid[self.pos.x as usize + x][self.pos.y as usize + y] = 1;
				}
			}
		}
	}

}