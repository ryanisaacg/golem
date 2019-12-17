use crate::{Position, Dimension};

#[derive(Clone)]
pub struct Attribute {
    name: &'static str,
    value: AttributeType,
}

#[derive(Clone)]
pub enum AttributeType {
    Scalar,
    Vector(Dimension),
    Matrix(Dimension, Dimension),
}

impl Position {
    #[cfg(target_arch = "wasm32")]
    fn glsl_string(self) -> &'static str {
        use Position::*;

        match self {
            Input => "attribute ",
            Output => "varying ",
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn glsl_string(self) -> &'static str {
        use Position::*;

        match self {
            Input => "in ",
            Output => "out ",
        }
    }
}

impl Attribute {
    pub fn new(name: &'static str, value: AttributeType) -> Attribute {
        Attribute {
            name,
            value
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn size(&self) -> i32 {
        use AttributeType::*;

        match self.value {
            Scalar => 1,
            Vector(n) => n as i32,
            Matrix(m, n) => (m as i32) * (n as i32),
        }
    }

    pub(crate) fn as_glsl(&self, _is_vertex: bool, pos: Position, shader: &mut String) {
        use AttributeType::*;

        #[cfg(target_arch = "wasm32")]
        let pos = if _is_vertex { pos } else { Position::Output };

        shader.push_str(pos.glsl_string());
        let gl_type = match self.value {
            Scalar => "float ".to_owned(),
            Vector(n) => format!("vec{} ", n as i32),
            Matrix(m, n) => format!("mat{}x{} ", m as i32, n as i32),
        };
        shader.push_str(&gl_type);
        shader.push_str(self.name());
        shader.push_str(";");
    }
}
