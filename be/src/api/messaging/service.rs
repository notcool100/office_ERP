use crate::{
    api::messaging::dto::{
        AttachmentResponse, ChannelMediaItem, CreateChannelRequest, MessageResponse,
        ReactionSummary, SendMessageRequest,
    },
    db::Db,
    models::messaging::Channel,
};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub const MAX_ATTACHMENT_SIZE: usize = 25 * 1024 * 1024; // 25 MB per file
pub const MAX_ATTACHMENTS_PER_MESSAGE: usize = 10;

/// A file uploaded alongside a message, before it has been written to disk
/// or recorded in `message_attachments`.
pub struct NewAttachment {
    pub file_name: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

fn upload_dir() -> PathBuf {
    let base =
        std::env::var("MESSAGING_UPLOAD_DIR").unwrap_or_else(|_| "./uploads/messaging".into());
    PathBuf::from(base)
}

/// Grouped reaction counts for a batch of messages, from the viewer's
/// point of view. Mirrors `fetch_attachments`'s batch-then-distribute
/// shape so `list_messages` stays two queries regardless of page size.
async fn fetch_reactions_batch(
    db: &Db,
    message_ids: &[Uuid],
    viewer_id: Uuid,
) -> Result<HashMap<Uuid, Vec<ReactionSummary>>> {
    if message_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = sqlx::query_as::<_, (Uuid, String, i64, bool)>(
        r#"
        SELECT message_id, emoji, COUNT(*) as count,
               BOOL_OR(user_id = $2) as reacted_by_me
        FROM message_reactions
        WHERE message_id = ANY($1)
        GROUP BY message_id, emoji
        ORDER BY MIN(created_at)
        "#,
    )
    .bind(message_ids)
    .bind(viewer_id)
    .fetch_all(db)
    .await?;

    let mut by_message: HashMap<Uuid, Vec<ReactionSummary>> = HashMap::new();
    for (message_id, emoji, count, reacted_by_me) in rows {
        by_message.entry(message_id).or_default().push(ReactionSummary {
            emoji,
            count,
            reacted_by_me,
        });
    }
    Ok(by_message)
}

/// Adds the caller's reaction if they haven't used this emoji on this
/// message yet, removes it if they have — the same toggle gesture as
/// tapping an emoji a second time in any chat app — then returns the
/// message's updated reaction summary.
pub async fn toggle_reaction(
    db: &Db,
    message_id: Uuid,
    user_id: Uuid,
    emoji: &str,
) -> Result<Vec<ReactionSummary>> {
    if emoji.trim().is_empty() || emoji.chars().count() > 8 {
        return Err(anyhow!("Invalid reaction"));
    }

    let removed = sqlx::query(
        "DELETE FROM message_reactions WHERE message_id = $1 AND user_id = $2 AND emoji = $3",
    )
    .bind(message_id)
    .bind(user_id)
    .bind(emoji)
    .execute(db)
    .await?
    .rows_affected();

    if removed == 0 {
        sqlx::query(
            "INSERT INTO message_reactions (message_id, user_id, emoji) VALUES ($1, $2, $3) \
             ON CONFLICT DO NOTHING",
        )
        .bind(message_id)
        .bind(user_id)
        .bind(emoji)
        .execute(db)
        .await?;
    }

    let mut summary = fetch_reactions_batch(db, &[message_id], user_id).await?;
    Ok(summary.remove(&message_id).unwrap_or_default())
}

async fn fetch_attachments(db: &Db, message_id: Uuid) -> Result<Vec<AttachmentResponse>> {
    let attachments = sqlx::query_as::<_, AttachmentResponse>(
        r#"
        SELECT id, message_id, file_name, content_type, file_size,
            (content_type LIKE 'image/%') as is_image, created_at
        FROM message_attachments
        WHERE message_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(message_id)
    .fetch_all(db)
    .await?;
    Ok(attachments)
}

pub async fn get_channel(db: &Db, channel_id: Uuid, user_id: Uuid) -> Result<Channel> {
    let channel = sqlx::query_as::<_, Channel>(
        r#"
        SELECT c.*,
            (SELECT u.user_name FROM channel_members cm2
             JOIN users u ON u.id = cm2.user_id
             WHERE cm2.channel_id = c.id AND cm2.user_id != $2
             LIMIT 1) as dm_other_user_name
        FROM channels c
        LEFT JOIN channel_members cm ON c.id = cm.channel_id
        WHERE c.id = $1 AND (c.is_private = false OR cm.user_id = $2)
        "#,
    )
    .bind(channel_id)
    .bind(user_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Channel not found or unauthorized"))?;
    Ok(channel)
}

pub async fn list_channels(db: &Db, user_id: Uuid) -> Result<Vec<Channel>> {
    let channels = sqlx::query_as::<_, Channel>(
        r#"
        SELECT DISTINCT c.*,
            (SELECT u.user_name FROM channel_members cm2
             JOIN users u ON u.id = cm2.user_id
             WHERE cm2.channel_id = c.id AND cm2.user_id != $1
             LIMIT 1) as dm_other_user_name
        FROM channels c
        LEFT JOIN channel_members cm ON c.id = cm.channel_id
        WHERE c.is_private = false OR cm.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;
    Ok(channels)
}

pub async fn create_channel(
    db: &Db,
    req: CreateChannelRequest,
    creator_id: Uuid,
) -> Result<Channel> {
    let channel = sqlx::query_as::<_, Channel>(
        r#"
        INSERT INTO channels (id, name, description, is_private, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(req.name)
    .bind(req.description)
    .bind(req.is_private)
    .bind(creator_id)
    .fetch_one(db)
    .await?;

    // Auto-join the creator
    sqlx::query("INSERT INTO channel_members (channel_id, user_id, role) VALUES ($1, $2, 'admin')")
        .bind(channel.id)
        .bind(creator_id)
        .execute(db)
        .await?;

    // Add other members if any
    if let Some(members) = req.members {
        for member_id in members {
            if member_id != creator_id {
                sqlx::query(
                    "INSERT INTO channel_members (channel_id, user_id, role) VALUES ($1, $2, 'member')"
                )
                .bind(channel.id)
                .bind(member_id)
                .execute(db)
                .await?;
            }
        }
    }

    Ok(channel)
}

pub async fn list_messages(
    db: &Db,
    channel_id: Uuid,
    limit: i64,
    viewer_id: Uuid,
) -> Result<Vec<MessageResponse>> {
    let mut messages = sqlx::query_as::<_, MessageResponse>(
        r#"
        SELECT m.id, m.channel_id, m.sender_id,
            CONCAT(p.first_name, ' ', p.last_name) as sender_name,
            m.content, m.created_at
        FROM messages m
        LEFT JOIN users u ON m.sender_id = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        WHERE m.channel_id = $1
        ORDER BY m.created_at DESC
        LIMIT $2
        "#,
    )
    .bind(channel_id)
    .bind(limit)
    .fetch_all(db)
    .await?;

    let ids: Vec<Uuid> = messages.iter().map(|m| m.id).collect();
    if !ids.is_empty() {
        let attachments = sqlx::query_as::<_, AttachmentResponse>(
            r#"
            SELECT id, message_id, file_name, content_type, file_size,
                (content_type LIKE 'image/%') as is_image, created_at
            FROM message_attachments
            WHERE message_id = ANY($1)
            ORDER BY created_at
            "#,
        )
        .bind(&ids)
        .fetch_all(db)
        .await?;

        let mut by_message: HashMap<Uuid, Vec<AttachmentResponse>> = HashMap::new();
        for a in attachments {
            by_message.entry(a.message_id).or_default().push(a);
        }
        for m in &mut messages {
            if let Some(list) = by_message.remove(&m.id) {
                m.attachments = list;
            }
        }

        let mut reactions_by_message = fetch_reactions_batch(db, &ids, viewer_id).await?;
        for m in &mut messages {
            if let Some(list) = reactions_by_message.remove(&m.id) {
                m.reactions = list;
            }
        }
    }

    Ok(messages)
}

pub async fn send_message(
    db: &Db,
    channel_id: Uuid,
    sender_id: Uuid,
    req: SendMessageRequest,
    files: Vec<NewAttachment>,
) -> Result<MessageResponse> {
    if files.len() > MAX_ATTACHMENTS_PER_MESSAGE {
        return Err(anyhow!(
            "Too many attachments (max {})",
            MAX_ATTACHMENTS_PER_MESSAGE
        ));
    }
    for f in &files {
        if f.data.len() > MAX_ATTACHMENT_SIZE {
            return Err(anyhow!("Attachment too large (max 25 MB)"));
        }
    }
    if req.content.trim().is_empty() && files.is_empty() {
        return Err(anyhow!("Message must have content or an attachment"));
    }
    // Verify user is a member of the channel, auto-joining them if the
    // channel is public (list_channels/get_channel already expose public
    // channels to every user, so sending should not require a prior
    // explicit join).
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)",
    )
    .bind(channel_id)
    .bind(sender_id)
    .fetch_one(db)
    .await?;

    if !is_member {
        let is_private = sqlx::query_scalar::<_, bool>(
            "SELECT is_private FROM channels WHERE id = $1",
        )
        .bind(channel_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| anyhow!("Channel not found"))?;

        if is_private {
            return Err(anyhow!("User is not a member of this channel"));
        }

        sqlx::query(
            "INSERT INTO channel_members (channel_id, user_id, role) VALUES ($1, $2, 'member') \
             ON CONFLICT DO NOTHING",
        )
        .bind(channel_id)
        .bind(sender_id)
        .execute(db)
        .await?;
    }

    let mut message = sqlx::query_as::<_, MessageResponse>(
        r#"
        WITH inserted AS (
            INSERT INTO messages (id, channel_id, sender_id, content, parent_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, channel_id, sender_id, content, created_at
        )
        SELECT i.id, i.channel_id, i.sender_id,
            CONCAT(p.first_name, ' ', p.last_name) as sender_name,
            i.content, i.created_at
        FROM inserted i
        LEFT JOIN users u ON i.sender_id = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(channel_id)
    .bind(sender_id)
    .bind(req.content)
    .bind(req.parent_id)
    .fetch_one(db)
    .await?;

    if !files.is_empty() {
        let dir = upload_dir().join(channel_id.to_string());
        tokio::fs::create_dir_all(&dir).await?;

        for f in files {
            let ext = Path::new(&f.file_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("bin")
                .to_lowercase();
            let stored_name = format!("{}.{}", Uuid::new_v4(), ext);
            let abs_path = dir.join(&stored_name);
            tokio::fs::write(&abs_path, &f.data).await?;

            sqlx::query(
                r#"
                INSERT INTO message_attachments
                    (id, message_id, file_name, content_type, file_size, file_path)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(message.id)
            .bind(&f.file_name)
            .bind(&f.content_type)
            .bind(f.data.len() as i64)
            .bind(abs_path.to_string_lossy().into_owned())
            .execute(db)
            .await?;
        }

        message.attachments = fetch_attachments(db, message.id).await?;
    }

    Ok(message)
}

/// All shared attachments in a channel, newest first — powers the "shared
/// media" panel in the chat header (images/files a la Messenger).
pub async fn list_channel_media(
    db: &Db,
    channel_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<ChannelMediaItem>> {
    let can_access = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM channels c
            LEFT JOIN channel_members cm ON c.id = cm.channel_id
            WHERE c.id = $1 AND (c.is_private = false OR cm.user_id = $2)
        )
        "#,
    )
    .bind(channel_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

    if !can_access {
        return Err(anyhow!("User does not have access to this channel"));
    }

    let items = sqlx::query_as::<_, ChannelMediaItem>(
        r#"
        SELECT a.id, a.message_id, a.file_name, a.content_type, a.file_size,
            (a.content_type LIKE 'image/%') as is_image, a.created_at,
            m.sender_id, CONCAT(p.first_name, ' ', p.last_name) as sender_name
        FROM message_attachments a
        JOIN messages m ON m.id = a.message_id
        LEFT JOIN users u ON m.sender_id = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        WHERE m.channel_id = $1
        ORDER BY a.created_at DESC
        "#,
    )
    .bind(channel_id)
    .fetch_all(db)
    .await?;

    Ok(items)
}

#[derive(sqlx::FromRow)]
pub struct AttachmentFile {
    pub file_name: String,
    pub content_type: String,
    pub file_path: String,
}

pub async fn get_attachment_file(
    db: &Db,
    channel_id: Uuid,
    attachment_id: Uuid,
    user_id: Uuid,
) -> Result<AttachmentFile> {
    let can_access = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM channels c
            LEFT JOIN channel_members cm ON c.id = cm.channel_id
            WHERE c.id = $1 AND (c.is_private = false OR cm.user_id = $2)
        )
        "#,
    )
    .bind(channel_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

    if !can_access {
        return Err(anyhow!("User does not have access to this channel"));
    }

    let file = sqlx::query_as::<_, AttachmentFile>(
        r#"
        SELECT a.file_name, a.content_type, a.file_path
        FROM message_attachments a
        JOIN messages m ON m.id = a.message_id
        WHERE a.id = $1 AND m.channel_id = $2
        "#,
    )
    .bind(attachment_id)
    .bind(channel_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Attachment not found"))?;

    Ok(file)
}

pub async fn add_member(
    db: &Db,
    channel_id: Uuid,
    adder_id: Uuid,
    new_member_id: Uuid,
) -> Result<()> {
    // Verify adder is a member of the channel
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)",
    )
    .bind(channel_id)
    .bind(adder_id)
    .fetch_one(db)
    .await?;

    if !is_member {
        return Err(anyhow!("User is not a member of this channel"));
    }

    // Verify channel exists
    let channel = sqlx::query_as::<_, Channel>("SELECT * FROM channels WHERE id = $1")
        .bind(channel_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| anyhow!("Channel not found"))?;

    // Only allow adding members to private channels if that's the intention, but here it's fine
    // just to add to any channel.

    // Insert new member (ignore if already exists via ON CONFLICT do nothing)
    // We don't have a unique constraint specified in the plan, but generally it's channel_id, user_id
    // For now we'll do a simple insert. If it fails, they might already be a member.
    let already_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)",
    )
    .bind(channel_id)
    .bind(new_member_id)
    .fetch_one(db)
    .await?;

    if already_member {
        return Ok(()); // Or return an error depending on desired UX
    }

    sqlx::query(
        "INSERT INTO channel_members (channel_id, user_id, role) VALUES ($1, $2, 'member')",
    )
    .bind(channel.id)
    .bind(new_member_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn list_channel_members(
    db: &Db,
    channel_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<crate::api::messaging::dto::ChannelMemberResponse>> {
    // First verify user is a member/can access
    let can_access = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM channels c
            LEFT JOIN channel_members cm ON c.id = cm.channel_id
            WHERE c.id = $1 AND (c.is_private = false OR cm.user_id = $2)
        )
        "#,
    )
    .bind(channel_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

    if !can_access {
        return Err(anyhow!("User does not have access to this channel"));
    }

    let members = sqlx::query_as::<_, crate::api::messaging::dto::ChannelMemberResponse>(
        r#"
        SELECT u.id, CONCAT(p.first_name, ' ', p.last_name) as display_name, u.email
        FROM users u
        LEFT JOIN persons p ON u.person_id = p.id
        INNER JOIN channel_members cm ON u.id = cm.user_id
        WHERE cm.channel_id = $1
        "#,
    )
    .bind(channel_id)
    .fetch_all(db)
    .await?;

    Ok(members)
}

pub async fn get_channel_member_ids(db: &Db, channel_id: Uuid) -> Result<Vec<Uuid>> {
    let ids =
        sqlx::query_scalar::<_, Uuid>("SELECT user_id FROM channel_members WHERE channel_id = $1")
            .bind(channel_id)
            .fetch_all(db)
            .await?;
    Ok(ids)
}

pub async fn update_channel(
    db: &Db,
    channel_id: Uuid,
    user_id: Uuid,
    req: crate::api::messaging::dto::UpdateChannelRequest,
) -> Result<Channel> {
    // Verify user is an admin or creator (for now, simply being an admin in channel_members)
    // If there is no admin logic implemented widely, we can just check if they are a member if not a strict system.
    // Let's check if they are "admin" role or just a member if roles aren't strictly updated everywhere.
    // The create_channel sets role = 'admin' for creator.
    let role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM channel_members WHERE channel_id = $1 AND user_id = $2",
    )
    .bind(channel_id)
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    if role.is_none() {
        return Err(anyhow!("User is not a member of this channel"));
    }

    // For simplicity, skip strict "admin" check for renaming unless we specifically enforce it.
    // Given the prompt didn't specify strict permissions, we'll allow any member to edit (like slack default for some channels)
    // or we check if role == Some("admin"). Let's check role == "admin" for safety.
    // Wait, DMs don't have admins explicitly in all cases, maybe just creator.
    // Let's just allow channel members to update the channel (name/description) for simplicity now.

    let mut q = sqlx::QueryBuilder::new("UPDATE channels SET ");
    let mut separated = q.separated(", ");

    if let Some(name) = &req.name {
        separated.push("name = ");
        separated.push_bind_unseparated(name);
    }
    if let Some(desc) = &req.description {
        separated.push("description = ");
        separated.push_bind_unseparated(desc);
    }

    if req.name.is_none() && req.description.is_none() {
        // Nothing to update
        let channel = get_channel(db, channel_id, user_id).await?;
        return Ok(channel);
    }

    q.push(" WHERE id = ");
    q.push_bind(channel_id);
    q.push(" RETURNING *");

    let channel = q.build_query_as::<Channel>().fetch_one(db).await?;

    Ok(channel)
}

pub async fn remove_member(
    db: &Db,
    channel_id: Uuid,
    caller_id: Uuid,
    target_user_id: Uuid,
) -> Result<()> {
    // Allowed if caller is target (leaving) or if caller is a member (simplistic permission)
    let caller_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)",
    )
    .bind(channel_id)
    .bind(caller_id)
    .fetch_one(db)
    .await?;

    if !caller_member {
        return Err(anyhow!("Unauthorized"));
    }

    sqlx::query("DELETE FROM channel_members WHERE channel_id = $1 AND user_id = $2")
        .bind(channel_id)
        .bind(target_user_id)
        .execute(db)
        .await?;

    Ok(())
}
