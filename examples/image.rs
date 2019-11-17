use blinds::traits::*;
use blinds::{EventStream, Window, Settings, run};
use golem::buffers::{VertexBuffer, ElementBuffer};
use golem::shaders::{Attribute, VertexShader, FragmentShader, ShaderProgram};

// TODO: how to handle image?

static VERTEX_SHADER: &'static str = r#"
void main() {
    gl_Position = vec4(v_pos, 0, 1);
    frag_uv = vert_uv;
}"#;
static FRAGMENT_SHADER: &'static str = r#"
void main() {
    gl_FragColor = texture(tex, frag_uv);
}"#;

static VERTICES: [Vec2; 4] = [
    Vec2::new(0.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, 0.0),
];

async fn app(window: Window, gl: RawContext, mut events: EventStream) -> Result<(), GraphicsError> {
    let ctx = Context::from_glow(gl);

    let texture = ctx.upload_texture_from_bytes(include_bytes!("image.png"));

    let vertex_input = &[
        Attribute::new::<Vec2>("v_pos"),
    ];
    let fragment_input = &[
        Attribute::new::<Vec2>("frag_uv"),
    ];
    let uniforms = &[
        Attribute::new::<Sampler2D>("tex")
    ];
    let vertex_shader = VertexShader::new(vertex_input, fragment_input, VERTEX_SHADER);
    let fragment_shader = FragmentShader::new(fragment_input, FRAGMENT_SHADER);
    let shader = ShaderProgram::new(&mut ctx, vertex_shader, fragment_shader, uniforms)?;

    let vb = VertexBuffer::new(vertex_input);
    let eb = ElementBuffer::new();
    let mut vb_data = VertexBufferBuilder::new();
    vb.send_data(0, VERTICES.map(|(pos, uv)| vb.new_vertex().add(pos).add(uv)));
    ebo.send_data(0, &[0, 1, 2]);

    let surface = ctx.window_surface();

    while let Some(event) = events.next().await {
        if let Event::Draw = event {
            ctx.clear(Color::BLACK);
            let uniforms = &[
                ("tex", &texture)
            ];
            ctx.render(surface, &shader, &vb, &eb);
        }
    }
}

fn main() {
    run(Settings::default(), app);
}
