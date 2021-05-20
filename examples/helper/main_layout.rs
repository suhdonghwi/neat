use std::path::Path;

use ggez::conf::WindowSetup;
use ggez::graphics;
use ggez::nalgebra as na;

use neat::network::network_graph::NetworkGraph;

use super::fitness_plot::FitnessPlot;
use super::graph_visual::GraphVisual;

pub struct MainLayout {
    graph_visual: Option<GraphVisual>,
    fitness_plot: FitnessPlot,
    font: graphics::Font,
    max_weight: f64,
}

impl MainLayout {
    pub fn new(ctx: &mut ggez::Context, max_weight: f64) -> Self {
        let font = graphics::Font::new(ctx, Path::new("/LiberationMono-Regular.ttf")).unwrap();

        MainLayout {
            graph_visual: None,
            fitness_plot: FitnessPlot::new(
                [550.0, 300.0, 400.0, 300.0].into(),
                4.0,
                1.0,
                1.0,
                font,
            ),
            font,
            max_weight,
        }
    }

    pub fn builder() -> ggez::ContextBuilder {
        ggez::ContextBuilder::new("neat", "suhdonghwi")
            .window_mode(ggez::conf::WindowMode::default().dimensions(950.0, 600.0))
            .window_setup(WindowSetup::default().title("XOR"))
            .add_resource_path(Path::new("./resources"))
    }

    fn draw_separator(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let vertical = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(550.0, 0.0), na::Point2::new(550.0, 600.0)],
            3.0,
            graphics::Color::from_rgba(0, 0, 0, 50),
        )?;
        graphics::draw(ctx, &vertical, (na::Point2::new(0.0, 0.0),))?;

        let horizontal = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(550.0, 300.0), na::Point2::new(950.0, 300.0)],
            3.0,
            graphics::Color::from_rgba(0, 0, 0, 50),
        )?;
        graphics::draw(ctx, &horizontal, (na::Point2::new(0.0, 0.0),))
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(248, 249, 250));

        if let Some(graph) = &self.graph_visual {
            graph.draw(ctx)?;
        }

        self.fitness_plot.draw(ctx)?;

        self.draw_separator(ctx)
    }

    pub fn update(&mut self, graph: NetworkGraph, fitness: f64, generation: usize) {
        self.graph_visual = Some(GraphVisual::new(
            graph,
            [550.0, 0.0, 400.0, 300.0].into(),
            self.max_weight,
            generation,
            fitness,
            self.font,
        ));
        self.fitness_plot.add_data(fitness);
    }
}
