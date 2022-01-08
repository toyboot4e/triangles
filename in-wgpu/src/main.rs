//! Draw triangle with [`wgpu`]

use sdl2::event::Event;

use in_wgpu::window::WindowWrapper;

fn main() {
    env_logger::init();

    let sdl = sdl2::init().unwrap();
    let vid = sdl.video().unwrap();

    let window = vid
        .window("Draw triangle with wgpu", 1280, 720)
        .position_centered()
        .resizable()
        .build()
        .expect("Unable to create SDL window");
    let window = WindowWrapper(&window);

    let mut pump = sdl.event_pump().expect("Unable to create SDL event pump");
    let mut state = pollster::block_on(in_wgpu::State::new(&window));

    'running: loop {
        for event in pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}
