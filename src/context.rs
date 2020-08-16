use crate::blend::{BlendEquation, BlendFunction, BlendMode};
use crate::depth::DepthTestMode;
use crate::{GlFramebuffer, GlProgram, GlVertexArray, GolemError};
use alloc::rc::Rc;
use core::cell::RefCell;
use core::ffi::c_void;
use glow::HasContext;

/// The context required to interact with the GPU
pub struct Context(pub(crate) Rc<ContextContents>);

pub(crate) struct ContextContents {
    pub(crate) gl: glow::Context,
    pub(crate) current_program: RefCell<Option<GlProgram>>,
    pub(crate) current_surface: RefCell<Option<GlFramebuffer>>,
    vao: GlVertexArray,
}

impl Drop for ContextContents {
    fn drop(&mut self) {
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml
        // glow handles passing in the pointer to our value, and GL will silently ignore invalid
        // values
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

impl Context {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_loader_function(func: impl FnMut(&str) -> *const c_void) -> Result<Context, GolemError> {
        Self::from_glow(glow::Context::from_loader_function(func))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_handle(handle: raw_window_handle::web::Webhandle) -> Result<Context, GolemError> {
        let index = handle.id.to_string();
        #[cfg(feature = "stdweb")]
        return Self::from_handle_stdweb(index);
        #[cfg(feature = "web-sys")]
        return Self::from_handle_stdweb(index);
    }

    fn from_handle_stdweb(index: String) -> Result<Context, GolemError> {
        let document = stdweb::document();
        // TODO: stdweb doesn't have this
        let elements = document.get_elements_by_tag_name("canvas");
        let mut found = None;
        for i in 0..elements.length() {
            let potential = elements.get_with_index(i);
            if potential.get_attribute("data-raw-handle").map_or(false, |attr| attr == index) {
                found = Some(potential);
            }
        }
        match found {
            Some(element) => {
                let canvas: web_sys::HtmlCanvasElement = element.unchecked_into();
                use js_sys::{Map, Object};
                use wasm_bindgen::{JsCast, JsValue};
                use winit::platform::web::WindowExtWebSys;
                let map = Map::new();
                map.set(&JsValue::from_str("premultipliedAlpha"), &JsValue::FALSE);
                map.set(&JsValue::from_str("alpha"), &JsValue::FALSE);
                let props = Object::from_entries(&map).expect("TODO");

                let ctx = canvas.get_context_with_context_options("webgl", &props)
                    .expect("Failed to acquire a WebGL rendering context")
                    .expect("Failed to acquire a WebGL rendering context")
                    .dyn_into::<web_sys::WebGlRenderingContext>()
                    .expect("WebGL context of unexpected type");

                Self::from_glow(glow::Context::from_webgl1_context(ctx))
            }
            None => Err(GolemError::CanvasNotFound),
        }
    }

    fn from_handle_bindgen(index: String) -> Result<Context, GolemError> {
        let document = web_sys::document();
        let elements = document.get_elements_by_tag_name("canvas");
        let mut found = None;
        for i in 0..elements.length() {
            let potential = elements.get_with_index(i);
            if potential.get_attribute("data-raw-handle").map_or(false, |attr| attr == index) {
                found = Some(potential);
            }
        }
        match found {
            Some(element) => {
                let canvas: web_sys::HtmlCanvasElement = element.unchecked_into();
                use js_sys::{Map, Object};
                use wasm_bindgen::{JsCast, JsValue};
                use winit::platform::web::WindowExtWebSys;
                let map = Map::new();
                map.set(&JsValue::from_str("premultipliedAlpha"), &JsValue::FALSE);
                map.set(&JsValue::from_str("alpha"), &JsValue::FALSE);
                let props = Object::from_entries(&map).expect("TODO");

                let ctx = canvas.get_context_with_context_options("webgl", &props)
                    .expect("Failed to acquire a WebGL rendering context")
                    .expect("Failed to acquire a WebGL rendering context")
                    .dyn_into::<web_sys::WebGlRenderingContext>()
                    .expect("WebGL context of unexpected type");

                Self::from_glow(glow::Context::from_webgl1_context(ctx))
            }
            None => Err(GolemError::CanvasNotFound),
        }
    }

    /// Create an instance from an OpenGL context
    #[deprecated]
    pub fn from_glow(gl: glow::Context) -> Result<Context, GolemError> {
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
            current_surface: RefCell::new(None),
            vao,
        }));
        contents.set_clear_color(0.0, 0.0, 0.0, 1.0);

        Ok(contents)
    }

