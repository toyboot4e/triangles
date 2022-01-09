//! Draw triangle with [`wgpu`]

use sdl2::event::{Event, WindowEvent};

use in_wgpu::gfx::WindowWrapper;

fn main() -> Result<(), wgpu::SurfaceError> {
    env_logger::init();

    let ver = sdl2::version::version();
    log::info!("Linked with SDL {}.{}.{}", ver.major, ver.minor, ver.patch);

    let window = WindowWrapper::new(&mut |vid| {
        vid.window("Draw triangle with wgpu", 1280, 720)
            .position_centered()
            // NOTE: This is requred for `WindowWrapper::fb_size_*` to work as expected
            .allow_highdpi()
            .resizable()
            .build()
            .expect("Unable to create SDL window")
    });

    let mut pump = window
        .sdl
        .event_pump()
        .expect("Unable to create SDL event pump");

    let mut app = pollster::block_on(in_wgpu::app::App::new(&window));

    'running: loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(_w, _h) => {
                        app.gpu.on_resize(&window);
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        app.render()?;

        // super-dirty around 60 FPS
        std::thread::sleep(std::time::Duration::from_micros(1000 / 60));
    }

    Ok(())
}
