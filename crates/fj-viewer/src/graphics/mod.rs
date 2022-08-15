//! Rendering primitives, routines, and structures.

mod config_ui;
mod draw_config;
mod drawables;
mod geometries;
mod pipelines;
mod reference_axes;
mod renderer;
mod shaders;
mod transform;
mod uniforms;
mod vertices;

pub use self::{
    draw_config::DrawConfig,
    renderer::{DrawError, InitError, Renderer},
};

const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
