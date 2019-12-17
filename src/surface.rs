use crate::*;

pub struct Surface {
    pub(crate) ctx: Context,
    pub(crate) id: GlFramebuffer,
    pub(crate) texture: Option<Texture>,
}

impl Surface {
    pub fn new(ctx: &Context) -> Result<Surface, GolemError> {
        let ctx = Context(ctx.0.clone());
        let id = unsafe { ctx.0.gl.create_framebuffer() }?;
        
        Ok(Surface {
            ctx,
            id,
            texture: None
        })
    }

    pub fn bind(ctx: &Context, surface: Option<&Surface>) {
        unsafe {
            ctx.0.gl.bind_framebuffer(glow::FRAMEBUFFER, surface.map(|s| s.id));
        }
    }

    pub fn texture(&self) -> Option<&Texture> {
        self.texture.as_ref()
    }

    pub fn texture_mut(&mut self) -> Option<&mut Texture> {
        self.texture.as_mut()
    }

    pub fn set_texture(&mut self, texture: Option<Texture>) {
        let gl = &self.ctx.0.gl;
        self.texture = texture;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
            gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, self.texture.as_ref().map(|tex| tex.id), 0);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    pub fn get_pixels(&self, x: u32, y: u32, width: u32, height: u32, format: ColorFormat) -> Vec<u8> {
        let bytes_per_pixel = match format {
            ColorFormat::RGBA => 4,
            ColorFormat::RGB => 3
        };
        let format = match format {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA
        };
        let length = (width * height * bytes_per_pixel) as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
            let pointer = buffer.as_mut_slice();
            gl.read_pixels(x as i32, y as i32, width as i32, height as i32, format, glow::UNSIGNED_BYTE, pointer);
            buffer.set_len(length);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        buffer
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_framebuffer(self.id);
        }
    }
}
