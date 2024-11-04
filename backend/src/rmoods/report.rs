use std::fmt::Debug;

pub trait RMoodsReport: Debug + Send {}

/// Temporary solution until we add proper report result structs.
/// TODO: Add proper report result structs.
impl<T> RMoodsReport for T where T: Debug + Send {}
