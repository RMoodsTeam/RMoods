//! Routes for generating reports


mod sentiment;
mod language;
mod sarcasm;
mod keywords;
mod spam;
mod politics;
mod hate_speech;
mod clickbait;
mod troll;

// Re-exporting the functions to the top level
// avoid having to use the module name to call the functions
pub use sentiment::sentiment;
pub use language::language;
pub use sarcasm::sarcasm;
pub use keywords::keywords;
pub use spam::spam;
pub use politics::politics;
pub use hate_speech::hate_speech;
pub use clickbait::clickbait;
pub use troll::troll;
