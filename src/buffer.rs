use crate::*;

pub type VertexBuffer = Buffer<f32>;

pub type ElementBuffer = Buffer<u32>;

pub struct Buffer<T> {
    ctx: Context,
    id: GlBuffer,
    length: usize,
    _p: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> Buffer<T> {
    pub fn new(ctx: &Context) -> Result<Self, GolemError> {
        let ctx = Context(ctx.0.clone());
        let id = unsafe { ctx.0.gl.create_buffer() }?;

        Ok(Buffer {
            ctx,
            id,
            length: 0,
            _p: std::marker::PhantomData,
        })
    }

    pub(crate) fn bind(&self, target: u32) {
        unsafe {
            self.ctx.0.gl.bind_buffer(target, Some(self.id));
        }
    }

    pub fn size(&self) -> usize {
        self.length
    }

    pub fn set_data(&mut self, target: u32, data: &[T]) {
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

    pub fn set_sub_data(&self, target: u32, start: usize, data: &[T]) {
        let u8_buffer = bytemuck::cast_slice(data);
        let data_length = u8_buffer.len();
        assert!(start + data_length < self.length);
        log::trace!("Writing data to OpenGL buffer");
        unsafe {
            self.ctx
                .0
                .gl
                .buffer_sub_data_u8_slice(target, start as i32, u8_buffer);
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_buffer(self.id);
        }
    }
}
