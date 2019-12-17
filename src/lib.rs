// TODO: add out-of-memory to GolemError?
// TODO: unsafe audit: check for possible GL error conditions, and track them

use glow::HasContext;

type GlTexture = <glow::Context as HasContext>::Texture;
type GlProgram = <glow::Context as HasContext>::Program;
type GlShader = <glow::Context as HasContext>::Shader;
type GlFramebuffer = <glow::Context as HasContext>::Framebuffer;
type GlBuffer = <glow::Context as HasContext>::Buffer;


mod attribute;
mod buffer;
mod context;
mod shader;
mod surface;
mod texture;
mod uniform;

pub use self::attribute::{Attribute, AttributeType};
pub use self::buffer::{VertexBuffer, ElementBuffer};
pub use self::context::Context;
pub use self::shader::{ShaderDescription, ShaderProgram};
pub use self::surface::Surface;
pub use self::texture::{Texture, TextureFilter, TextureWrap};
pub use self::uniform::{Uniform, UniformType, UniformValue};

pub(crate) enum Position { Input, Output }

pub enum NumberType { Int, Float }

pub enum ColorFormat {
    RGB, RGBA
}

#[derive(Copy, Clone)]
pub enum Dimension {
    D2 = 2,
    D3 = 3,
    D4 = 4,
}

pub enum GeometryMode {
    Points, Lines, LineStrip, LineLoop, TriangleStrip, TriangleFan, Triangles
}

#[derive(Debug)]
pub enum GolemError {
    /// The OpenGL Shader compilation failed, with the given error message
    ///
    /// This may be during vertex-time, fragment-time, or link-time
    ShaderCompilationError(String),
    /// Some general error bubbling up from the GL context
    ContextError(String),
    /// An attempt was made to bind to an illegal uniform TODO
    NoSuchUniform(&'static str),
    /// An operation was performed on a shader that wasn't bound
    ///
    /// Shader operations include setting uniforms and drawing
    NotCurrentProgram,
}

impl From<String> for GolemError {
    fn from(other: String) -> Self {
        GolemError::ContextError(other)
    }
}
