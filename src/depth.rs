pub struct DepthTestMode {
    pub function: DepthTestFunction,
    pub range_near: f64,
    pub range_far: f64,
}

pub enum DepthTestFunction {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always,
}

impl DepthTestFunction {
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn to_gl(self) -> u32 {
        use DepthTestFunction::*;
        match self {
            Never => glow::NEVER,
            Less => glow::LESS,
            Equal => glow::EQUAL,
            LessOrEqual => glow::LEQUAL,
            Greater => glow::GREATER,
            NotEqual => glow::NOTEQUAL,
            GreaterOrEqual => glow::GEQUAL,
            Always => glow::ALWAYS,
        }
    }
}
