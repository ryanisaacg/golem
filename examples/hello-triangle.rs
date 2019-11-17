use blinds::traits::*;
use blinds::*;
use golem::{Context, GolemError};
use golem::input::{Color, Uniforms, Vec2, Vec4, VertexBuilder, vec2, rgba};
use golem::program::{Attribute, ShaderDescription};

async fn app(_window: Window, ctx: glow::Context, mut events: EventStream) -> Result<(), GolemError> {
    let mut ctx = Context::from_glow(ctx);

    let vertices: [(Vec2, Vec4); 3] = [
        (vec2(-0.5, -0.5),  rgba(1.0, 0.0, 0.0, 1.0)),
        (vec2(0.5, -0.5),   rgba(0.0, 1.0, 0.0, 1.0)),
        (vec2(0.0, 0.5),    rgba(0.0, 0.0, 1.0, 1.0)),
    ];

    let vertex_input = &[
        Attribute::new::<Vec2>("vert_position"),
        Attribute::new::<Vec4>("vert_color"),
    ];
    let shader = ctx.new_shader(ShaderDescription {
        vertex_input: vertex_input,
        fragment_input: &[ Attribute::new::<Vec4>("frag_color") ],
        uniforms: &[],
        vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
        fragment_shader: 
        r#" void main() {
            gl_FragColor = frag_color;
        }"#
    })?;

    let mut vb = ctx.new_vertex_buffer();
    let mut eb = ctx.new_element_buffer();
    let mut builder = VertexBuilder::new(vertex_input);
    for (pos, col) in vertices.iter() {
        builder.start()
            .add(pos)
            .add(col)
            .build();
    }
    vb.send_data(0, builder.data());
    eb.send_data(0, &[0, 1, 2]);

    let uniforms = Uniforms::new();

    ctx.set_program(&shader);
    
    while let Some(_) = events.next().await {
        ctx.clear(rgba(0.0, 0.0, 0.0, 1.0));
        ctx.draw(&vb, &eb, &uniforms);
    }

    Ok(())
}

fn main() {
    blinds::run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}
