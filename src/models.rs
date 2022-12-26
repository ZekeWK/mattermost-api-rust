use serde::{Serialize, Deserialize};

pub type UserId = String;
pub type ChannelId = String;
pub type PostId = String;
pub type TeamId = String;
pub type FileId = String;
pub type JSON = String;
pub type Emoji = String;
pub type Roles = String;


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Reaction {
    pub user_id : UserId,
    pub post_id : PostId,
    pub emoji_name : Emoji,
    pub create_at : u64,
    pub update_at : u64,
    pub delete_at : u64,
    pub remote_id : String,
    pub channel_id : ChannelId,
}
