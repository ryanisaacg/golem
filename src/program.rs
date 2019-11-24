use crate::{GolemError, GlProgram, GlShader};
use crate::buffer::VertexBuffer;
use crate::objects::UniformValue;

pub struct ShaderDescription<'a> {
    pub vertex_input: &'a [Attribute],
    pub fragment_input: &'a [Attribute],
    pub uniforms: &'a [Uniform],
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
}

pub struct ShaderProgram {
    pub(crate) ctx: crate::Context,
    pub(crate) id: GlProgram,
    pub(crate) vertex: GlShader,
    pub(crate) fragment: GlShader,
    pub(crate) input: Vec<Attribute>,
}

impl ShaderProgram {
    pub fn is_bound(&self) -> bool {
        self.ctx.is_program_bound(self.id)
    }

    pub fn set_uniform(&self, name: &str, uniform: UniformValue) -> Result<(), GolemError> {
        if self.is_bound() {
            self.ctx.bind_uniform(self.id, name, uniform)
        } else {
            Err(GolemError::NotCurrentProgram)
        }
    }

    pub fn bind(&mut self, vb: &VertexBuffer) {
        self.ctx.bind_program(self.id, &self.input, vb);
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.ctx.delete_shader(self.id, self.vertex, self.fragment);
    }
}

#[derive(Clone)]
pub enum Attribute {
    Scalar(&'static str),
    Vector(u8, &'static str),
    Matrix(u8, u8, &'static str),
}

pub(crate) enum Position { Input, Output }

impl Position {
    #[cfg(target_arch = "wasm32")]
    fn glsl_string(self) -> &'static str {
        use Position::*;

        match self {
            Input => "attribute ",
            Output => "varying ",
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn glsl_string(self) -> &'static str {
        use Position::*;

        match self {
            Input => "in ",
            Output => "out ",
        }
    }
}

impl Attribute {
    pub fn name(&self) -> &str {
        use Attribute::*;

        match self {
            Scalar(name) => name,
            Vector(_, name) => name,
            Matrix(_, _, name) => name,
        }
    }

    pub fn size(&self) -> i32 {
        use Attribute::*;

        match self {
            Scalar(_) => 1,
            Vector(n, _) => *n as i32,
            Matrix(m, n, _) => (m * n) as i32,
        }
    }


    pub(crate) fn as_glsl(&self, _is_vertex: bool, pos: Position, shader: &mut String) {
        use Attribute::*;

        #[cfg(target_arch = "wasm32")]
        let pos = if _is_vertex { pos } else { Position::Output };

        shader.push_str(pos.glsl_string());
        let (gl_type, name) = match self {
            Scalar(name) => ("float ".to_owned(), name),
            Vector(n, name) => (format!("vec{} ", n), name),
            Matrix(m, n, name) => (format!("mat{}x{} ", m, n), name),
        };
        shader.push_str(&gl_type);
        shader.push_str(name);
        shader.push_str(";");
    }
}

pub enum NumberType { Int, Float }

pub enum UniformType {
    Scalar(NumberType),
    Vector(NumberType, u8),
    Matrix(u8),
    Sampler(u8),
    Array(Box<UniformType>, usize),
    UserType(String),
}

pub struct Uniform {
    pub name: &'static str,
    pub u_type: UniformType,
}

impl Uniform {
    pub fn new(name: &'static str, u_type: UniformType) -> Uniform {
        Uniform {
            name,
            u_type
        }
    }

    pub(crate) fn as_glsl(&self, shader: &mut String) {
        shader.push_str("uniform ");
        self.u_type.write_type(shader);
        shader.push_str(self.name);
        shader.push_str(";");
    }
}



impl UniformType {
    fn write_type(&self, shader: &mut String) {
        use NumberType::*;
        use UniformType::*;

        match self {
            Scalar(Int) => shader.push_str("int "),
            Scalar(Float) => shader.push_str("float "),
            Vector(Int, x) => shader.push_str(&format!("ivec{} ", x)),
            Vector(Float, x) => shader.push_str(&format!("fvec{} ", x)),
            Matrix(x) => shader.push_str(&format!("mat{} ", x)),
            Sampler(x) => shader.push_str(&format!("sampler{}D ", x)),
            Array(u_type, dim) => {
                u_type.write_type(shader);
                shader.push_str(&format!("[{}]", dim));
            },
            UserType(string) => shader.push_str(&string),
        }

    }
}
