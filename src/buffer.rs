use crate::*;

pub struct VertexBuffer(Buffer);

impl VertexBuffer {
    pub fn new(ctx: &Context) -> Result<VertexBuffer, GolemError> {
        Ok(VertexBuffer(Buffer::new(ctx)?))
    }

    pub fn set_data(&mut self, data: &[f32]) {
        self.0.set_data(glow::ARRAY_BUFFER, data);
    }

    pub fn set_sub_data(&mut self, start: usize, data: &[f32]) {
        self.0.set_sub_data(glow::ARRAY_BUFFER, start, data);
    }

    pub fn size(&self) -> usize {
        self.0.length
    }

    pub(crate) fn bind(&self) {
        self.0.bind(glow::ARRAY_BUFFER);
    }
}

pub struct ElementBuffer(Buffer);

impl ElementBuffer {
    pub fn new(ctx: &Context) -> Result<ElementBuffer, GolemError> {
        Ok(ElementBuffer(Buffer::new(ctx)?))
    }

    pub fn set_data(&mut self, data: &[u32]) {
        self.0.set_data(glow::ELEMENT_ARRAY_BUFFER, data);
    }
    
    pub fn set_sub_data(&mut self, start: usize, data: &[u32]) {
        self.0.set_sub_data(glow::ELEMENT_ARRAY_BUFFER, start, data);
    }

    pub fn size(&self) -> usize {
        self.0.length
    }

    pub(crate) fn bind(&self) {
        self.0.bind(glow::ELEMENT_ARRAY_BUFFER);
    }
}

pub(crate) struct Buffer {
    ctx: Context,
    id: GlBuffer,
    length: usize,
}

impl Buffer {
    fn new(ctx: &Context) -> Result<Buffer, GolemError> {
        let ctx = Context(ctx.0.clone());
        let id = unsafe { ctx.0.gl.create_buffer() }?;

        Ok(Buffer { ctx, id, length: 0 })
    }

    fn bind(&self, target: u32) {
        unsafe {
            self.ctx.0.gl.bind_buffer(target, Some(self.id));
        }
    }

    fn set_data<T: bytemuck::Pod>(&mut self, target: u32, data: &[T]) {
        let gl = &self.ctx.0.gl;

        let u8_buffer = bytemuck::cast_slice(data);
        let data_length = u8_buffer.len();
        self.bind(target);
        if data_length >= self.length {
            log::trace!("Resizing buffer to hold new data");
            let new_length = data_length * 2;
            unsafe {
                gl.buffer_data_size(target, new_length as i32, glow::STREAM_DRAW);
            }
            self.length = new_length;
        }
        log::trace!("Writing data to OpenGL buffer");
        unsafe {
            gl.buffer_sub_data_u8_slice(target, 0, u8_buffer);
        }
    }

    fn set_sub_data<T: bytemuck::Pod>(&self, target: u32, start: usize, data: &[T]) {
        let u8_buffer = bytemuck::cast_slice(data);
        let data_length = u8_buffer.len();
        assert!(start + data_length < self.length);
        log::trace!("Writing data to OpenGL buffer");
        unsafe {
            self.ctx.0.gl.buffer_sub_data_u8_slice(target, start as i32, u8_buffer);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_buffer(self.id);
        }
    }
}
