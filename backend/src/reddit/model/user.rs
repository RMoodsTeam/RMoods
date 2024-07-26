use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

/// Contains some properties of a Reddit user. For some real-world examples see
/// [this u/spez profile request.](https://www.reddit.com/user/spez/about.json)
#[serde_as]
#[derive(Getters, Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// Is the user a Reddit employee?
    is_employee: bool,
    /// Karma received by getting awards
    awardee_karma: i64,
    /// User ID, eg. 1w72
    id: String,
    /// Is the user a verified?
    verified: bool,
    /// Karma given to others by giving awards
    awarder_karma: i64,
    /// Better described as `post karma`, received for others upvoting your posts
    link_karma: i64,
    /// Karma received for others upvoting your comments
    comment_karma: i64,
    /// Total karma, sum of all karma types. Maybe remove this field and calculate it?
    total_karma: i64,
    /// Username without `u/`, eg. spez
    name: String,
    /// Link to the user icon
    #[serde_as(as = "NoneAsEmptyString")]
    icon_img: Option<String>,
    /// Link to the user snoovatar, whatever that is
    #[serde_as(as = "NoneAsEmptyString")]
    snoovatar_img: Option<String>,
}
