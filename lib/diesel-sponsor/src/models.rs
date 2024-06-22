// Generated by diesel_ext, edited
#![allow(unused)]

use crate::schema::{
    archived_sponsor_times,
    category_votes,
    config,
    lock_categories,
    ratings,
    shadow_banned_ips,
    shadow_banned_users,
    sponsor_times,
    thumbnail_timestamps,
    thumbnail_votes,
    thumbnails,
    title_votes,
    titles,
    unlisted_videos,
    user_features,
    user_names,
    video_info,
    vip_users,
    warnings
};
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = archived_sponsor_times)]
pub struct ArchivedSponsorTime {
    pub video_id: String,
    pub start_time: f32,
    pub end_time: f32,
    pub votes: i32,
    pub locked: i32,
    pub incorrect_votes: i32,
    pub uuid: String,
    pub user_id: String,
    pub time_submitted: i32,
    pub views: i32,
    pub category: String,
    pub action_type: String,
    pub service: String,
    pub video_duration: i32,
    pub hidden: i32,
    pub reputation: i32,
    pub shadow_hidden: i32,
    pub hashed_video_id: String,
    pub user_agent: String,
    pub description: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = category_votes)]
pub struct CategoryVote {
    pub uuid: String,
    pub category: String,
    pub votes: i32,
    pub id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(key))]
#[diesel(table_name = config)]
pub struct Config {
    pub key: String,
    pub value: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = lock_categories)]
pub struct LockCategory {
    pub video_id: String,
    pub user_id: String,
    pub action_type: String,
    pub category: String,
    pub hashed_video_id: String,
    pub reason: String,
    pub service: String,
    pub id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub video_id: String,
    pub service: String,
    pub type_: i32,
    pub count: i32,
    pub hashed_video_id: String,
    pub id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(hashed_id))]
#[diesel(table_name = shadow_banned_ips)]
pub struct ShadowBannedIp {
    pub hashed_id: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = shadow_banned_users)]
pub struct ShadowBannedUser {
    pub user_id: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = sponsor_times)]
pub struct SponsorTime {
    pub video_id: String,
    pub start_time: f32,
    pub end_time: f32,
    pub votes: i32,
    pub locked: i32,
    pub incorrect_votes: i32,
    pub uuid: String,
    pub user_id: String,
    pub time_submitted: i32,
    pub views: i32,
    pub category: String,
    pub action_type: String,
    pub service: String,
    pub video_duration: i32,
    pub hidden: i32,
    pub reputation: i32,
    pub shadow_hidden: i32,
    pub hashed_video_id: String,
    pub user_agent: String,
    pub description: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = thumbnail_timestamps)]
pub struct ThumbnailTimestamp {
    pub uuid: String,
    pub timestamp: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = thumbnail_votes)]
pub struct ThumbnailVote {
    pub uuid: String,
    pub votes: Option<i32>,
    pub locked: Option<i32>,
    pub shadow_hidden: Option<i32>,
    pub downvotes: Option<i32>,
    pub removed: Option<i32>,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = thumbnails)]
pub struct Thumbnail {
    pub original: Option<i32>,
    pub user_id: String,
    pub service: String,
    pub hashed_video_id: String,
    pub time_submitted: i32,
    pub uuid: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = title_votes)]
pub struct TitleVote {
    pub uuid: String,
    pub votes: i32,
    pub locked: i32,
    pub shadow_hidden: i32,
    pub verification: Option<i32>,
    pub downvotes: i32,
    pub removed: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = titles)]
pub struct Title {
    pub video_id: String,
    pub title: String,
    pub original: Option<i32>,
    pub user_id: String,
    pub service: String,
    pub hashed_video_id: String,
    pub time_submitted: i32,
    pub uuid: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(table_name = unlisted_videos)]
pub struct UnlistedVideo {
    pub video_id: String,
    pub year: i32,
    pub views: i32,
    pub channel_id: String,
    pub time_submitted: i32,
    pub service: String,
    pub id: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(user_id, feature))]
#[diesel(table_name = user_features)]
pub struct UserFeature {
    pub user_id: String,
    pub feature: i32,
    pub issuer_user_id: String,
    pub time_submitted: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = user_names)]
pub struct UserName {
    pub user_id: String,
    pub user_name: String,
    pub locked: i32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(video_id))]
#[diesel(table_name = video_info)]
pub struct VideoInfo {
    pub video_id: String,
    pub channel_id: String,
    pub title: String,
    pub published: f32,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = vip_users)]
pub struct VipUser {
    pub user_id: String,
}

#[derive(Queryable, Debug, Identifiable)]
#[diesel(primary_key(user_id, issue_time))]
#[diesel(table_name = warnings)]
pub struct Warning {
    pub user_id: String,
    pub issue_time: i32,
    pub issuer_user_id: String,
    pub enabled: i32,
    pub reason: String,
    pub type_: Option<i32>,
}

