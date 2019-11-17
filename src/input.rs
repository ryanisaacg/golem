use crate::program::{Attribute, GlType};
use std::collections::HashMap;

pub struct VertexBuffer(pub(crate) Buffer);

impl VertexBuffer {
    pub fn send_data(&mut self, start: usize, data: &[u8]) {
        unimplemented!();
    }
}

pub struct VertexBuilder();

impl VertexBuilder {
    pub fn new<'a>(attr: &'a [Attribute]) -> VertexBuilder {
        unimplemented!();
    }

    pub fn start<'a>(&'a mut self) -> Vertex<'a> {
        unimplemented!();
    }

    pub fn data<'a>(&'a self) -> &'a Vec<u8> {
        unimplemented!();
    }
}

pub struct Vertex<'a> {
    buffer: &'a mut Vec<u8>
}

impl Vertex<'_> {
    pub fn add<'a>(self, val: &impl GlType) -> Vertex<'a> {
        unimplemented!();
    }

    pub fn build(self) {
        unimplemented!();
    }
}

pub struct ElementBuffer(pub(crate) Buffer);

impl ElementBuffer {
    pub fn send_data(&mut self, start: usize, data: &[u32]) {
        unimplemented!();
    }
}

pub(crate) struct Buffer {
    id: u32,
    length: usize
}

pub struct Uniforms(HashMap<&'static str, Box<dyn Uniform>>);

impl Uniforms {
    pub fn new() -> Uniforms {
        unimplemented!();
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
