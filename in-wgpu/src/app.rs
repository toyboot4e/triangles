use anyhow::Result;

use crate::gfx::{Gpu, StaticMesh, Texture, TriVertex, Vertex, WindowWrapper};

#[derive(Debug)]
pub struct App {
    pub gpu: Gpu,
    rpip: wgpu::RenderPipeline,
    mesh: StaticMesh<TriVertex, u16>,
    texture: Texture,
    bind_group: wgpu::BindGroup,
}

fn verts() -> [TriVertex; 5] {
    [
        // (pos, color, uv)
        (
            [-0.0868241, 0.49240386],
            [1.0, 1.0, 1.0, 1.0],
            [0.4131759, 0.00759614],
        ),
        (
            [-0.49513406, 0.06958647],
            [1.0, 1.0, 1.0, 1.0],
            [0.0048659444, 0.43041354],
        ),
        (
            [-0.21918549, -0.44939706],
            [1.0, 1.0, 1.0, 1.0],
            [0.28081453, 0.949397],
        ),
        (
            [0.35966998, -0.3473291],
            [1.0, 1.0, 1.0, 1.0],
            [0.85967, 0.84732914],
        ),
        (
            [0.44147372, 0.2347359],
            [1.0, 1.0, 1.0, 1.0],
            [0.9414737, 0.2652641],
        ),
    ]
    .map(TriVertex::from)
}

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];

impl App {
    pub async fn new(window: &WindowWrapper) -> Result<Self> {
        let gpu = Gpu::new(window).await;

        let mesh = StaticMesh::new(&gpu.device, &verts(), INDICES);

        let bytes = include_bytes!("../assets/happy-tree.png");
        let texture = Texture::from_bytes(&gpu, bytes, "happy-tree")?;
        let (group_layout, bind_group) = self::simple_bind_group(&gpu, &texture);

        let rpip = self::simple_rpip::<TriVertex>(
            &gpu.device,
            include_str!("shader.wgsl"),
            gpu.config.format,
            &group_layout,
        );

        Ok(Self {
            gpu,
            rpip,
            mesh,
            texture,
            bind_group,
        })
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
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

            rpass.set_pipeline(&self.rpip);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            self.mesh.draw_all(&mut rpass);
        }

        // submit will accept anything that implements IntoIter
        self.gpu.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn simple_rpip<V: Vertex>(
    device: &wgpu::Device,
    src: &str,
    tex_fmt: wgpu::TextureFormat,
    layout: &wgpu::BindGroupLayout,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("shader"),
        source: wgpu::ShaderSource::Wgsl(src.into()),
    });

    let rpip_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("render-pipeline-layout"),
        bind_group_layouts: &[layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("render-pipeline"),
        layout: Some(&rpip_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[V::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: tex_fmt,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            // cull the back face
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // other than this requires`Features::NON_FILL_POLYGON_MODE`
            polygon_mode: wgpu::PolygonMode::Fill,
            // requires `Features::DEPTH_CLIP_CONTROL`
            unclipped_depth: false,
            // requires `Features::CONSERVATIVE_RASTERIZATION`
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1, // single sampling
            mask: !0, // all samples are active
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    })
}

fn simple_bind_group(gpu: &Gpu, texture: &Texture) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let layout = gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture-bind-group-layout"),
        });

    let group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            },
        ],
        label: Some("diffuse-bind-group"),
    });

    (layout, group)
}
