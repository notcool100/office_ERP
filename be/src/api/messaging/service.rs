use crate::{
    api::messaging::dto::{CreateChannelRequest, SendMessageRequest, MessageResponse},
    db::Db,
    models::messaging::{Channel, Message},
};
use anyhow::{Result, anyhow};
use uuid::Uuid;

pub async fn get_channel(db: &Db, channel_id: Uuid, user_id: Uuid) -> Result<Channel> {
    let channel = sqlx::query_as::<_, Channel>(
        r#"
        SELECT c.* 
        FROM channels c
        LEFT JOIN channel_members cm ON c.id = cm.channel_id
        WHERE c.id = $1 AND (c.is_private = false OR cm.user_id = $2)
        "#
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
        SELECT DISTINCT c.* 
        FROM channels c
        LEFT JOIN channel_members cm ON c.id = cm.channel_id
        WHERE c.is_private = false OR cm.user_id = $1
        "#
    )
        .bind(user_id)
        .fetch_all(db)
        .await?;
    Ok(channels)
}

pub async fn create_channel(db: &Db, req: CreateChannelRequest, creator_id: Uuid) -> Result<Channel> {
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
    sqlx::query(
        "INSERT INTO channel_members (channel_id, user_id, role) VALUES ($1, $2, 'admin')"
    )
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

pub async fn list_messages(db: &Db, channel_id: Uuid, limit: i64) -> Result<Vec<MessageResponse>> {
    let messages = sqlx::query_as::<_, MessageResponse>(
        r#"
        SELECT m.id, m.channel_id, m.sender_id, u.user_name as sender_name, m.content, m.created_at
        FROM messages m
        LEFT JOIN users u ON m.sender_id = u.id
        WHERE m.channel_id = $1
        ORDER BY m.created_at DESC
        LIMIT $2
        "#,
    )
    .bind(channel_id)
    .bind(limit)
    .fetch_all(db)
    .await?;

    Ok(messages)
}

pub async fn send_message(db: &Db, channel_id: Uuid, sender_id: Uuid, req: SendMessageRequest) -> Result<Message> {
    // Verify user is a member of the channel
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)"
    )
    .bind(channel_id)
    .bind(sender_id)
    .fetch_one(db)
    .await?;

    if !is_member {
        return Err(anyhow!("User is not a member of this channel"));
    }

    let message = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (id, channel_id, sender_id, content, parent_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(channel_id)
    .bind(sender_id)
    .bind(req.content)
    .bind(req.parent_id)
    .fetch_one(db)
    .await?;

    Ok(message)
}

pub async fn add_member(db: &Db, channel_id: Uuid, adder_id: Uuid, new_member_id: Uuid) -> Result<()> {
    // Verify adder is a member of the channel
    let is_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)"
    )
    .bind(channel_id)
    .bind(adder_id)
    .fetch_one(db)
    .await?;

    if !is_member {
        return Err(anyhow!("User is not a member of this channel"));
    }

    // Verify channel exists
    let channel = sqlx::query_as::<_, Channel>(
        "SELECT * FROM channels WHERE id = $1"
    )
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
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)"
    )
    .bind(channel_id)
    .bind(new_member_id)
    .fetch_one(db)
    .await?;

    if already_member {
        return Ok(()); // Or return an error depending on desired UX
    }

    sqlx::query(
        "INSERT INTO channel_members (channel_id, user_id, role) VALUES ($1, $2, 'member')"
    )
    .bind(channel.id)
    .bind(new_member_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn list_channel_members(db: &Db, channel_id: Uuid, user_id: Uuid) -> Result<Vec<crate::models::user::User>> {
    // Note: User model might need to be imported or we can map to a specific dto. We will return the models::user::User.
    
    // First verify user is a member/can access
    let can_access = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1 FROM channels c
            LEFT JOIN channel_members cm ON c.id = cm.channel_id
            WHERE c.id = $1 AND (c.is_private = false OR cm.user_id = $2)
        )
        "#
    )
    .bind(channel_id)
    .bind(user_id)
    .fetch_one(db)
    .await?;

    if !can_access {
        return Err(anyhow!("User does not have access to this channel"));
    }

    let members = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        SELECT u.* 
        FROM users u
        INNER JOIN channel_members cm ON u.id = cm.user_id
        WHERE cm.channel_id = $1
        "#
    )
    .bind(channel_id)
    .fetch_all(db)
    .await?;

    Ok(members)
}

pub async fn update_channel(db: &Db, channel_id: Uuid, user_id: Uuid, req: crate::api::messaging::dto::UpdateChannelRequest) -> Result<Channel> {
    // Verify user is an admin or creator (for now, simply being an admin in channel_members)
    // If there is no admin logic implemented widely, we can just check if they are a member if not a strict system. 
    // Let's check if they are "admin" role or just a member if roles aren't strictly updated everywhere.
    // The create_channel sets role = 'admin' for creator.
    let role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM channel_members WHERE channel_id = $1 AND user_id = $2"
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

    let channel = q.build_query_as::<Channel>()
        .fetch_one(db)
        .await?;

    Ok(channel)
}

pub async fn remove_member(db: &Db, channel_id: Uuid, caller_id: Uuid, target_user_id: Uuid) -> Result<()> {
    // Allowed if caller is target (leaving) or if caller is a member (simplistic permission)
    let caller_member = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM channel_members WHERE channel_id = $1 AND user_id = $2)"
    )
    .bind(channel_id)
    .bind(caller_id)
    .fetch_one(db)
    .await?;

    if !caller_member {
        return Err(anyhow!("Unauthorized"));
    }

    sqlx::query(
        "DELETE FROM channel_members WHERE channel_id = $1 AND user_id = $2"
    )
    .bind(channel_id)
    .bind(target_user_id)
    .execute(db)
    .await?;

    Ok(())
}
