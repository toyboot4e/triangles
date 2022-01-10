//! Immediate-mode 2D rendering

mod gpu;
mod mesh;
mod window;

pub use gpu::Gpu;
pub use mesh::StaticMesh;
pub use window::WindowWrapper;

use std::mem;

use vek::{Vec2, Vec4};

pub trait Vertex {
    /// Declares memory layout of vertex buffer
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

/// Triangle vertex
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct TriVertex {
    /// [x, y]
    pub pos: Vec2<f32>,
    /// RGBA
    pub color: Vec4<f32>,
}

impl Vertex for TriVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRS: &'static [wgpu::VertexAttribute] =
            &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: ATTRS,
        }
    }
}
