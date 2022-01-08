pub mod window;

use sdl2::event::WindowEvent;
use vek::Extent2;

use crate::window::WindowWrapper;

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: Extent2<u32>,
}

impl State {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &WindowWrapper<'_>) -> Self {
        let size = Extent2::<u32>::from(window.0.size());

        // handle to our GPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // the frame buffer
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

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: Some("frame-buffer"),
                },
                None, // path for API tracing
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            // write to screen
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.w,
            height: size.h,
            // vsync
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: Extent2<u32>) {
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
