#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate oauth_client;

use oauth_client::Token;
use oauth_client::ParamList;


#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlogInfo {
    admin: Option<bool>,
    ask: bool,
    ask_anon: bool,
    ask_page_title: String,
    can_send_fan_mail: bool,
    can_submit: Option<bool>,
    can_subscribe: bool,
    description: String,
    drafts: Option<u64>,
    facebook: Option<String>,
    facebook_opengraph_enabled: Option<String>,
    followed: bool,
    followers: Option<u64>,
    is_adult: bool,
    is_blocked_from_primary: bool,
    is_nsfw: bool,
    likes: Option<u64>,
    messages: Option<u64>,
    name: String,
    posts: u64,
    primary: Option<bool>,
    queue: Option<u64>,
    reply_conditions: String,
    share_likes: bool,
    submission_page_title: Option<String>,
    submission_terms: Option<SubmissionTerms>,
    subscribed: bool,
    title: String,
    total_posts: u64,
    tweet: Option<String>,
    twitter_enabled: Option<bool>,
    twitter_send: Option<bool>,
    #[serde(rename = "type")]
    blog_type: Option<String>,
    updated: u64,
    url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmissionTerms {
    accepted_types: Vec<String>,
    guidelines: String,
    tags: Vec<String>,
    title: String,
}

