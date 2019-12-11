use glow::HasContext;
use crate::{GolemError, GlFramebuffer, GlProgram, GlShader, GlTexture};
use crate::buffer::{Buffer, BufferContents, ElementBuffer, VertexBuffer};
use crate::objects::{ColorFormat, GeometryType, Surface, Texture, UniformValue};
use crate::shader::{Attribute, AttributeType, Dimension::*, Position, Uniform, ShaderDescription, ShaderProgram};
use std::mem::size_of;
use std::ops::Range;
use std::rc::Rc;

pub struct Context(Rc<ContextContents>);

struct ContextContents {
    gl: glow::Context,
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

fn generate_shader_text(is_vertex: bool, body: &str, inputs: &[Attribute], outputs: &[Attribute], uniforms: &[Uniform]) -> String {
    let mut shader = String::new();

    #[cfg(not(target_arch = "wasm32"))]
    shader.push_str("#version 150\n");

    shader.push_str("precision mediump float;\n");
    for attr in inputs.iter() {
        attr.as_glsl(is_vertex, Position::Input, &mut shader);
    }
    for attr in outputs.iter() {
        attr.as_glsl(is_vertex, Position::Output, &mut shader);
    }
    for uniform in uniforms.iter() {
        uniform.as_glsl(&mut shader);
    }
    shader.push_str(body);

    shader
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

        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml
        // Set the default clear color to (0, 0, 0, 1)
        unsafe { gl.clear_color(0.0, 0.0, 0.0, 1.0) };

        let contents = Rc::new(ContextContents {
            gl,
            #[cfg(not(target_arch = "wasm32"))]
            vao,
        });


        Ok(Context(contents))
    }

    pub fn new_shader(&self, desc: ShaderDescription) -> Result<ShaderProgram, GolemError> {
        let gl = &self.0.gl;
        unsafe {
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml
            // Errors:
            // 1. An error occurred creating the shader (handled by glow's error layer)
            // 2. An invalid value was passed (VERTEX_SHADER is valid)
            let vertex = gl.create_shader(glow::VERTEX_SHADER)?;
            let vertex_source = generate_shader_text(true, desc.vertex_shader, desc.vertex_input, desc.fragment_input, desc.uniforms);
            log::debug!("Vertex shader source: {}", vertex_source);
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
            // Errror conditions:
            // 1 & 2. Vertex isn't a GL shader (it always will be)
            // 3. Shader size is handled by glow
            gl.shader_source(vertex, &vertex_source);
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml
            // Errror conditions: Vertex isn't a GL shader (it always will be)
            gl.compile_shader(vertex);
            if !gl.get_shader_compile_status(vertex) {
                let info = gl.get_shader_info_log(vertex);
                log::error!("Failed to compile vertex shader: {}", info);
                Err(GolemError::ShaderCompilationError(info))?
            }
            log::trace!("Compiled vertex shader succesfully");

            // For GL pre/post condition explanations, see vertex shader compilation above
            let fragment = gl.create_shader(glow::FRAGMENT_SHADER)?;
            // Handle creating the output color and giving it a name, but only on desktop gl
            #[cfg(target_arch = "wasm32")]
            let (fragment_output, fragment_body) = {
                (&[], desc.fragment_shader)
            };
            #[cfg(not(target_arch = "wasm32"))]
            let (fragment_output, fragment_body) = {
                (&[ Attribute::new("outputColor", AttributeType::Vector(D4)) ],
                &desc.fragment_shader.replace("gl_FragColor", "outputColor"))
            };
            let fragment_source = generate_shader_text(false, fragment_body, desc.fragment_input, fragment_output, desc.uniforms);
            log::debug!("Fragment shader source: {}", vertex_source);
            gl.shader_source(fragment, &fragment_source);
            gl.compile_shader(fragment);
            if !gl.get_shader_compile_status(fragment) {
                let info = gl.get_shader_info_log(fragment);
                log::error!("Failed to compile vertex shader: {}", info);
                Err(GolemError::ShaderCompilationError(info))?
            }
            log::trace!("Compiled fragment shader succesfully");

            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml
            // Failing to create a program is handled by glow
            let id = gl.create_program()?;

            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml
            // Errors:
            // 1, 2, 3: id, vertex, and fragment are all assigned to once, by the correct GL calls
            // 4: vertex and fragment are generated then immediately attached exactly once
            gl.attach_shader(id, vertex);
            gl.attach_shader(id, fragment);

            // Bind the color output for desktop GL
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocation.xhtml
            // Errors:
            // 1. colorNumber will always be 0, and therefore cannot overrun the bounds
            // 2. 'outputColor' does not started with the reserved 'gl_' prefix
            // 3. 'id' is generated by create_program above
            #[cfg(not(target_arch = "wasm32"))]
            gl.bind_frag_data_location(id, 0, "outputColor");

            for (index, attr) in desc.vertex_input.iter().enumerate() {
                gl.bind_attrib_location(id, index as u32, attr.name());
            }

            gl.link_program(id);
            if !gl.get_program_link_status(id) {
                let info = gl.get_program_info_log(id);
                log::error!("Failed to link the shader program: {}", info);
                Err(GolemError::ShaderCompilationError(info))?
            }
            log::trace!("Linked shader program succesfully");

            Ok(ShaderProgram {
                ctx: Context(self.0.clone()),
                id,
                vertex,
                fragment,
                input: desc.vertex_input.iter().cloned().collect(),
            })
        }
    }

