use glow::HasContext;
use crate::GolemError;
use crate::buffer::{Buffer, ElementBuffer, VertexBuffer};
use crate::objects::{ColorFormat, DrawList, Surface, Texture, UniformValue};
use crate::program::{Attribute, Position, Uniform, ShaderDescription, ShaderProgram};
use std::rc::Rc;

pub struct Context(Rc<ContextContents>);

struct ContextContents {
    gl: glow::Context,
    vao: u32
}

impl Drop for ContextContents {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

fn generate_shader_text(body: &str, inputs: &[Attribute], outputs: &[Attribute], uniforms: &[Uniform]) -> String {
    let mut shader = String::new();

    #[cfg(not(target_arch = "wasm32"))]
    shader.push_str("#version 150\n");

    shader.push_str("precision mediump float;\n");
    for attr in inputs.iter() {
        attr.as_glsl(Position::Input, &mut shader);
    }
    for attr in outputs.iter() {
        attr.as_glsl(Position::Output, &mut shader);
    }
    for uniform in uniforms.iter() {
        uniform.as_glsl(&mut shader);
    }
    shader.push_str(body);

    shader
}

impl Context {
    pub fn from_glow(gl: glow::Context) -> Context {
        #[cfg(not(target_arch = "wasm32"))]
        let vao = unsafe {
            let vao = gl.create_vertex_array().expect("Failed to create a VAO");
            gl.bind_vertex_array(Some(vao));

            vao
        };
        #[cfg(target_arch = "wasm32")]
        let vao = 0;

        let contents = Rc::new(ContextContents {
            gl,
            vao
        });


        Context(contents)
    }

    pub fn new_shader(&self, desc: ShaderDescription) -> Result<ShaderProgram, GolemError> {
        let gl = &self.0.gl;
        // TODO: check for shader creation errors
        // TODO: OpenGL will drop unused variables, that's probably going to bite me?
        unsafe {
            let vertex = gl.create_shader(glow::VERTEX_SHADER)?;
            let vertex_source = generate_shader_text(desc.vertex_shader, desc.vertex_input, desc.fragment_input, desc.uniforms);
            log::debug!("Vertex shader source: {}", vertex_source);
            gl.shader_source(vertex, &vertex_source);
            gl.compile_shader(vertex);
            if !gl.get_shader_compile_status(vertex) {
                let info = gl.get_shader_info_log(vertex);
                log::error!("Failed to compile vertex shader: {}", info);
                Err(GolemError::ShaderCompilationError(info))?
            }
            log::trace!("Compiled vertex shader succesfully");

            let fragment = gl.create_shader(glow::FRAGMENT_SHADER)?;
            // Handle creating the output color and giving it a name, but only on desktop gl
            #[cfg(target_arch = "wasm32")]
            let (fragment_output, fragment_body) = {
                (&[], desc.fragment_input)
            };
            #[cfg(not(target_arch = "wasm32"))]
            let (fragment_output, fragment_body) = {
                (&[ Attribute::Vector(4, "outputColor") ], &desc.fragment_shader.replace("gl_FragColor", "outputColor"))
            };
            let fragment_source = generate_shader_text(fragment_body, desc.fragment_input, fragment_output, desc.uniforms);
            log::debug!("Fragment shader source: {}", vertex_source);
            gl.shader_source(fragment, &fragment_source);
            gl.compile_shader(fragment);
            if !gl.get_shader_compile_status(fragment) {
                let info = gl.get_shader_info_log(fragment);
                log::error!("Failed to compile vertex shader: {}", info);
                Err(GolemError::ShaderCompilationError(info))?
            }
            log::trace!("Compiled fragment shader succesfully");

            let id = gl.create_program()?;

            gl.attach_shader(id, vertex);
            gl.attach_shader(id, fragment);

            // Bind the color output for desktop GL
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
            id,
            length: 0
        })
    }

    pub fn new_vertex_buffer(&self) -> Result<VertexBuffer, GolemError> {
        Ok(VertexBuffer(self.new_buffer()?))
    }

    pub fn new_element_buffer(&self) -> Result<ElementBuffer, GolemError> {
        Ok(ElementBuffer(self.new_buffer()?))
    }

    pub fn new_texture(&self, image: &[u8], width: u32, height: u32, color: ColorFormat) -> Result<Texture, GolemError> {
        assert!(width < glow::MAX_TEXTURE_SIZE);
        assert!(height < glow::MAX_TEXTURE_SIZE);
        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA
        };
        let gl = &self.0.gl;
        unsafe {
            let id = gl.create_texture()?;
            gl.bind_texture(glow::TEXTURE_2D, Some(id));
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, width as i32,
                            height as i32, 0, format, glow::UNSIGNED_BYTE, Some(image));
            // TODO: is this important
            //gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);

