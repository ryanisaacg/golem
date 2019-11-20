// TODO: should there be a vertex-safe way of inserting stuff into the VBO?
// TODO: validate vec and matrix dimensions
#[derive(Debug)]
pub struct GolemError();
mod context;
pub use self::context::Context;
pub mod input;
pub mod program;
