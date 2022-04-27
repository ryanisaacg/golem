use blinds::*;
use golem::{
    Attribute, AttributeType, Context,
    Dimension::{D2, D4},
    ElementBuffer, GeometryMode, GolemError, ShaderDescription, ShaderProgram, VertexBuffer,
};

async fn app(window: Window, mut events: EventStream) -> Result<(), GolemError> {
    #[cfg(not(target_arch = "wasm32"))]
    let gl =
        unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) };
    #[cfg(target_arch = "wasm32")]
    let gl = unsafe { glow::Context::from_webgl1_context(window.webgl_context()) };
    let ctx = Context::from_glow(gl)?;

    #[rustfmt::skip]
    let vertices = [
        // Position         Color
        -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
        0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
        0.5, 0.5,           0.0, 0.0, 1.0, 1.0,
        -0.5, 0.5,          1.0, 1.0, 1.0, 1.0,
    ];
    let indices = [0, 1, 1, 2, 2, 3, 3, 0];

    let mut shader = ShaderProgram::new(
        &ctx,
        ShaderDescription {
            vertex_input: &[
                Attribute::new("vert_position", AttributeType::Vector(D2)),
                Attribute::new("vert_color", AttributeType::Vector(D4)),
            ],
            fragment_input: &[Attribute::new("frag_color", AttributeType::Vector(D4))],
            uniforms: &[],
            vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
            fragment_shader: r#" void main() {
            gl_FragColor = frag_color;
        }"#,
        },
    )?;

    let mut vb = VertexBuffer::new(&ctx)?;
    let mut eb = ElementBuffer::new(&ctx)?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind();

    ctx.clear();
    unsafe {
        shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Lines)?;
    }
    window.present();

    loop {
        events.next_event().await;
    }
}

fn main() {
    run(Settings::default(), |window, events| async move {
        app(window, events).await.unwrap()
    });
}
