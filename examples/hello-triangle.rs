use quick_lifecycle::*;
use quick_graphics::*;


fn main() -> Result<(), GraphicsError> {
    let ctx = Context::new();
    let shader = ctx.create_shader(r#"
    in vec2 vert_position;
    in vec4 vert_color;
    out vec4 frag_color;
    void main() {
        gl_Position = vec4(vert_position, 0, 1);
        frag_color = vert_color;
    }"#, r#"
    in vec4 frag_color;
    out vec4 color;
    void main() {
        color = frag_color;
    }"#)?;


}
