use crate::Context;
use std::ops::Range;

pub struct Texture {
    pub(crate) ctx: Context,
    pub(crate) id: u32
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.ctx.delete_texture(self.id);
    }
}

pub struct Surface {
    pub(crate) ctx: Context,
    pub(crate) id: u32
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.ctx.delete_surface(self.id);
    }
}


// TODO: matrix uniforms

#[derive(Clone)]
pub enum UniformValue {
    Int(i32),
    Float(f32),
    Vector2([f32; 2]),
    Vector3([f32; 3]),
    Vector4([f32; 4]),
    IVector2([i32; 2]),
    IVector3([i32; 3]),
    IVector4([i32; 4]),
}

pub struct DrawList {
    pub(crate) range: Range<usize>,
    pub(crate) uniforms: Vec<(&'static str, UniformValue)>
}

impl DrawList {
    pub fn new(range: Range<usize>) -> DrawList {
        DrawList {
            range,
            uniforms: Vec::new()
        }
    }

    pub fn add_uniform_binding(&mut self, name: &'static str, uni: UniformValue) {
        self.uniforms.push((name, uni));
    }
}

pub enum ColorFormat {
    RGB, RGBA
}
