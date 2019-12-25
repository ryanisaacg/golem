use crate::*;

/// A framebuffer that allows render-to-texture
pub struct Surface {
    pub(crate) ctx: Context,
    pub(crate) id: GlFramebuffer,
    pub(crate) texture: Texture,
}

impl Surface {
    /// Create a new Surface to render to, backed by the given texture
    pub fn new(ctx: &Context, texture: Texture) -> Result<Surface, GolemError> {
        let ctx = Context(ctx.0.clone());
        let gl = &ctx.0.gl;
        let id = unsafe { gl.create_framebuffer() }?;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(id));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture.id),
                0,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, *ctx.0.current_surface.borrow());
        }

        Ok(Surface { ctx, id, texture })
    }

    /// Set the current render target to this surface
    ///
    /// Also necessary for operations like [`Surface::get_pixel_data`]
    pub fn bind(&self) {
        *self.ctx.0.current_surface.borrow_mut() = Some(self.id);
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
        }
    }

    /// Unbind the surface and set the render target to the screen
    pub fn unbind(ctx: &Context) {
        *ctx.0.current_surface.borrow_mut() = None;
        let gl = &ctx.0.gl;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    /// Get a reference to the bound texture
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Get a mutable reference to the bound texture
    pub fn texture_mut(&mut self) -> &mut Texture {
        &mut self.texture
    }

    /// Check if this surface is bound to be operated on
    pub fn is_bound(&self) -> bool {
        match *self.ctx.0.current_surface.borrow() {
            Some(surface) => self.id == surface,
            None => false,
        }
    }

    /// Get the pixel data and write it to a buffer
    ///
    /// The surface must be bound first, see [`Surface::bind`].
    ///
    /// The ColorFormat determines how many bytes each pixel is: 3 bytes for RGB and 4 for RGBA. The
    /// slice needs have a length of `(width - x) * (height - y) * ColorFormat size`.
    pub fn get_pixel_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: ColorFormat,
        data: &mut [u8],
    ) {
        assert!(
            self.is_bound(),
            "The surface wasn't bound when get_pixel_data was called"
        );
        let bytes_per_pixel = format.bytes_per_pixel();
        let length = (width * height * bytes_per_pixel) as usize;
        assert!(
            data.len() >= length,
            "The buffer was not large enough to hold the data"
        );
        let format = format.gl_format();
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
            gl.read_pixels(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                glow::UNSIGNED_BYTE,
                data,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_framebuffer(self.id);
        }
    }
}
