mod build_settings;
mod diagnostic_messages;
mod endpoints;
mod engine;
mod external_variables;
mod performer;
mod program;
mod source_files;

pub use build_settings::*;
pub use diagnostic_messages::*;
pub use endpoints::*;
pub use engine::*;
pub use external_variables::*;
pub use performer::*;
pub use program::*;
pub use source_files::*;

pub struct Span<T> {
    value: T,
}

pub struct Type;
