//! Draw triangle with `rokol`

use std::time::Duration;

use anyhow::{Error, Result};
use rokol::{
    gfx as rg,
    glue::sdl::{Init, WindowHandle},
};
use sdl2::event::Event;

use in_rokol::{
    gfx::{Shader, StaticMesh},
    runner, shaders,
};

#[derive(Debug)]
pub struct App {
    window: WindowHandle,
    /// Clears the frame color buffer on starting screen rendering pass
    pa: rg::PassAction,
    /// Triangle shader
    shd: Shader,
    /// Buffer for the triangle shader
    mesh: StaticMesh<shaders::TriangleVertex>,
}

impl App {
    pub fn new(window: WindowHandle) -> Self {
        // set up a triangle
        let verts: &[shaders::TriangleVertex] = &[
            // (vertex, color)
            ([0.0, 0.5, 0.5], [1.0, 0.0, 0.0, 1.0]).into(), // top
            ([0.5, -0.5, 0.5], [0.0, 1.0, 0.0, 1.0]).into(), // bottom right
            ([-0.5, -0.5, 0.5], [0.0, 0.0, 1.0, 1.0]).into(), // bottom left
        ];
        let indices: &[u16] = &[0, 1, 2];

        Self {
            window,
            pa: rg::PassAction::clear([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0]),
            shd: shaders::triangle(),
            mesh: StaticMesh::new_16(verts, indices),
        }
    }
}

impl App {
    pub fn on_event(&mut self, ev: &Event) {
        //
    }

    pub fn update(&mut self) {
        //
    }

    pub fn render(&mut self) {
        rg::begin_default_pass(&self.pa, 1280, 720);
        self.shd.apply_pip();
        self.mesh.draw_all();
        rg::end_pass();
    }

    pub fn end_frame(&mut self) {
        rg::commit();
        self.window.swap_window();
    }
}

// boilerplate
// -----------

fn main() -> Result<()> {
    env_logger::init();
    let (mut app, pump) = self::init()?;
    runner::run(pump, &mut app, self::on_event, self::on_frame);
    Ok(())
}

fn init() -> Result<(App, sdl2::EventPump)> {
    let window = Init {
        title: "Draw triangle with rokol".to_string(),
        w: 1280,
        h: 720,
        ..Default::default()
    }
    .init(|_b| {})
    .map_err(Error::msg)?;

    let pump = window.sdl.event_pump().map_err(Error::msg)?;
    let app = App::new(window);

    Ok((app, pump))
}

fn on_event(app: &mut App, ev: &Event) {
    app.on_event(ev);
}

fn on_frame(app: &mut App, dt: Duration) {
    app.update();
    app.render();
    app.end_frame();
}
