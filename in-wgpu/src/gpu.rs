use vek::Extent2;

use crate::window::WindowWrapper;

/// `wgpu` handles
#[derive(Debug)]
pub struct Gpu {
    /// Frame buffer
    pub(crate) surface: wgpu::Surface,
    /// Connection to a graphics device
    pub(crate) device: wgpu::Device,
    /// Command queue on the device
    pub(crate) queue: wgpu::Queue,
    /// Presentation parameters of the surface
    pub(crate) config: wgpu::SurfaceConfiguration,
    /// Current frame buffer size in pixels
    pub(crate) fb_size: Extent2<u32>,
}

impl Gpu {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &WindowWrapper) -> Self {
        let size = window.fb_size_u();

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
            fb_size: size,
        }
    }

    /// Updates the frame buffer
    ///
    /// - `new_size`: size not mulitplied by DPI scaling factor
    pub fn on_resize(&mut self, window: &WindowWrapper) {
        self.resize_raw(window.fb_size_u());
    }

    pub fn resize_raw(&mut self, new_size: Extent2<u32>) {
        assert!(
            new_size.w != 0 && new_size.h != 0,
            "resizing to zero size can panic the app"
        );

        if self.fb_size == new_size {
            return;
        }

        self.fb_size = new_size;
        self.config.width = new_size.w;
        self.config.height = new_size.h;
        self.surface.configure(&self.device, &self.config);
    }
}
