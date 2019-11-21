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
    Matrix2([f32; 4]),
    Matrix3([f32; 9]),
    Matrix4([f32; 16]),
}

pub struct DrawList {
    pub(crate) range: Range<usize>,
    pub(crate) uniforms: Vec<(&'static str, UniformValue)>,
    pub(crate) geometry: GeometryType,
}

impl DrawList {
    pub fn new(range: Range<usize>) -> DrawList {
        DrawList {
            range,
            uniforms: Vec::new(),
            geometry: GeometryType::Triangles,
        }
    }

    pub fn add_uniform_binding(&mut self, name: &'static str, uni: UniformValue) {
        self.uniforms.push((name, uni));
    }

    pub fn set_geometry_type(&mut self, geom_type: GeometryType) {
        self.geometry = geom_type;
    }
}

pub enum ColorFormat {
    RGB, RGBA
}

pub enum GeometryType {
    Points, Lines, LineStrip, LineLoop, TriangleStrip, TriangleFan, Triangles
}
