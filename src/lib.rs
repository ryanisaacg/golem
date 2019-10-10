use glow::*;

pub struct Context;

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

pub struct Context;
