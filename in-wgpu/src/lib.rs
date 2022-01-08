use sdl2::{event::WindowEvent, video::Window};
use vek::Vec2;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: Vec2<u32>,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self {
        let size = Vec2::<u32>::from(window.size());

        // handle to our GPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };

        // handle to graphics card
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                // LowPower | HighPerformance
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        todo!()
    }

    pub fn resize(&mut self, new_size: Vec2<u32>) {
        todo!()
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    pub fn update(&mut self) {
        todo!()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}
