// TODO: same array for uniforms and draw lists
// TODO: usize draw lists
// TODO: should there be a vertex-safe way of inserting stuff into the VBO?
// TODO: validate vec and matrix dimensions
// TODO: support non-float?
#[derive(Debug)]
pub struct GolemError();
mod context;
pub use self::context::Context;
pub mod input;
pub mod program;
/*
mod attribute;
mod buffer;
mod color;
mod shader;
pub use self::attribute::Attribute;
pub use self::buffer::{VertexBuffer, ElementBuffer};
pub use self::color::Color;
pub use self::shader::{FragmentShader, Shader, VertexShader};


mod context;
mod shaders;

pub struct Context {
    gl: RawContext,
    // TODO: cache shader, maybe vb and eb?
};

impl Context {
    pub fn from_raw(gl: RawContext) -> Context {
        Context {
            gl,
        }
    }

    pub fn clear(&mut self, col: Color) {
        self.gl.clear(col.r, col.g, col.b, col.a);
    }

    pub fn render(&mut self, surface: &Surface, vb: &VertexBuffer, eb: &ElementBuffer) {
        // TODO: Set up vertex specification from shader

    }
}

pub struct Buffer<T: GpuSerialize>;
impl<T> Buffer<T> {
    pub fn new() -> Buffer<T> {

    }
}
pub trait BufferLike;

pub trait IsUniform;
pub trait GpuSerialize;
pub struct Surface;

pub struct ShaderProgram<T>;
pub struct VertexShader<T>;
pub struct FragmentShader;



impl Surface {
    pub fn submit(shader: ShaderProgram<T>, vertices: &dyn BufferLike, indices: &dyn BufferLike, uniforms: Uniforms);
}

pub struct Context;*/
