//! Routes for generating reports

mod clickbait;
mod hate_speech;
mod keywords;
mod language;
mod politics;
mod sarcasm;
mod sentiment;
mod spam;
mod troll;

// Re-exporting the functions to the top level
// avoid having to use the module name to call the functions
pub use clickbait::clickbait;
pub use hate_speech::hate_speech;
pub use keywords::keywords;
pub use language::language;
pub use politics::politics;
pub use sarcasm::sarcasm;
pub use sentiment::sentiment;
pub use spam::spam;
pub use troll::troll;
