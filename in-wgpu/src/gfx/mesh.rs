//! Vertex/index buffer in handy API

use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use crate::gfx::Vertex;

/// `u16` | `u32`
pub trait Index: bytemuck::Pod {
    fn format() -> wgpu::IndexFormat;
}

impl Index for u16 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
}

impl Index for u32 {
    fn format() -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint32
    }
}

/// Static (CPU)/GPU buffers
#[derive(Debug)]
pub struct StaticMesh<V, I> {
    /// CPU vertices (no need to keep the memory)
    _verts: PhantomData<V>,
    _n_verts: u32,
    /// GPU vertices
    vbuf: wgpu::Buffer,
    /// GPU indices
    _indices: PhantomData<I>,
    n_indices: u32,
    /// GPU indices
    ibuf: wgpu::Buffer,
}

impl<V: bytemuck::Pod + Vertex, I: Index> StaticMesh<V, I> {
    pub fn new(device: &wgpu::Device, verts: &[V], indices: &[I]) -> Self {
        assert!(verts.len() <= std::u32::MAX as usize);

        let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("static-mesh-vbuf"),
            contents: bytemuck::cast_slice(verts),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let ibuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        let n_indices = indices.len() as u32;

        Self {
            _verts: PhantomData,
            _n_verts: verts.len() as u32,
            vbuf,
            _indices: PhantomData,
            ibuf,
            n_indices,
        }
    }

    pub fn draw_all<'v, 'p>(&'v mut self, rpass: &'p mut wgpu::RenderPass<'v>) {
        rpass.set_vertex_buffer(0, self.vbuf.slice(..));
        rpass.set_index_buffer(self.ibuf.slice(..), I::format());
        rpass.draw_indexed(0..self.n_indices, 0, 0..1);
    }
}
