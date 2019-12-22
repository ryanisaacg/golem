use super::*;

// TODO: unsafe verification

pub struct Texture {
    pub(crate) ctx: Context,
    pub(crate) id: GlTexture,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Texture {
    pub fn new(ctx: &Context) -> Result<Texture, GolemError> {
        let ctx = Context(ctx.0.clone());
        let id = unsafe { ctx.0.gl.create_texture()? };
        // TODO: is this an acceptable state to have a texture
        let tex = Texture {
            ctx,
            id,
            width: 0,
            height: 0,
        };
        tex.set_minification(TextureFilter::Linear);

        Ok(tex)
    }

    pub fn bind(ctx: &Context, tex: Option<&Texture>, bind_point: u32) {
        let gl = &ctx.0.gl;
        let value = tex.map(|tex| tex.id);
        unsafe {
            gl.active_texture(glow::TEXTURE0 + bind_point);
            gl.bind_texture(glow::TEXTURE_2D, value);
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_image(&mut self, data: Option<&[u8]>, width: u32, height: u32, color: ColorFormat) {
        // TODO: make into a recoverable error?
        assert!(width < glow::MAX_TEXTURE_SIZE);
        assert!(height < glow::MAX_TEXTURE_SIZE);
        self.width = width;
        self.height = height;

        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA,
        };
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width as i32,
                height as i32,
                0,
                format,
                glow::UNSIGNED_BYTE,
                data,
            );
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn set_subimage(
        &self,
        data: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: ColorFormat,
    ) {
        assert!(x + width <= self.width);
        assert!(y + height <= self.height);
        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA,
        };
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_sub_image_2d_u8_slice(
                glow::TEXTURE_2D,
                0,
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                glow::UNSIGNED_BYTE,
                Some(data),
            );
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    fn set_texture_param(&self, param: u32, value: i32) {
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_parameter_i32(glow::TEXTURE_2D, param, value);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn set_minification(&self, min: TextureFilter) {
        self.set_texture_param(glow::TEXTURE_MIN_FILTER, min.to_gl());
    }

    pub fn set_magnification(&self, max: TextureFilter) {
        self.set_texture_param(glow::TEXTURE_MAG_FILTER, max.to_gl());
    }

    pub fn set_wrap_h(&self, wrap: TextureWrap) {
        self.set_texture_param(glow::TEXTURE_WRAP_S, wrap.to_gl());
    }

    pub fn set_wrap_v(&self, wrap: TextureWrap) {
        self.set_texture_param(glow::TEXTURE_WRAP_T, wrap.to_gl());
    }
}

pub enum TextureFilter {
    Linear,
    Nearest,
}

impl TextureFilter {
    pub(crate) fn to_gl(&self) -> i32 {
        match self {
            TextureFilter::Linear => glow::LINEAR as i32,
            TextureFilter::Nearest => glow::NEAREST as i32,
        }
    }
}

pub enum TextureWrap {
    Repeat,
    ClampToEdge,
    MirroredRepeat,
}

impl TextureWrap {
    pub(crate) fn to_gl(&self) -> i32 {
        match self {
            TextureWrap::Repeat => glow::REPEAT as i32,
            TextureWrap::ClampToEdge => glow::CLAMP_TO_EDGE as i32,
            TextureWrap::MirroredRepeat => glow::MIRRORED_REPEAT as i32,
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_texture(self.id);
        }
    }
}