    fn new_buffer(&self) -> Result<Buffer, GolemError> {
        let id = unsafe { self.0.gl.create_buffer() }?;
        let ctx = Context(self.0.clone());

        Ok(Buffer {
            ctx,
            contents: BufferContents {
                id,
                length: 0
            }
        })
    }

    pub fn new_vertex_buffer(&self) -> Result<VertexBuffer, GolemError> {
        Ok(VertexBuffer(self.new_buffer()?))
    }

    pub fn new_element_buffer(&self) -> Result<ElementBuffer, GolemError> {
        Ok(ElementBuffer(self.new_buffer()?))
    }

    pub fn new_texture(&self, image: Option<&[u8]>, width: u32, height: u32, color: ColorFormat) -> Result<Texture, GolemError> {
        assert!(width < glow::MAX_TEXTURE_SIZE);
        assert!(height < glow::MAX_TEXTURE_SIZE);
        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA
        };
        let gl = &self.0.gl;
        unsafe {
            let tex = gl.create_texture()?;
            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, width as i32,
                            height as i32, 0, format, glow::UNSIGNED_BYTE, image);
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
            let ctx = Context(self.0.clone());
            
            Ok(Texture {
                ctx,
                id: tex,
            })
        }
    }

    pub fn bind_texture(&self, tex: Option<&Texture>, texture_unit: u32) {
        let gl = &self.0.gl;
        let value = tex.map(|tex| tex.id);
        unsafe {
            gl.active_texture(glow::TEXTURE0 + texture_unit);
            gl.bind_texture(glow::TEXTURE_2D, value);
        }
    }

    pub(crate) fn bind(&self, buffer: &BufferContents, target: u32) {
        unsafe {
            self.0.gl.bind_buffer(target, Some(buffer.id));
        }
    }

    pub(crate) fn set_data<T: bytemuck::Pod>(&self, buffer: &mut BufferContents, target: u32, data: &[T]) {
        let u8_buffer = bytemuck::cast_slice(data);
        let data_length = u8_buffer.len();
        self.bind(buffer, target);
        let gl = &self.0.gl;
        if data_length >= buffer.length {
            log::trace!("Resizing buffer to hold new data");
            let new_length = data_length * 2;
            unsafe {
                gl.buffer_data_size(target, new_length as i32, glow::STREAM_DRAW);
            }
            buffer.length = new_length;
        }
        log::trace!("Writing data to OpenGL buffer");
        unsafe {
            gl.buffer_sub_data_u8_slice(target, 0, u8_buffer);
        }
    }

    pub fn new_surface(&self, width: u32, height: u32, format: ColorFormat) -> Result<Surface, GolemError> {
        let gl = &self.0.gl;
        let id = unsafe { gl.create_framebuffer() }?;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(id));
        }
        let texture = self.new_texture(None, width, height, format)?;
        unsafe {
            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(texture.id), 0);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
        let ctx = Context(self.0.clone());
        
        Ok(Surface {
            ctx,
            id,
            texture,
        })
    }

    pub fn set_target(&self, surface: Option<&Surface>) {
        unsafe {
            self.0.gl.bind_framebuffer(glow::FRAMEBUFFER, surface.map(|s| s.id));
        }
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
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

    pub(crate) fn bind_program(&self, id: GlProgram, input: &Vec<Attribute>, vb: &VertexBuffer) {
        let gl = &self.0.gl;
        log::trace!("Binding the shader and buffers");
        unsafe {
            gl.use_program(Some(id));
        }
        self.bind(&vb.0.contents, glow::ARRAY_BUFFER);
        let stride: i32 = input.iter().map(|attr| attr.size()).sum();
        let stride = stride * size_of::<f32>() as i32;
        let mut offset = 0;
        log::trace!("Binding the attributes to draw");
        for (index, attr) in input.iter().enumerate() {
            let size = attr.size();
            unsafe {
                let pos_attrib = index as u32;
                gl.enable_vertex_attrib_array(pos_attrib);
                gl.vertex_attrib_pointer_f32(pos_attrib, size, glow::FLOAT, false, stride, offset);
            }
            offset += size * size_of::<f32>() as i32;
        }
    }

    pub(crate) fn is_program_bound(&self, id: GlProgram) -> bool {
        // TODO: web implementation
        true
        /*unsafe {
            self.0.gl.get_parameter_i32(glow::CURRENT_PROGRAM) == id as i32
        }*/
    }

    pub fn draw(&self, eb: &ElementBuffer, range: Range<usize>) -> Result<(), GolemError> {
        self.draw_with_type(eb, range, GeometryType::Triangles)
    }

    pub fn draw_with_type(&self, eb: &ElementBuffer, range: Range<usize>, geometry: GeometryType) -> Result<(), GolemError> {
        // TODO web implementation
        let program = unsafe { self.0.gl.get_parameter_i32(glow::CURRENT_PROGRAM) } + 1;
        if program == 0 {
            Err(GolemError::NoBoundProgram)
        } else {
            self.bind(&eb.0.contents, glow::ELEMENT_ARRAY_BUFFER);
            log::trace!("Dispatching draw command");
            let length = range.end - range.start;
            use GeometryType::*;
            let shape_type = match geometry {
                Points => glow::POINTS,
                Lines => glow::LINES,
                LineStrip => glow::LINE_STRIP,
                LineLoop => glow::LINE_LOOP,
                TriangleStrip => glow::TRIANGLE_STRIP,
                TriangleFan => glow::TRIANGLE_FAN,
                Triangles => glow::TRIANGLES,
            };
            unsafe {
                self.0.gl.draw_elements(shape_type, length as i32, glow::UNSIGNED_INT, range.start as i32);
            }

            Ok(())
        }
    }


    pub(crate) fn bind_uniform(&self, id: GlProgram, name: &str, uniform: UniformValue) -> Result<(), GolemError> {
        let gl = &self.0.gl;
        let location = unsafe { gl.get_uniform_location(id, name) };
        use UniformValue::*;
        unsafe {
            match uniform {
                Int(x) => gl.uniform_1_i32(location, x),
                IVector2([x, y]) => gl.uniform_2_i32(location, x, y),
                IVector3([x, y, z]) => gl.uniform_3_i32(location, x, y, z),
                IVector4([x, y, z, w]) => gl.uniform_4_i32(location, x, y, z, w),
                Float(x) => gl.uniform_1_f32(location, x),
                Vector2([x, y]) => gl.uniform_2_f32(location, x, y),
                Vector3([x, y, z]) => gl.uniform_3_f32(location, x, y, z),
                Vector4([x, y, z, w]) => gl.uniform_4_f32(location, x, y, z, w),
                Matrix2(mat) => gl.uniform_matrix_2_f32_slice(location, false, &mat),
                Matrix3(mat) => gl.uniform_matrix_3_f32_slice(location, false, &mat),
                Matrix4(mat) => gl.uniform_matrix_4_f32_slice(location, false, &mat),
            }
        }

        Ok(())
    }

    pub(crate) fn delete_shader(&self, id: GlProgram, fragment: GlShader, vertex: GlShader) {
        let gl = &self.0.gl;
        unsafe {
            gl.delete_program(id);
            gl.delete_shader(fragment);
            gl.delete_shader(vertex);
        }
    }

    pub(crate) fn delete_buffer(&self, contents: &BufferContents) {
        unsafe {
            self.0.gl.delete_buffer(contents.id);
        }
    }

    pub(crate) fn delete_texture(&self, id: GlTexture) {
        unsafe {
            self.0.gl.delete_texture(id);
        }
    }

    pub(crate) fn delete_surface(&self, id: GlFramebuffer) {
        unsafe {
            self.0.gl.delete_framebuffer(id);
        }
    }
}
