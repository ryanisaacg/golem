use crate::Context;
use std::collections::HashMap;

pub struct VertexBuffer(pub(crate) Buffer);

impl VertexBuffer {
    pub fn send_data(&mut self, start: usize, data: &[f32]) {
        self.0.send_data(glow::ARRAY_BUFFER, start, data);
    }
}

pub struct ElementBuffer(pub(crate) Buffer);

impl ElementBuffer {
    pub fn send_data(&mut self, start: usize, data: &[u32]) {
        self.0.send_data(glow::ELEMENT_ARRAY_BUFFER, start, data);
    }
}

pub(crate) struct Buffer {
    pub(crate) ctx: Context,
    pub(crate) id: u32,
    pub(crate) length: usize
}

impl Buffer {
    fn send_data<T: bytemuck::Pod>(&mut self, bind: u32, start: usize, data: &[T]) {
        self.ctx.bind(self, bind);
        self.ctx.send_data(bind, self.length, start, data);
    }
}

pub struct Uniforms(HashMap<&'static str, Box<dyn Uniform>>);

impl Uniforms {
    pub fn new() -> Uniforms {
        Uniforms(HashMap::new())
    }
}

pub trait Uniform {

}

use mint::*;

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;
pub type Color = Vec4;

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 { x, y }
}

// TODO: rest of the vecn types

pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Vec4 {
        x: r,
        y: g,
        z: b,
        w: a
    }
}

pub struct Texture {
    pub(crate) id: u32
}

pub struct Surface {
    pub(crate) id: u32
}
