use serde::Deserialize;
use serde_with::{json::JsonString, serde_as};

use crate::models::*;


#[derive(Deserialize, Debug)]
#[serde(tag = "event", content = "data", rename_all = "snake_case")]
pub enum WebSocketEvent {
    AddedToTeam(AddedToTeam),
    AuthenticationChallenge(AuthenticationChallenge),
    ChannelConverted(ChannelConverted),
    ChannelCreated(ChannelCreated),
    ChannelDeleted(ChannelDeleted),
    ChannelMemberUpdated(ChannelMemberUpdated),
    ChannelUpdated(ChannelUpdated),
    ChannelViewed(ChannelViewed),
    ConfigChanged(ConfigChanged),
    DeleteTeam(DeleteTeam),
    DirectAdded(DirectAdded),
    EmojiAdded(EmojiAdded),
    EphemeralMessage(EphemeralMessage),
    GroupAdded(GroupAdded),
    Hello(Hello),
    LeaveTeam(LeaveTeam),
    LicenseChanged(LicenseChanged),
    MemberroleUpdated(MemberroleUpdated),
    NewUser(NewUser),
    PluginDisabled(PluginDisabled),
    PluginEnabled(PluginEnabled),
    PluginStatusesChanged(PluginStatusesChanged),
    PostDeleted(PostDeleted),
    PostEdited(PostEdited),
    PostUnread(PostUnread),
    Posted(Posted),
    PreferenceChanged(PreferenceChanged),
    PreferencesChanged(PreferencesChanged),
    PreferencesDeleted(PreferencesDeleted),
    ReactionAdded(ReactionAdded),
    ReactionRemoved(ReactionRemoved),
    Response(Response),
    RoleUpdated(RoleUpdated),
    StatusChange(StatusChange),
    Typing(Typing),
    UpdateTeam(UpdateTeam),
    UserAdded(UserAdded),
    UserRemoved(UserRemoved),
    UserRoleUpdated(UserRoleUpdated),
    UserUpdated(UserUpdated),
    DialogOpened(DialogOpened),
    ThreadUpdated(ThreadUpdated),
    ThreadFollowChanged(ThreadFollowChanged),
    ThreadReadChanged(ThreadReadChanged),
}

#[derive(Deserialize, Debug)]
pub struct AddedToTeam { }

#[derive(Deserialize, Debug)]
pub struct AuthenticationChallenge { }

#[derive(Deserialize, Debug)]
pub struct ChannelConverted { }

#[derive(Deserialize, Debug)]
pub struct ChannelCreated { }

#[derive(Deserialize, Debug)]
pub struct ChannelDeleted { }

#[derive(Deserialize, Debug)]
pub struct ChannelMemberUpdated { }

#[derive(Deserialize, Debug)]
pub struct ChannelUpdated { }

#[derive(Deserialize, Debug)]
pub struct ChannelViewed { }

#[derive(Deserialize, Debug)]
pub struct ConfigChanged { }

#[derive(Deserialize, Debug)]
pub struct DeleteTeam { }

#[derive(Deserialize, Debug)]
pub struct DirectAdded { }

#[derive(Deserialize, Debug)]
pub struct EmojiAdded { }

#[derive(Deserialize, Debug)]
pub struct EphemeralMessage { }

#[derive(Deserialize, Debug)]
pub struct GroupAdded { }

#[derive(Deserialize, Debug)]
pub struct Hello  {
    pub server_version : String
}

#[derive(Deserialize, Debug)]
pub struct LeaveTeam { }

#[derive(Deserialize, Debug)]
pub struct LicenseChanged { }

#[derive(Deserialize, Debug)]
pub struct MemberroleUpdated { }

#[derive(Deserialize, Debug)]
pub struct NewUser { }

#[derive(Deserialize, Debug)]
pub struct PluginDisabled { }

#[derive(Deserialize, Debug)]
pub struct PluginEnabled { }

#[derive(Deserialize, Debug)]
pub struct PluginStatusesChanged { }

#[derive(Deserialize, Debug)]
pub struct PostDeleted { }

#[derive(Deserialize, Debug)]
pub struct PostEdited { }

#[derive(Deserialize, Debug)]
pub struct PostUnread { }

#[derive(Deserialize, Debug)]
pub struct Posted { }

#[derive(Deserialize, Debug)]
pub struct PreferenceChanged { }

#[derive(Deserialize, Debug)]
pub struct PreferencesChanged { }

#[derive(Deserialize, Debug)]
pub struct PreferencesDeleted { }

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct ReactionAdded {
    #[serde_as(as = "JsonString")]
    pub reaction : Reaction,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct ReactionRemoved {
    #[serde_as(as = "JsonString")]
    pub reaction : Reaction,
}

#[derive(Deserialize, Debug)]
pub struct Response { }

#[derive(Deserialize, Debug)]
pub struct RoleUpdated { }

#[derive(Deserialize, Debug)]
pub struct StatusChange { }

#[derive(Deserialize, Debug)]
pub struct Typing { }

#[derive(Deserialize, Debug)]
pub struct UpdateTeam { }

#[derive(Deserialize, Debug)]
pub struct UserAdded { }

#[derive(Deserialize, Debug)]
pub struct UserRemoved { }

#[derive(Deserialize, Debug)]
pub struct UserRoleUpdated { }

#[derive(Deserialize, Debug)]
pub struct UserUpdated { }

#[derive(Deserialize, Debug)]
pub struct DialogOpened { }

#[derive(Deserialize, Debug)]
pub struct ThreadUpdated { }

#[derive(Deserialize, Debug)]
pub struct ThreadFollowChanged { }

#[derive(Deserialize, Debug)]
pub struct ThreadReadChanged { }
