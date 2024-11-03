use std::fmt::Debug;

pub trait RMoodsReport {}

/// Temporary solution until we add proper report result structs.
/// TODO: Add proper report result structs.
impl<T> RMoodsReport for T where T: Debug {}

/// Type trickery to allow us to print the trait object when sent over `mpsc`.
/// https://users.rust-lang.org/t/how-to-implement-debug-for-dyn-trait/39950
impl Debug for dyn RMoodsReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RMoodsReport")
    }
}

/// Type trickery to allow us to print the trait object when sent over `mpsc`.
impl Debug for dyn RMoodsReport + Send {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RMoodsReport")
    }
}
