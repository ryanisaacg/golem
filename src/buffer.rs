use crate::Context;

pub struct VertexBuffer(pub(crate) Buffer);

impl VertexBuffer {
    pub fn set_data(&mut self, data: &[f32]) {
        self.0.set_data(glow::ARRAY_BUFFER, data);
    }
}

pub struct ElementBuffer(pub(crate) Buffer);

impl ElementBuffer {
    pub fn set_data(&mut self, data: &[u32]) {
        self.0.set_data(glow::ELEMENT_ARRAY_BUFFER, data);
    }
}

pub(crate) struct Buffer {
    pub(crate) ctx: Context,
    pub(crate) contents: BufferContents,
}

pub(crate) struct BufferContents {
    pub(crate) id: u32,
    pub(crate) length: usize,
}

impl Buffer {
    fn set_data<T: bytemuck::Pod>(&mut self, target: u32, data: &[T]) {
        self.ctx.set_data(&mut self.contents, target, data);
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.ctx.delete_buffer(&self.contents);
    }
}
