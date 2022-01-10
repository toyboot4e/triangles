//! Immediate-mode 2D rendering

mod gpu;
mod mesh;
mod window;

pub use gpu::Gpu;
pub use mesh::StaticMesh;
pub use window::WindowWrapper;

use std::mem;

use anyhow::*;
use image::GenericImageView;
use vek::{Vec2, Vec4};

// TODO: add color struct

pub trait Vertex {
    /// Declares memory layout of vertex buffer
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

/// Triangle vertex
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct TriVertex {
    /// XY
    pub pos: Vec2<f32>,
    /// RGBA
    pub color: Vec4<f32>,
    /// UV
    pub uv: Vec2<f32>,
}

impl From<([f32; 2], [f32; 4], [f32; 2])> for TriVertex {
    fn from(x: ([f32; 2], [f32; 4], [f32; 2])) -> Self {
        Self {
            pos: x.0.into(),
            color: x.1.into(),
            uv: x.2.into(),
        }
    }
}

impl Vertex for TriVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRS: &'static [wgpu::VertexAttribute] =
            &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4, 2 => Float32x2];
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: ATTRS,
        }
    }
}

/// `wgpu` texture
#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub fn from_bytes(gpu: &Gpu, bytes: &[u8], label: &str) -> Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(&gpu.device, &gpu.queue, &img, Some(label))
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.as_rgba8().unwrap();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self {
            texture,
            view,
            sampler,
        })
    }
}
