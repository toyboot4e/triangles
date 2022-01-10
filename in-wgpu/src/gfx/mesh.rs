//! Vertices

use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use crate::gfx::Vertex;

/// Static (CPU)/GPU buffers
#[derive(Debug)]
pub struct StaticMesh<V> {
    /// CPU vertices (no need to keep the memory)
    _verts: PhantomData<V>,
    n_verts: u32,
    /// GPU vertices
    vbuf: wgpu::Buffer,
    // /// GPU indices
    // ibuf: wgpu::Buffer,
}

impl<V: bytemuck::Pod + Vertex> StaticMesh<V> {
    pub fn new(device: &wgpu::Device, verts: &[V]) -> Self {
        assert!(verts.len() <= std::u32::MAX as usize);

        let vbuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("static-mesh-vbuf"),
            contents: bytemuck::cast_slice(verts),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            _verts: PhantomData,
            n_verts: verts.len() as u32,
            vbuf,
        }
    }

    pub fn draw_all<'v, 'p>(&'v mut self, rpass: &'p mut wgpu::RenderPass<'v>) {
        rpass.set_vertex_buffer(0, self.vbuf.slice(..));
        // rpass.set_index_buffer(self.ibuf.slice(..), wgpu::IndexFormat::Uint32);
        rpass.draw(0..self.n_verts, 0..1);
        // rpass.draw_indexed(0..self.num_elements, 0, instances);
    }
}
