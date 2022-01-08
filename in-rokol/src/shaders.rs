//! GLSL shaders

// TODO: consider supporting `sokol_shdc.h`

#![allow(unused)]

use rokol::gfx::{self as rg, BakedResource, LayoutDesc};

use crate::gfx::Shader;

/// Shorthand for specifying shader files
macro_rules! def_shd {
    ($file:expr) => {
        embed_shd!(
            concat!("glsl/", $file, ".vs"),
            concat!("glsl/", $file, ".fs"),
        )
    };
}

/// Loads shader files, dynamically on debug build, staticlly on release build
macro_rules! embed_shd {
    ($vs:expr, $fs:expr,) => {
        if cfg!(debug_assertions) {
            // debug: dynamically load the shader files
            let dir =
                std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/");
            let mut v = std::fs::read_to_string(dir.join($vs)).unwrap();
            v.push('\0');
            let mut f = std::fs::read_to_string(dir.join($fs)).unwrap();
            f.push('\0');
            [v, f]
        } else {
            // release: statically load the shader files
            [
                concat!(include_str!($vs), "\0").to_string(),
                concat!(include_str!($fs), "\0").to_string(),
            ]
        }
    };
}

/// Generates [`Shader`]
fn gen(
    vs_fs: &[impl AsRef<str>; 2],
    mut_shd_desc: impl Fn(&mut rg::ShaderDesc),
    pip_desc: &mut rg::PipelineDesc,
) -> Shader {
    let mut shd_desc = unsafe { rokol::gfx::shader_desc(vs_fs[0].as_ref(), vs_fs[1].as_ref()) };
    mut_shd_desc(&mut shd_desc);

    let shd = rg::Shader::create(&shd_desc);

    pip_desc.shader = shd;
    let pip = rg::Pipeline::create(&pip_desc);

    Shader::new(shd, pip)
}

/// Sets image type
macro_rules! img_type {
    ($name:expr,$ty:expr) => {
        rg::ShaderImageDesc {
            name: concat!($name, "\0").as_ptr() as *const _,
            image_type: $ty.to_ffi(),
            ..Default::default()
        }
    };
}

/// Single-value uniform block
macro_rules! ub {
    ($name:expr, $uniform_ty:expr, $size_ty:ty) => {{
        let mut block = rg::ShaderUniformBlockDesc::default();

        block.uniforms[0] = rg::ShaderUniformDesc {
            name: concat!($name, "\0").as_ptr() as *const _,
            type_: $uniform_ty.to_ffi(),
            ..Default::default()
        };
        block.size += std::mem::size_of::<$size_ty>().to_ffi();

        block
    }};
}

/// (position, color) vertex
#[derive(Debug, Clone)]
#[repr(C)]
pub struct TriangleVertex {
    /// X, Y, Z
    pos: [f32; 3],
    /// R, G, B, A
    color: [f32; 4],
}

impl TriangleVertex {
    pub fn layout_desc() -> rg::LayoutDesc {
        let mut desc = rg::LayoutDesc::default();
        desc.attrs[0].format = rg::VertexFormat::Float3.to_ffi();
        desc.attrs[1].format = rg::VertexFormat::Float4.to_ffi();
        desc
    }
}

impl<T, U> From<(T, U)> for TriangleVertex
where
    T: Into<[f32; 3]>,
    U: Into<[f32; 4]>,
{
    fn from(data: (T, U)) -> Self {
        Self {
            pos: data.0.into(),
            color: data.1.into(),
        }
    }
}

pub fn triangle() -> Shader {
    gen(
        &def_shd!("triangle"),
        |_shd| {},
        &mut rg::PipelineDesc {
            index_type: rg::IndexType::UInt16.to_ffi(),
            layout: TriangleVertex::layout_desc(),
            // cull_mode: rg::CullMode::None .to_ffi(),
            ..Default::default()
        },
    )
}

/// (position, color) vertex
#[derive(Debug, Clone, LayoutDesc)]
#[repr(C)]
pub struct TexturedVertex {
    // /// X, Y, Z
    pub pos: [f32; 3],
    // /// R, G, B, A
    pub color: [u8; 4],
    // /// u, v
    pub uv: [f32; 2],
}

// impl TexturedVertex {
//     pub fn layout_desc() -> rg::LayoutDesc {
//         let mut desc = rg::LayoutDesc::default();
//         desc.attrs[0].format = rg::VertexFormat::Float3.to_ffi();
//         desc.attrs[1].format = rg::VertexFormat::UByte4N.to_ffi();
//         desc.attrs[2].format = rg::VertexFormat::Float2.to_ffi();
//         desc
//     }
// }

impl<T, U, V> From<(T, U, V)> for TexturedVertex
where
    T: Into<[f32; 3]>,
    U: Into<[u8; 4]>,
    V: Into<[f32; 2]>,
{
    fn from(data: (T, U, V)) -> Self {
        Self {
            pos: data.0.into(),
            color: data.1.into(),
            uv: data.2.into(),
        }
    }
}

fn alpha_blend() -> rg::BlendState {
    rg::BlendState {
        enabled: true,
        src_factor_rgb: rg::BlendFactor::SrcAlpha.to_ffi(),
        dst_factor_rgb: rg::BlendFactor::OneMinusSrcAlpha.to_ffi(),
        op_rgb: rg::BlendOp::_Default.to_ffi(),
        src_factor_alpha: rg::BlendFactor::One.to_ffi(),
        dst_factor_alpha: rg::BlendFactor::Zero.to_ffi(),
        op_alpha: rg::BlendOp::_Default.to_ffi(),
    }
}

pub fn texture() -> Shader {
    gen(
        &def_shd!("texture"),
        |shd| {
            shd.fs.images[0] = img_type!("tex", rg::ImageType::Dim2);
        },
        &mut {
            let mut pip = rg::PipelineDesc {
                index_type: rg::IndexType::UInt16.to_ffi(),
                layout: TexturedVertex::layout_desc(),
                cull_mode: rg::CullMode::None.to_ffi(),
                ..Default::default()
            };
            // pip.colors[0].blend = alpha_blend();
            pip
        },
    )
}
