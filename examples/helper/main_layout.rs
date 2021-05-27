use std::path::Path;

use ggez::conf::WindowSetup;
use ggez::graphics;
use ggez::nalgebra as na;

use neat::network::network_graph::NetworkGraph;

use super::{graph_visual::GraphVisual, plot::Axis};
use super::{opencolor, plot::Plot};

pub struct MainLayout {
    graph_visual: Option<GraphVisual>,
    fitness_plot: Plot,
    font: graphics::Font,
    max_weight: f64,

    fitness_points: Vec<na::Point2<f32>>,
}

impl MainLayout {
    pub fn new(weight_max: f64, x_axis: Axis, y_axis: Axis, font: graphics::Font) -> Self {
        MainLayout {
            graph_visual: None,
            fitness_plot: Plot::new(
                [550.0, 300.0, 400.0, 300.0].into(),
                x_axis,
                y_axis,
                "fitness-generation graph",
                font,
            ),
            font,
            max_weight: weight_max,
            fitness_points: Vec::new(),
        }
    }

    pub fn builder(window_title: &str) -> ggez::ContextBuilder {
        ggez::ContextBuilder::new("neat", "suhdonghwi")
            .window_mode(ggez::conf::WindowMode::default().dimensions(950.0, 600.0))
            .window_setup(WindowSetup::default().title(window_title))
            .add_resource_path(Path::new("./resources"))
    }

    fn draw_separator(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let vertical = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(550.0, 0.0), na::Point2::new(550.0, 600.0)],
            3.0,
            opencolor::with_alpha(*opencolor::BLACK, 0.2),
        )?;
        graphics::draw(ctx, &vertical, (na::Point2::new(0.0, 0.0),))?;

        let horizontal = graphics::Mesh::new_line(
            ctx,
            &[na::Point2::new(550.0, 300.0), na::Point2::new(950.0, 300.0)],
            3.0,
            opencolor::with_alpha(*opencolor::BLACK, 0.2),
        )?;
        graphics::draw(ctx, &horizontal, (na::Point2::new(0.0, 0.0),))
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, *opencolor::GRAY0);

        if let Some(graph) = &self.graph_visual {
            graph.draw(ctx)?;
        }

        self.fitness_plot
            .draw_plane(ctx, |x| format!("{}", x), |y| format!("{:.2}", y))?;

        if self.fitness_points.len() > 1 {
            self.fitness_plot.start_plotting();
            self.fitness_plot
                .draw_line(&self.fitness_points, *opencolor::INDIGO5)?;
            self.fitness_plot.finish_plotting(ctx)?;
        }

        self.draw_separator(ctx)
    }

    pub fn update(&mut self, graph: &mut NetworkGraph, fitness: f64, generation: usize) {
        self.graph_visual = Some(GraphVisual::new(
            graph,
            [550.0, 0.0, 400.0, 300.0].into(),
            self.max_weight,
            generation,
            fitness,
            self.font,
        ));

        self.fitness_points
            .push(na::Point2::new(generation as f32, fitness as f32));

        let points_count = self.fitness_points.len();
        let max_points: usize = 40;
        let tick_count: usize = 4;
        let min = if points_count <= max_points {
            1
        } else {
            points_count - max_points
        };
        let max = self.fitness_points.len();
        let delta = ((max - min) as f32 / tick_count as f32).ceil();

        self.fitness_plot
            .x_axis_mut()
            .set_range(min as f32, max as f32, delta);
    }
}
