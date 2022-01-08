use sdl2::event::Event;

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

    let mut event_pump = sdl.event_pump().expect("Unable to create SDL event pump");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}