            Ok(Texture {
                ctx: Context(self.0.clone()),
                id,
            })
        }
    }

    pub fn bind_texture(&self, tex: &Texture, texture_unit: u32) {
        let gl = &self.0.gl;
        unsafe {
            gl.active_texture(glow::TEXTURE0 + texture_unit);
            gl.bind_texture(glow::TEXTURE_2D, Some(tex.id));
        }
    }

    pub(crate) fn bind(&self, buffer: &Buffer, target: u32) {
        unsafe {
            self.0.gl.bind_buffer(target, Some(buffer.id));
        }
    }

    pub(crate) fn send_data<T: bytemuck::Pod>(&self, bind: u32, length: usize, start: usize, data: &[T]) {
        use std::mem::size_of;
        let data_length = size_of::<T>() * data.len();
        let data_start = size_of::<T>() * start;
        let u8_buffer = bytemuck::cast_slice(data);
        let gl = &self.0.gl;
        unsafe {
            if data_length + start > length {
                log::trace!("Resizing buffer to hold new data");
                let new_length = data_length + data_start;
                gl.buffer_data_size(bind, new_length as i32 * 2, glow::STREAM_DRAW);
            }
            gl.buffer_sub_data_u8_slice(bind, start as i32, u8_buffer);
        };
    }

    pub fn set_target(&mut self, _surface: &Surface) {
        unimplemented!();
    }

    pub fn reset_target(&mut self) {
        unimplemented!();
    }

    pub fn clear(&mut self, r: f32, g: f32, b: f32, a: f32) {
        let gl = &self.0.gl;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.clear_color(r, g, b, a);
        }
    }

    // TODO: API allow glow::LINES
    pub fn draw(&mut self, shader: &ShaderProgram, vb: &VertexBuffer, eb: &ElementBuffer, draw_list: &[DrawList]) {
        let gl = &self.0.gl;
        log::trace!("Setting up the shader and buffers to draw");
        unsafe {
            gl.use_program(Some(shader.id));
        }
        self.bind(&vb.0, glow::ARRAY_BUFFER);
        self.bind(&eb.0, glow::ELEMENT_ARRAY_BUFFER);
        use std::mem::size_of;
        let stride: i32 = shader.input.iter().map(|attr| attr.size()).sum();
        let stride = stride * size_of::<f32>() as i32;
        let mut offset = 0;
        log::trace!("Binding the attributes to draw");
        for (index, attr) in shader.input.iter().enumerate() {
            let size = attr.size();
            unsafe {
                let pos_attrib = index as u32;
                gl.enable_vertex_attrib_array(pos_attrib);
                gl.vertex_attrib_pointer_f32(pos_attrib, size, glow::FLOAT, false, stride, offset);
            }
            offset += size * size_of::<f32>() as i32;
        }
        log::trace!("Beginning draw calls");
        draw_list.iter().for_each(|draw_list| {
            log::trace!("Binding uniform values");
            for (name, value) in draw_list.uniforms.iter() { 
                let location = unsafe { gl.get_uniform_location(shader.id, name) };
                self.bind_uniform(location.expect("Generated uniform was not found in shader"), value.clone());
            }
            log::trace!("Dispatching draw commands");
            let range = draw_list.range.clone();
            let length = range.end - range.start;
            unsafe {
                gl.draw_elements(glow::TRIANGLES, length as i32, glow::UNSIGNED_INT, range.start as i32);
            }
        });
    }

    fn bind_uniform(&self, location: u32, uniform: UniformValue) {
        use UniformValue::*;
        let location = Some(location);
        let gl = &self.0.gl;
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
            }
        }
    }

    pub(crate) fn delete_shader(&self, id: u32, fragment: u32, vertex: u32) {
        let gl = &self.0.gl;
        unsafe {
            gl.delete_program(id);
            gl.delete_shader(fragment);
            gl.delete_shader(vertex);
        }
    }

    pub(crate) fn delete_buffer(&self, id: u32) {
        unsafe {
            self.0.gl.delete_buffer(id);
        }
    }

    pub(crate) fn delete_texture(&self, id: u32) {
        unsafe {
            self.0.gl.delete_texture(id);
        }
    }

    pub(crate) fn delete_surface(&self, _id: u32) {
        unimplemented!();
    }
}
