use crate::input::{Vec2, Vec4};

pub(crate) enum Position {
    Input, Output, Uniform
}

#[derive(Clone)]
pub struct Attribute {
    pub(crate) name: String,
    pub(crate) size: i32,
    pub(crate) type_index: u32,
    gl_type: &'static str,
}

impl Attribute {
    pub fn new<T: GlType>(name: &str) -> Attribute {
        Attribute {
            name: String::from(name),
            size: T::size(),
            gl_type: T::type_value(),
            type_index: T::type_index(),
        }
    }

    pub(crate) fn as_glsl(&self, position: Position, string: &mut String) {
        #[cfg(target_arch = "wasm32")]
        let quantifier = match position {
            Position::Input => "attribute",
            Position::Output => "varying",
            Position::Uniform => "uniform",
        };
        #[cfg(not(target_arch = "wasm32"))]
        let quantifier = match position {
            Position::Input => "in",
            Position::Output => "out",
            Position::Uniform => "uniform",
        };
        string.push_str(quantifier);
        string.push_str(" ");
        string.push_str(self.gl_type);
        string.push_str(" ");
        string.push_str(&self.name);
        string.push_str(";");
    }

    pub fn matches<T: GlType>(&self) -> bool {
        self.type_index == T::type_index()
    }
}

// TODO: conflating GlType and VertexType, and Attribute should know somehow

pub trait GlType {
    fn type_value() -> &'static str;
    fn size() -> i32;
    fn type_index() -> u32;
}

pub trait VertexType: GlType {
    fn to_buffer(&self, buffer: &mut Vec<f32>);
}

impl GlType for Vec2 {
    fn type_value() -> &'static str { "vec2" }
    fn size() -> i32 { 2 }
    fn type_index() -> u32 { glow::FLOAT }
}

impl VertexType for Vec2 {
    fn to_buffer(&self, buffer: &mut Vec<f32>) {
        let floats: [f32; 2] = (*self).into();
        
        buffer.extend(floats.iter())
    }
}

impl GlType for Vec4 {
    fn type_value() -> &'static str { "vec4" }
    fn size() -> i32 { 4 }
    fn type_index() -> u32 { glow::FLOAT }
}

impl VertexType for Vec4 {
    fn to_buffer(&self, buffer: &mut Vec<f32>) {
        let floats: [f32; 4] = (*self).into();
        
        buffer.extend(floats.iter())
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
    pub(crate) vertex: u32,
    pub(crate) fragment: u32,
    pub(crate) input: Vec<Attribute>,
}
