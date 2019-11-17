use glow::HasContext;
use crate::GolemError;
use crate::input::{Buffer, Color, ElementBuffer, Surface, VertexBuffer, Uniforms};
use crate::program::{ShaderDescription, ShaderProgram};

pub struct Context {
    gl: glow::Context,
}

impl Context {
    pub fn from_glow(gl: glow::Context) -> Context {
        Context {
            gl
        }
    }

    pub fn new_shader(&self, desc: ShaderDescription) -> Result<ShaderProgram, GolemError> {
        unimplemented!();
    }

    fn new_buffer(&self) -> Buffer {
        unimplemented!();
    }

    pub fn new_vertex_buffer(&self) -> VertexBuffer {
        VertexBuffer(self.new_buffer())
    }

    pub fn new_element_buffer(&self) -> ElementBuffer {
        ElementBuffer(self.new_buffer())
    }

    pub fn set_target(&mut self, surface: &Surface) {
        unimplemented!();

    }

    pub fn reset_target(&mut self) {

    }

    pub fn set_program(&mut self, program: &ShaderProgram) {
        unimplemented!();
    }

    pub fn clear(&mut self, col: Color) {
        unimplemented!();
    }

    pub fn draw(&mut self, vb: &VertexBuffer, eb: &ElementBuffer, u: &Uniforms) {
        unimplemented!();
    }
}
