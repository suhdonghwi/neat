use std::path::Path;

use ggez::graphics;
pub struct Text {
    text: graphics::Text,
}

impl Text {
    pub fn new(ctx: &mut ggez::Context, text: &str, scale: f32) -> Self {
        let font = graphics::Font::new(ctx, Path::new("/LiberationMono-Regular.ttf")).unwrap();
        let mut text = graphics::Text::new(text);
        text.set_font(font, graphics::Scale::uniform(scale));

        Text { text }
    }

    pub fn width(&self, ctx: &mut ggez::Context) -> u32 {
        self.text.width(ctx)
    }

    pub fn draw(
        &self,
        ctx: &mut ggez::Context,
        point: ggez::nalgebra::Point2<f32>,
        color: graphics::Color,
    ) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.text,
            graphics::DrawParam::default()
                .dest(point)
                .scale([0.5, 0.5])
                .color(color),
        )
    }
}
