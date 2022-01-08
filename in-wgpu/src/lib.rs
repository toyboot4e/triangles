pub mod window;

use vek::Extent2;

use crate::window::WindowWrapper;

/// `wgpu` handles
#[derive(Debug)]
pub struct Gpu {
    /// Frame buffer
    surface: wgpu::Surface,
    /// Connection to a graphics device
    device: wgpu::Device,
    /// Command queue on the device
    queue: wgpu::Queue,
    /// Presentation parameters of the surface
    config: wgpu::SurfaceConfiguration,
    /// Current frame buffer size in pixels
    size: Extent2<u32>,
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
            size,
        }
    }

    /// Updates the frame buffer
    ///
    /// - `new_size`: size not mulitplied by DPI scaling factor
    pub fn resize(&mut self, window: &WindowWrapper) {
        self.resize_raw(window.fb_size_u());
    }

    pub fn resize_raw(&mut self, new_size: Extent2<u32>) {
        assert!(
            new_size.w != 0 && new_size.h != 0,
            "resizing to zero size can panic the app"
        );

        if self.size == new_size {
            return;
        }

        self.size = new_size;
        self.config.width = new_size.w;
        self.config.height = new_size.h;
        self.surface.configure(&self.device, &self.config);
    }
}

/// Accessors
impl Gpu {
    /// Current frame buffer's size in picels
    pub fn fb_size(&self) -> Extent2<u32> {
        self.size
    }
}

#[derive(Debug)]
pub struct App {
    pub gpu: Gpu,
}

impl App {
    pub fn new(window: &WindowWrapper) -> Self {
        let state = pollster::block_on(Gpu::new(window));
        Self { gpu: state }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.gpu.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render-pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        // submit will accept anything that implements IntoIter
        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
