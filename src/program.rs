pub struct ShaderDescription<'a> {
    pub vertex_input: &'a [Attribute],
    pub fragment_input: &'a [Attribute],
    pub uniforms: &'a [Uniform],
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
}

pub struct ShaderProgram {
    pub(crate) id: u32,
    pub(crate) vertex: u32,
    pub(crate) fragment: u32,
    pub(crate) input: Vec<Attribute>,
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


    pub(crate) fn as_glsl(&self, pos: Position, shader: &mut String) {
        use Attribute::*;

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

// TODO: handle floats v ints?
pub enum UniformType {
    Scalar(NumberType),
    Vector(NumberType, u8),
    Matrix(NumberType, u8, u8),
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
}
