use blinds::*;
use golem::{
    Attribute, AttributeType, ColorFormat, Context, Dimension::D2, ElementBuffer, GeometryMode,
    GolemError, ShaderDescription, ShaderProgram, Texture, Uniform, UniformType, UniformValue,
    VertexBuffer,
};

async fn app(window: Window, mut events: EventStream) -> Result<(), GolemError> {
    #[cfg(not(target_arch = "wasm32"))]
    let gl =
        unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) };
    #[cfg(target_arch = "wasm32")]
    let gl = unsafe { glow::Context::from_webgl1_context(window.webgl_context()) };
    let ctx = Context::from_glow(gl)?;

    #[rustfmt::skip]
    let image = [
        // R, G, B
        255, 255, 255,
        0, 255, 0,
        255, 0, 0,
        255, 255, 255,
        0, 0, 255
    ];

    let mut texture = Texture::new(&ctx)?;
    texture.set_image(Some(&image), 2, 2, ColorFormat::RGB);

    #[rustfmt::skip]
    let vertices = [
        // Position         UV
        -0.5, -0.5,         0.0, 0.0,
        0.5, -0.5,          1.0, 0.0,
        0.5, 0.5,           1.0, 1.0,
        -0.5, 0.5,          0.0, 1.0,
    ];
    let indices = [0, 1, 2, 2, 3, 0];

    let mut shader = ShaderProgram::new(
        &ctx,
        ShaderDescription {
            vertex_input: &[
                Attribute::new("vert_position", AttributeType::Vector(D2)),
                Attribute::new("vert_uv", AttributeType::Vector(D2)),
            ],
            fragment_input: &[Attribute::new("frag_uv", AttributeType::Vector(D2))],
            uniforms: &[Uniform::new("image", UniformType::Sampler2D)],
            vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_uv = vert_uv;
        }"#,
            fragment_shader: r#" void main() {
            gl_FragColor = texture(image, frag_uv);
        }"#,
        },
    )?;

    let mut vb = VertexBuffer::new(&ctx)?;
    let mut eb = ElementBuffer::new(&ctx)?;
    vb.set_data(&vertices);
    eb.set_data(&indices);
    shader.bind();
    shader.set_uniform("image", UniformValue::Int(1))?;

    let bind_point = std::num::NonZeroU32::new(1).unwrap();
    texture.set_active(bind_point);

    ctx.clear();
    unsafe {
        shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
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
