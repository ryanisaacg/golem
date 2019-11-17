use crate::input::{Vec2, Vec4};

pub struct Attribute {
    name: String,
    gl_type: &'static str
}

impl Attribute {
    pub fn new<T: GlType>(name: &str) -> Attribute {
        Attribute {
            name: String::from(name),
            gl_type: T::type_value(),
        }
    }
}

pub trait GlType {
    fn type_value() -> &'static str;
}

impl GlType for Vec2 {
    fn type_value() -> &'static str {
        "vec2"
    }
}

impl GlType for Vec4 {
    fn type_value() -> &'static str {
        "vec4"
    }
}

pub struct ShaderDescription<'a> {
    pub vertex_input: &'a [Attribute],
    pub fragment_input: &'a [Attribute],
    pub uniforms: &'a [Attribute],
    pub vertex_shader: &'a str,
    pub fragment_shader: &'a str,
}

pub struct ShaderProgram {
    pub(crate) id: u32,
    pub(crate) vert: u32,
    pub(crate) frag: u32,
    pub(crate) input: Vec<Attribute>,
}
