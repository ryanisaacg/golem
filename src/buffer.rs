use crate::Context;

pub struct VertexBuffer(pub(crate) Buffer);

impl VertexBuffer {
    pub fn send_data(&mut self, start: usize, data: &[f32]) {
        self.0.send_data(glow::ARRAY_BUFFER, start, data);
    }
}

pub struct ElementBuffer(pub(crate) Buffer);

impl ElementBuffer {
    pub fn send_data(&mut self, start: usize, data: &[u32]) {
        self.0.send_data(glow::ELEMENT_ARRAY_BUFFER, start, data);
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
    fn send_data<T: bytemuck::Pod>(&mut self, target: u32, start: usize, data: &[T]) {
        self.ctx.send_data(&mut self.contents, target, start, data);
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        self.ctx.delete_buffer(self.id);
    }
}