    /// Set the section of the framebuffer that will be rendered to
    ///
    /// By default, this is the entire internal area of the window. When switching to a
    /// [`Surface`], it's generally important to set the viewport to its area.
    ///
    /// [`Surface`]: crate::Surface
    pub fn set_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.0
                .gl
                .viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }

    /// Set the color the render target will be cleared to by [`clear`]
    ///
    /// [`clear`]: Context::clear
    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml
        // Set the clear color to (r, g, b, a)
        unsafe {
            self.0.gl.clear_color(r, g, b, a);
        }
    }

    /// Clear the current render target to the render color (see [`set_clear_color`])
    ///
    /// [`set_clear_color`]: Context::set_clear_color
    pub fn clear(&self) {
        let gl = &self.0.gl;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    /// Set the blend mode, with `None` disabling blending
    ///
    /// By default, this is `None`
    ///
    /// See the documentation for [`BlendMode`] for the various blending options
    pub fn set_blend_mode(&self, blend_state: Option<BlendMode>) {
        let gl = &self.0.gl;
        match blend_state {
            Some(BlendMode {
                equation,
                function,
                global_color: [r, g, b, a],
            }) => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                // gl::BLEND is on the whitelist
                gl.enable(glow::BLEND);

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquation.xhtml
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparate.xhtml
                // The to_gl() function only produces valid values
                match equation {
                    BlendEquation::Same(eq) => gl.blend_equation(eq.to_gl()),
                    BlendEquation::Separate { color, alpha } => {
                        gl.blend_equation_separate(color.to_gl(), alpha.to_gl());
                    }
                }

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunc.xhtml
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparate.xhtml
                // The to_gl() function only produces valid values
                match function {
                    BlendFunction::Same {
                        source,
                        destination,
                    } => {
                        gl.blend_func(source.to_gl(), destination.to_gl());
                    }
                    BlendFunction::Separate {
                        source_color,
                        source_alpha,
                        destination_alpha,
                        destination_color,
                    } => {
                        gl.blend_func_separate(
                            source_color.to_gl(),
                            source_alpha.to_gl(),
                            destination_alpha.to_gl(),
                            destination_color.to_gl(),
                        );
                    }
                }

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendColor.xhtml
                gl.blend_color(r, g, b, a);
            },
            None => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                // gl::BLEND is on the whitelist
                gl.disable(glow::BLEND);
            },
        }
    }

    /// Set the depth test mode, with `None` disabling depth testing
    ///
    /// By default, this is `None`
    ///
    /// See the documentation for [`DepthTestMode`](depth/struct.DepthTestMode.html)
    /// for the various depth testing options
    pub fn set_depth_test_mode(&self, depth_test_state: Option<DepthTestMode>) {
        let gl = &self.0.gl;
        match depth_test_state {
            Some(DepthTestMode {
                function,
                range_near,
                range_far,
                depth_mask,
            }) => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                gl.enable(glow::DEPTH_TEST);
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthFunc.xhtml
                // The to_gl() function only produces valid values
                gl.depth_func(function.to_gl());

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRange.xhtml
                #[cfg(not(target_arch = "wasm32"))]
                gl.depth_range_f64(range_near as f64, range_far as f64);

                // https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDepthRangef.xhtml
                #[cfg(target_arch = "wasm32")]
                gl.depth_range_f32(range_near, range_far);

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthMask.xhtml
                gl.depth_mask(depth_mask);
            },
            None => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                gl.disable(glow::DEPTH_TEST);
            },
        }
    }
}
