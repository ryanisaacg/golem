use glow::HasContext;
use crate::{Context, GlFramebuffer, GlTexture};

// TODO: unsafe verification

pub struct Texture {
    pub(crate) ctx: Context,
    pub(crate) id: GlTexture,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Texture {
    pub fn set_image(&self, data: Option<&[u8]>, width: u32, height: u32, color: ColorFormat) {
        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA
        };
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, width as i32,
                            height as i32, 0, format, glow::UNSIGNED_BYTE, data);
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    pub fn set_subimage(&self, data: &[u8], x: u32, y: u32, width: u32, height: u32, color: ColorFormat) {
        assert!(x + width <= self.width);
        assert!(y + height <= self.height);
        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA
        };
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_sub_image_2d_u8_slice(glow::TEXTURE_2D, 0, x as i32, y as i32, width as i32,
                            height as i32, format, glow::UNSIGNED_BYTE, Some(data));
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


pub enum TextureFilter { Linear, Nearest }

impl TextureFilter {
    pub(crate) fn to_gl(&self) -> i32 {
        match self {
            TextureFilter::Linear => glow::LINEAR as i32,
            TextureFilter::Nearest => glow::NEAREST as i32,
        }
    }
}

pub enum TextureWrap { Repeat, ClampToEdge, MirroredRepeat }

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
        self.ctx.delete_texture(self.id);
    }
}

pub struct Surface {
    pub(crate) ctx: Context,
    pub(crate) id: GlFramebuffer,
    pub(crate) texture: Texture
}

impl Surface {
    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        self.ctx.delete_surface(self.id);
    }
}


#[derive(Clone)]
pub enum UniformValue {
    Int(i32),
    Float(f32),
    Vector2([f32; 2]),
    Vector3([f32; 3]),
    Vector4([f32; 4]),
    IVector2([i32; 2]),
    IVector3([i32; 3]),
    IVector4([i32; 4]),
    Matrix2([f32; 4]),
    Matrix3([f32; 9]),
    Matrix4([f32; 16]),
}

pub enum ColorFormat {
    RGB, RGBA
}

pub enum GeometryType {
    Points, Lines, LineStrip, LineLoop, TriangleStrip, TriangleFan, Triangles
}
