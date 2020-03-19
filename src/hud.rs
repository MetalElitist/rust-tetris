use ggez::graphics;
use ggez::nalgebra as na;

pub struct HUD {
	font: graphics::Font,
	level_label: graphics::Text,
	lines_label: graphics::Text,
}

impl HUD {
	pub fn new(ctx: &mut ggez::Context) -> Self {
		let font = graphics::Font::new(ctx, "/arial.ttf").unwrap();
		HUD {
			font,
			level_label: graphics::Text::new(graphics::TextFragment{
				text: String::from(""),
				color: Some(graphics::Color::from_rgb(200,200,200)),
				font: Some(font),
				scale: Some(graphics::Scale::uniform(30.0))
			}),
			lines_label: graphics::Text::new(graphics::TextFragment{
				text: String::from(""),
				color: Some(graphics::Color::from_rgb(200,200,200)),
				font: Some(font),
				scale: Some(graphics::Scale::uniform(30.0))
			}),
		}
	}

	pub fn draw(&mut self, ctx: &mut ggez::Context, pos: &na::Point2::<f32>, level: u16, lines: u16) {
		let f = self.level_label.fragments_mut();
		f[0].text = String::from(format!("Level: {}", level));
		graphics::draw(ctx, &self.level_label, (na::Point2::<f32>::new(pos.x + 10.0, pos.y + 0.0),));

		let f = self.lines_label.fragments_mut();
		f[0].text = String::from(format!("Lines: {}", lines));
		graphics::draw(ctx, &self.lines_label, (na::Point2::<f32>::new(pos.x + 10.0, pos.y + 500.0),));
	}
}