#![recursion_limit = "512"]
mod helper;

use std::path::Path;
use std::time::Duration;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use ggez::timer;
use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

use helper::flappy::Bird;
use helper::{main_layout::MainLayout, plot::Axis};

struct MainState {
    layout: MainLayout,
    innov_record: InnovationRecord,
    pool: Pool<Feedforward>,
    birds: Vec<Bird>,
    population: usize,
    generation_start: Duration,
}

impl MainState {
    fn reset_birds(&mut self) {
        let mut birds = Vec::new();
        for _ in 0..self.population {
            let bird = Bird::new(na::Point2::new(100.0, 200.0), 0.0, 0.3);
            birds.push(bird);
        }

        self.birds = birds;
    }

    fn new(ctx: &mut ggez::Context) -> Self {
        let args = helper::cli::get_arguments();
        let params = helper::read_parameters_file("./params/flappy.toml");

        let mut innov_record = InnovationRecord::new(params.input_number, params.output_number);
        let pool = Pool::<Feedforward>::new(params.clone(), args.verbosity, &mut innov_record);

        let font = graphics::Font::new(ctx, Path::new("/LiberationMono-Regular.ttf")).unwrap();

        let layout = MainLayout::new(
            params.mutation.weight_max,
            Axis::new(1.0, 10.0, 2.0),
            Axis::new(0.0, 4.0, 1.0),
            font,
        );

        let mut state = MainState {
            innov_record,
            pool,
            layout,
            birds: Vec::new(),
            population: params.population,
            generation_start: Duration::new(0, 0),
        };

        state.reset_birds();
        state
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        for (i, bird) in self.birds.iter_mut().enumerate() {
            if bird.is_dead() {
                continue;
            } else if bird.rect().y < 0.0 || bird.rect().y + bird.rect().h >= 600.0 {
                let fitness = (timer::time_since_start(ctx) - self.generation_start).as_secs_f64();
                bird.kill(fitness);
            }

            let output = self
                .pool
                .activate_nth(i, &[bird.y_velocity().into(), 0.0, 0.0])
                .unwrap();

            if output[0] > 0.5 {
                bird.jump();
            }

            bird.update();
        }

        if self.birds.iter().all(|bird| bird.is_dead()) {
            let generation = self.pool.generation();
            let fitness_list: Vec<f64> = self
                .birds
                .iter()
                .map(|bird| bird.fitness().unwrap())
                .collect();

            let mut best_network = self
                .pool
                .evaluate(|i, network| network.evaluate(fitness_list[i]))
                .clone();
            let best_fitness = best_network.fitness().unwrap();

            self.layout
                .update(best_network.graph_mut(), best_fitness, generation);

            self.pool.evolve(&mut self.innov_record);
            self.reset_birds();

            self.generation_start = timer::time_since_start(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.layout.draw(ctx)?;

        for bird in &self.birds {
            if bird.is_dead() {
                continue;
            }

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
