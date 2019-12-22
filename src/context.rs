use crate::{GlProgram, GolemError};
use glow::HasContext;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Context(pub(crate) Rc<ContextContents>);

pub(crate) struct ContextContents {
    pub(crate) gl: glow::Context,
    pub(crate) current_program: RefCell<Option<GlProgram>>,
    #[cfg(not(target_arch = "wasm32"))]
    vao: u32,
}

impl Drop for ContextContents {
    fn drop(&mut self) {
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml
        // glow handles passing in the pointer to our value, and GL will silently ignore invalid
        // values
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

impl Context {
    pub fn from_glow(gl: glow::Context) -> Result<Context, GolemError> {
        #[cfg(not(target_arch = "wasm32"))]
        let vao = unsafe {
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml
            // glow handles passing in '1' and returning the value to us
            let vao = gl.create_vertex_array()?;
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml
            // In this case, we know 'vao' must be a valid vao because we just constructed it
            gl.bind_vertex_array(Some(vao));

            vao
        };

        let contents = Context(Rc::new(ContextContents {
            gl,
            current_program: RefCell::new(None),
            #[cfg(not(target_arch = "wasm32"))]
            vao,
        }));
        contents.set_clear_color(0.0, 0.0, 0.0, 1.0);

        Ok(contents)
    }

    pub fn set_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.0
                .gl
                .viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml
        // Set the clear color to (r, g, b, a)
        unsafe {
            self.0.gl.clear_color(r, g, b, a);
        }
    }

    pub fn clear(&self) {
        let gl = &self.0.gl;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }
}
