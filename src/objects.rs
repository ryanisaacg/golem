use std::ops::Range;

pub struct Texture {
    pub(crate) id: u32
}

pub struct Surface {
    pub(crate) id: u32
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

// TODO: BGR, BGRA?
pub enum ColorFormat {
    RGB, RGBA
}
