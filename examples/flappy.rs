#![recursion_limit = "512"]
mod helper;

use std::path::Path;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

use helper::{main_layout::MainLayout, opencolor, plot::Axis};

struct Bird {
    pos: na::Point2<f32>,
    y_velocity: f32,
    y_accel: f32,
}

impl Bird {
    fn new(pos: na::Point2<f32>, velocity: f32, accel: f32) -> Self {
        Bird {
            pos,
            y_velocity: velocity,
            y_accel: accel,
        }
    }

    fn update(&mut self) {
        self.pos.y += self.y_velocity;
        self.y_velocity += self.y_accel;
    }

    fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let rect = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.pos,
            30.0,
            0.3,
            *opencolor::GRAY5,
        )?;

        graphics::draw(ctx, &rect, (na::Point2::new(0.0, 0.0),))
    }
}

struct MainState {
    layout: MainLayout,
    innov_record: InnovationRecord,
    pool: Pool<Feedforward>,
    birds: Vec<Bird>,
}

impl MainState {
    fn new(ctx: &mut ggez::Context) -> Self {
        let args = helper::cli::get_arguments();
        let params = helper::read_parameters_file("./params/flappy.toml");

        let mut innov_record = InnovationRecord::new(3, 2);
        let pool = Pool::<Feedforward>::new(params.clone(), args.verbosity, &mut innov_record);

        let font = graphics::Font::new(ctx, Path::new("/LiberationMono-Regular.ttf")).unwrap();

        let layout = MainLayout::new(
            params.mutation.weight_max,
            Axis::new(1.0, 10.0, 2.0),
            Axis::new(0.0, 4.0, 1.0),
            font,
        );

        let mut birds = Vec::new();
        for i in 0..params.population {
            let bird = Bird::new(na::Point2::new(100.0, 200.0), 0.0, i as f32 * 0.01);
            birds.push(bird);
        }

        MainState {
            innov_record,
            pool,
            layout,
            birds,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        for bird in &mut self.birds {
            bird.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.layout.draw(ctx)?;

        for bird in &self.birds {
            bird.draw(ctx)?;
        }

        graphics::present(ctx)
    }
}

pub fn main() -> ggez::GameResult {
    let cb = MainLayout::builder("Flappy Bird");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx);

    event::run(ctx, event_loop, state)
}