//TODO This is just a workaround until serde #119 is done
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum SerializablePost {
    Text(SText),
    Quote(SQuote),
    Link(SLink),
    Answer(SAnswer),
    Video(SVideo),
    Audio(SAudio),
    Photo(SPhoto),
    Chat(SChat),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SText {
    body: String,
    title: Option<String>,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SQuote {
    source: String,
    text: String,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SLink {
    description: String,
    excerpt: Option<String>,
    link_author: Option<String>,
    link_image: Option<String>,
    link_image_dimensions: Option<ImageDimensions>,
    photos: Option<Vec<PhotoInfo>>,
    publisher: String,
    title: Option<String>,
    url: String,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SAnswer {
    answer: String,
    asking_name: String,
    asking_url: Option<String>,
    question: String,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SVideo {
    caption: String,
    duration: Option<u64>,
    html5_capable: bool,
    permalink_url: Option<String>,
    player: Vec<VideoEmbed>,
    thumbnail_height: u32,
    thumbnail_url: String,
    thumbnail_width: u32,
    video: Option<Video>,
    video_type: String,
    video_url: Option<String>,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SAudio {
    album: Option<String>,
    album_art: Option<String>,
    artist: Option<String>,
    audio_source_url: String,
    audio_type: String,
    audio_url: String,
    caption: String,
    embed: String,
    is_external: Option<bool>,
    player: String,
    plays: u64,
    provider_uri: Option<String>,
    track: Option<String>,
    track_name: Option<String>,
    year: Option<u64>,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SPhoto {
    caption: String,
    image_permalink: Option<String>,
    link_url: Option<String>,
    photos: Vec<PhotoInfo>,
    photoset_layout: Option<String>,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SChat {
    body: String,
    dialogue: Vec<Dialogue>,
    title: Option<String>,
    //everything has these
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug)]
pub struct Post {
    post_type: PostType,
    blog_name: String,
    can_like: bool,
    can_reblog: bool,
    can_reply: bool,
    can_send_in_message: bool,
    date: String,
    display_avatar: bool,
    featured_in_tag: Option<Vec<String>>,
    featured_timestamp: Option<u64>,
    followed: bool,
    format: String,
    id: u64,
    is_anonymous: Option<bool>,
    is_submission: Option<bool>,
    liked: bool,
    liked_timestamp: Option<u64>,
    note_count: u64,
    post_author: Option<String>,
    post_url: String,
    queued_state: Option<String>,
    reblog: Option<Reblog>,
    reblog_key: String,
    recommended_color: Option<String>,
    recommended_source: Option<String>,
    scheduled_publish_time: Option<String>,
    short_url: String,
    slug: String,
    source_title: Option<String>,
    source_url: Option<String>,
    state: PostState,
    summary: String,
    tags: Vec<String>,
    timestamp: u64,
    trail: Option<Vec<Trail>>,
    //Only when param reblog_info is true
    reblogged_from_can_message: Option<bool>,
    reblogged_from_following: Option<bool>,
    reblogged_from_id: Option<u64>,
    reblogged_from_name: Option<String>,
    reblogged_from_title: Option<String>,
    reblogged_from_url: Option<String>,
    reblogged_from_uuid: Option<String>,
    reblogged_root_can_message: Option<bool>,
    reblogged_root_following: Option<bool>,
    reblogged_root_id: Option<String>,
    reblogged_root_name: Option<String>,
    reblogged_root_title: Option<String>,
    reblogged_root_url: Option<String>,
    reblogged_root_uuid: Option<String>,
    //Only when param notes_info is true
    notes: Option<Vec<Note>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostType {
    Text {
        body: String,
        title: Option<String>,
    },
    Quote {
        source: String,
        text: String,
    },
    Link {
        description: String,
        excerpt: Option<String>,
        link_author: Option<String>,
        link_image: Option<String>,
        link_image_dimensions: Option<ImageDimensions>,
        photos: Option<Vec<PhotoInfo>>,
        publisher: String,
        title: Option<String>,
        url: String,
    },
    Answer {
        answer: String,
        asking_name: String,
        asking_url: Option<String>,
        question: String,
    },
    Video {
        caption: String,
        duration: Option<u64>,
        html5_capable: bool,
        permalink_url: Option<String>,
        player: Vec<VideoEmbed>,
        thumbnail_height: u32,
        thumbnail_url: String,
        thumbnail_width: u32,
        video: Option<Video>,
        video_type: String,
        video_url: Option<String>,
    },
    Audio {
        album: Option<String>,
        album_art: Option<String>,
        artist: Option<String>,
        audio_source_url: String,
        audio_type: String,
        audio_url: String,
        caption: String,
        embed: String,
        is_external: Option<bool>,
        player: String,
        plays: u64,
        provider_uri: Option<String>,
        track: Option<String>,
        track_name: Option<String>,
        year: Option<u64>,
    },
    Photo {
        caption: String,
        image_permalink: Option<String>,
        link_url: Option<String>,
        photos: Vec<PhotoInfo>,
        photoset_layout: Option<String>,
    },
    Chat {
        body: String,
        dialogue: Vec<Dialogue>,
        title: Option<String>,
    },
}

impl SerializablePost {
    pub fn into_post(self) -> Post {
        match self {
            SerializablePost::Text(spost) => {
                let post_type = PostType::Text { body: spost.body, title: spost.title };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
            SerializablePost::Quote(spost) => {
                let post_type = PostType::Quote { source: spost.source, text: spost.text };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
            SerializablePost::Link(spost) => {
                let post_type = PostType::Link {
                    description: spost.description,
                    excerpt: spost.excerpt,
                    link_author: spost.link_author,
                    link_image: spost.link_image,
                    link_image_dimensions: spost.link_image_dimensions,
                    photos: spost.photos,
                    publisher: spost.publisher,
                    title: spost.title,
                    url: spost.url,
                };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
            SerializablePost::Answer(spost) => {
                let post_type = PostType::Answer {
                    answer: spost.answer,
                    asking_name: spost.asking_name,
                    asking_url: spost.asking_url,
                    question: spost.question,
                };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
            SerializablePost::Video(spost) => {
                let post_type = PostType::Video {
                    caption: spost.caption,
                    duration: spost.duration,
                    html5_capable: spost.html5_capable,
                    permalink_url: spost.permalink_url,
                    player: spost.player,
                    thumbnail_height: spost.thumbnail_height,
                    thumbnail_url: spost.thumbnail_url,
                    thumbnail_width: spost.thumbnail_width,
                    video: spost.video,
                    video_type: spost.video_type,
                    video_url: spost.video_url,
                };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
            SerializablePost::Audio(spost) => {
                let post_type = PostType::Audio {
                    album: spost.album,
                    album_art: spost.album_art,
                    artist: spost.artist,
                    audio_source_url: spost.audio_source_url,
                    audio_type: spost.audio_type,
                    audio_url: spost.audio_url,
                    caption: spost.caption,
                    embed: spost.embed,
                    is_external: spost.is_external,
                    player: spost.player,
                    plays: spost.plays,
                    provider_uri: spost.provider_uri,
                    track: spost.track,
                    track_name: spost.track_name,
                    year: spost.year,
                };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
            SerializablePost::Photo(spost) => {
                let post_type = PostType::Photo {
                    caption: spost.caption,
                    image_permalink: spost.image_permalink,
                    link_url: spost.link_url,
                    photos: spost.photos,
                    photoset_layout: spost.photoset_layout,
                };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
            SerializablePost::Chat(spost) => {
                let post_type = PostType::Chat {
                    body: spost.body,
                    dialogue: spost.dialogue,
                    title: spost.title,
                };
                Post {
                    post_type: post_type,
                    blog_name: spost.blog_name,
                    can_like: spost.can_like,
                    can_reblog: spost.can_reblog,
                    can_reply: spost.can_reply,
                    can_send_in_message: spost.can_send_in_message,
                    date: spost.date,
                    display_avatar: spost.display_avatar,
                    featured_in_tag: spost.featured_in_tag,
                    featured_timestamp: spost.featured_timestamp,
                    followed: spost.followed,
                    format: spost.format,
                    id: spost.id,
                    is_anonymous: spost.is_anonymous,
                    is_submission: spost.is_submission,
                    liked: spost.liked,
                    liked_timestamp: spost.liked_timestamp,
                    note_count: spost.note_count,
                    post_author: spost.post_author,
                    post_url: spost.post_url,
                    queued_state: spost.queued_state,
                    reblog: spost.reblog,
                    reblog_key: spost.reblog_key,
                    recommended_color: spost.recommended_color,
                    recommended_source: spost.recommended_source,
                    scheduled_publish_time: spost.scheduled_publish_time,
                    short_url: spost.short_url,
                    slug: spost.slug,
                    source_title: spost.source_title,
                    source_url: spost.source_url,
                    state: spost.state,
                    summary: spost.summary,
                    tags: spost.tags,
                    timestamp: spost.timestamp,
                    trail: spost.trail,
                    reblogged_from_can_message: spost.reblogged_from_can_message,
                    reblogged_from_following: spost.reblogged_from_following,
                    reblogged_from_id: spost.reblogged_from_id,
                    reblogged_from_name: spost.reblogged_from_name,
                    reblogged_from_title: spost.reblogged_from_title,
                    reblogged_from_url: spost.reblogged_from_url,
                    reblogged_from_uuid: spost.reblogged_from_uuid,
                    reblogged_root_can_message: spost.reblogged_root_can_message,
                    reblogged_root_following: spost.reblogged_root_following,
                    reblogged_root_id: spost.reblogged_root_id,
                    reblogged_root_name: spost.reblogged_root_name,
                    reblogged_root_title: spost.reblogged_root_title,
                    reblogged_root_url: spost.reblogged_root_url,
                    reblogged_root_uuid: spost.reblogged_root_uuid,
                    notes: spost.notes,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostState {
    Published,
    Queued,
    Draft,
    Private,
    Submission,
}

#[derive(Debug)]
pub enum Error {
    OAuth(oauth_client::Error),
    Json(serde_json::Error),
    Meta(Meta),
    Missing(serde_json::Map<String, serde_json::Value>),
}

impl From<oauth_client::Error> for Error {
    fn from(error: oauth_client::Error) -> Self {
        Error::OAuth(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Meta {
    status: u32,
    msg: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    blog: Option<BlogInfo>,
    posts: Option<Vec<SerializablePost>>,
    liked_count: Option<u64>,
    liked_posts: Option<Vec<SerializablePost>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Reblog {
    comment: String,
    tree_html: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Trail {
    content: Option<String>,
    content_raw: String,
    is_current_item: Option<bool>,
    is_root_item: Option<bool>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Note {
    Reblog {
        added_text: Option<String>,
        post_id: String,
        reblog_parent_blog_name: String,
        avatar_shape: String,
        blog_name: String,
        blog_url: String,
        blog_uuid: String,
        followed: bool,
        timestamp: u64,
    },
    Like {
        avatar_shape: String,
        blog_name: String,
        blog_url: String,
        blog_uuid: String,
        followed: bool,
        timestamp: u64,
    },
    Posted {
        avatar_shape: String,
        blog_name: String,
        blog_url: String,
        blog_uuid: String,
        followed: bool,
        timestamp: u64,
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ImageDimensions {
    height: u32,
    width: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Video {
    Youtube {
        height: u32,
        width: u32,
        video_id: String,
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhotoInfo {
    alt_sizes: Vec<Photo>,
    original_size: Photo,
    exif: Option<Exif>,
    caption: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Photo {
    url: String,
    height: u32,
    width: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Exif {
    aperture: Option<String>,
    camera: Option<String>,
    exposure: Option<String>,
    focal_length: Option<String>,
    #[serde(rename = "ISO")]
    iso: Option<u64>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VideoEmbed {
    embed_code: String,
    width: u32,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dialogue {
    label: String,
    name: String,
    phrase: String,
}

#[derive(Debug)]
pub struct Tumblr<'a> {
    consumer: Token<'a>,
    token: Option<Token<'a>>,
}

impl<'a> Tumblr<'a> {
    pub fn new(consumer: &'a str, consumer_secret: &'a str) -> Tumblr<'a> {
        Tumblr {
            consumer: Token::new(consumer, consumer_secret),
            token: None,
        }
    }
    pub fn set_token(&mut self, token: &'a str, token_secret: &'a str) {
        self.token = Some(Token::new(token, token_secret));
    }

    pub fn get<T>(&self, endpoint: &str, params: Option<&ParamList>) -> Result<T, Error>
        where T: serde::de::DeserializeOwned
    {
        let url = format!("https://api.tumblr.com/v2{}", endpoint);
        let reply = if let Some(ref token) = self.token {
            oauth_client::get(&url, &self.consumer, Some(&token), params)?
        } else {
            oauth_client::get(&url, &self.consumer, None, params)?
        };
        let mut map: serde_json::Map<String, serde_json::Value> = serde_json::from_slice(&reply)?;
        if let Some(response) = map.remove("response") {
            Ok(serde_json::from_value(response)?)
        } else if let Some(meta) = map.remove("meta") {
            let meta = serde_json::from_value(meta)?;
            Err(Error::Meta(meta))
        } else {
            Err(Error::Missing(map))
        }
    }

    pub fn post(&self, endpoint: &str, params: Option<&ParamList>) -> Result<(), Error> {
        let url = format!("https://api.tumblr.com/v2{}", endpoint);
        let reply = if let Some(ref token) = self.token {
            oauth_client::post(&url, &self.consumer, Some(&token), params)?
        } else {
            oauth_client::post(&url, &self.consumer, None, params)?
        };
        println!("{}", String::from_utf8(reply).unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Tumblr, Response};
    use oauth_client::ParamList;
    #[test]
    fn requests() {
        let consumer_key = include_str!("../keys/consumer_key");
        let consumer_secret = include_str!("../keys/consumer_secret");
        let token = include_str!("../keys/token");
        let token_secret = include_str!("../keys/token_secret");
        let mut tumblr = Tumblr::new(consumer_key, consumer_secret);
        tumblr.set_token(token, token_secret);
        let mut params = ParamList::new();
        params.insert("type".into(), "audio".into());
        let posts = tumblr.get::<Response>("/user/dashboard", Some(&params)).unwrap();
        println!("{:#?}", posts);
    }

#[test]
fn posting() {
    let consumer_key = include_str!("../keys/consumer_key");
    let consumer_secret = include_str!("../keys/consumer_secret");
    let token = include_str!("../keys/token");
    let token_secret = include_str!("../keys/token_secret");
    let mut tumblr = Tumblr::new(consumer_key, consumer_secret);
    tumblr.set_token(token, token_secret);
    let posts: Response = tumblr.get("/blog/mroaf/posts/submission", None).unwrap();
    for post in posts.posts.unwrap() {
        let post = post.into_post();
        println!("{:#?}", post);
        let mut params = ParamList::new();
        params.insert("id".into(), post.id.to_string().into());
        params.insert("state".into(), "queue".into());
        params.insert("answer".into(), "Grobbler".into());
        tumblr.post("/blog/mroaf/post/edit", Some(&params)).unwrap();
    }
}
}
