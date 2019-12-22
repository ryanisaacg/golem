use crate::{Dimension, NumberType};

pub struct Uniform {
    pub name: &'static str,
    pub u_type: UniformType,
}

impl Uniform {
    pub fn new(name: &'static str, u_type: UniformType) -> Uniform {
        Uniform { name, u_type }
    }

    pub(crate) fn as_glsl(&self, shader: &mut String) {
        shader.push_str("uniform ");
        self.u_type.write_type(shader);
        shader.push_str(self.name);
        shader.push_str(";");
    }
}

pub enum UniformType {
    Scalar(NumberType),
    Vector(NumberType, Dimension),
    Matrix(Dimension),
    Sampler2D,
    Array(Box<UniformType>, usize),
    UserType(String),
}

impl UniformType {
    fn write_type(&self, shader: &mut String) {
        use NumberType::*;
        use UniformType::*;

        match self {
            Scalar(Int) => shader.push_str("int "),
            Scalar(Float) => shader.push_str("float "),
            Vector(Int, x) => shader.push_str(&format!("ivec{} ", *x as i32)),
            Vector(Float, x) => shader.push_str(&format!("vec{} ", *x as i32)),
            Matrix(x) => shader.push_str(&format!("mat{} ", *x as i32)),
            Sampler2D => shader.push_str("sampler2D "),
            Array(u_type, dim) => {
                u_type.write_type(shader);
                shader.push_str(&format!("[{}]", dim));
            }
            UserType(string) => shader.push_str(&string),
        }
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
