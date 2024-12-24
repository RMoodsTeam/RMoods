use chrono::{DateTime, SubsecRound, Utc};

/// Returns the UTC DateTime timestamp rounded to 6 subsecond digits.
///
/// This is due to how Postgres handles timestamps. It returns them with 6 digits,
/// as opposed to [chrono::Utc]'s 9 digits.
pub fn get_utc_timestamp() -> DateTime<Utc> {
    Utc::now().round_subsecs(6)
}
