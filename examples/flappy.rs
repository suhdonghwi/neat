#![recursion_limit = "512"]
mod helper;

use std::path::Path;
use std::time::Duration;

use ggez::event;
use ggez::graphics;
use ggez::graphics::spritebatch;
use ggez::nalgebra as na;

use ggez::timer;
use neat::network::Network;
use neat::{innovation_record::InnovationRecord, network::feedforward::Feedforward, pool::Pool};

use helper::flappy::{Bird, PipePair};
use helper::{main_layout::MainLayout, plot::Axis};
use rand::Rng;

struct MainState {
    layout: MainLayout,

    innov_record: InnovationRecord,
    pool: Pool<Feedforward>,
    population: usize,
    generation_start: Duration,

    birds: Vec<Bird>,
    spritebatch: spritebatch::SpriteBatch,

    pipes: Vec<PipePair>,
    pipe_image: graphics::Image,
    current_pipe_index: usize,
    pipe_timer: Duration,
}

impl MainState {
    fn reset_birds(&mut self) {
        let mut birds = Vec::new();
        for _ in 0..self.population {
            let bird = Bird::new(na::Point2::new(70.0, 300.0), 0.0, 0.3);
            birds.push(bird);
        }

        self.birds = birds;
    }

    fn new_pipe(&self) -> PipePair {
        let mut rng = rand::thread_rng();
        PipePair::new(
            self.pipe_image.clone(),
            na::Point2::new(600.0, rng.gen_range(50.0..400.0)),
        )
    }

    fn reset_pipes(&mut self) {
        self.pipes = vec![self.new_pipe()];
        self.pipe_timer = Duration::new(0, 0);
        self.current_pipe_index = 0;
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

        let bird_image = graphics::Image::new(ctx, "/flappy/bird.png").unwrap();
        let batch = spritebatch::SpriteBatch::new(bird_image);

        let mut state = MainState {
            innov_record,
            pool,
            layout,
            population: params.population,
            generation_start: Duration::new(0, 0),

            birds: Vec::new(),
            spritebatch: batch,

            pipes: Vec::new(),
            pipe_image: graphics::Image::new(ctx, "/flappy/pipe.png").unwrap(),
            current_pipe_index: 0,
            pipe_timer: Duration::new(0, 0),
        };

        state.reset_birds();
        state.reset_pipes();
        state
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.pipe_timer += timer::delta(ctx);
        if self.pipe_timer >= Duration::from_secs_f64(1.9) {
            self.pipes.push(self.new_pipe());
            self.pipe_timer = Duration::new(0, 0);
        }

        for pipe_pair in &mut self.pipes {
            pipe_pair.update();
        }

        if self.pipes[self.current_pipe_index].past(self.birds[0].rect().x) {
            self.current_pipe_index += 1;
        }

        let current_pipe = &self.pipes[self.current_pipe_index];
        for (i, bird) in self.birds.iter_mut().enumerate() {
            if bird.is_dead() {
                continue;
            } else if bird.rect().top() < 0.0
                || bird.rect().bottom() >= 600.0
                || current_pipe.overlaps(&bird.rect())
            {
                let fitness = (timer::time_since_start(ctx) - self.generation_start).as_secs_f64();
                bird.kill(fitness);
            }

            let output = self
                .pool
                .activate_nth(
                    i,
                    &[
                        bird.y_velocity().into(),
                        (current_pipe.upper_rect().bottom() - bird.rect().top()).into(),
                        (bird.rect().bottom() - current_pipe.lower_rect().top()).into(),
                    ],
                )
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
            self.reset_pipes();

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

            self.spritebatch.add(bird.draw_param());
            bird.draw(ctx)?;
        }

        graphics::draw(ctx, &self.spritebatch, (na::Point2::new(0.0, 0.0),))?;

        for pipe_pair in &self.pipes {
            pipe_pair.draw(ctx)?;
        }

        self.spritebatch.clear();
        graphics::present(ctx)
    }
}

pub fn main() -> ggez::GameResult {
    let cb = MainLayout::builder("Flappy Bird");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx);

    event::run(ctx, event_loop, state)
}
