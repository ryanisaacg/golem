use crate::Context;

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

pub enum ColorFormat {
    RGB, RGBA
}

pub enum GeometryType {
    Points, Lines, LineStrip, LineLoop, TriangleStrip, TriangleFan, Triangles
}
