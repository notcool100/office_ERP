use crate::{
    api::meetings::dto::{CreateMeetingRequest, MeetingResponse, ParticipantResponse},
    api::messaging::dto::{MessageResponse, SendMessageRequest},
    db::Db,
};
use anyhow::{Result, anyhow};
use uuid::Uuid;

pub async fn create_meeting(
    db: &Db,
    req: CreateMeetingRequest,
    host_id: Uuid,
) -> Result<(MeetingResponse, Option<MessageResponse>)> {
    let meeting_id = Uuid::new_v4();
    let title = req.title.clone().unwrap_or_else(|| "Meeting".to_string());

    let meeting = sqlx::query_as::<_, MeetingResponse>(
        r#"
        WITH inserted AS (
            INSERT INTO meetings (id, title, channel_id, host_id, status)
            VALUES ($1, $2, $3, $4, 'active')
            RETURNING id, title, channel_id, host_id, status, created_at, ended_at
        )
        SELECT i.id, i.title, i.channel_id, i.host_id,
            CONCAT(p.first_name, ' ', p.last_name) as host_name,
            i.status, i.created_at, i.ended_at
        FROM inserted i
        LEFT JOIN users u ON i.host_id = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        "#,
    )
    .bind(meeting_id)
    .bind(&title)
    .bind(req.channel_id)
    .bind(host_id)
    .fetch_one(db)
    .await?;

    sqlx::query(
        "INSERT INTO meeting_participants (id, meeting_id, user_id, role) VALUES ($1, $2, $3, 'host')",
    )
    .bind(Uuid::new_v4())
    .bind(meeting_id)
    .bind(host_id)
    .execute(db)
    .await?;

    let message = if let Some(channel_id) = req.channel_id {
        let content = format!(
            "\u{1F4F9} started a meeting \u{2014} tap to join: /meetings/{}",
            meeting_id
        );
        let msg = crate::api::messaging::service::send_message(
            db,
            channel_id,
            host_id,
            SendMessageRequest {
                content,
                parent_id: None,
            },
        )
        .await?;
        Some(msg)
    } else {
        None
    };

    Ok((meeting, message))
}

pub async fn get_meeting(db: &Db, meeting_id: Uuid) -> Result<MeetingResponse> {
    let meeting = sqlx::query_as::<_, MeetingResponse>(
        r#"
        SELECT m.id, m.title, m.channel_id, m.host_id,
            CONCAT(p.first_name, ' ', p.last_name) as host_name,
            m.status, m.created_at, m.ended_at
        FROM meetings m
        LEFT JOIN users u ON m.host_id = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        WHERE m.id = $1
        "#,
    )
    .bind(meeting_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Meeting not found"))?;

    Ok(meeting)
}

pub async fn join_meeting(
    db: &Db,
    meeting_id: Uuid,
    user_id: Uuid,
) -> Result<Vec<ParticipantResponse>> {
    let status = sqlx::query_scalar::<_, String>("SELECT status FROM meetings WHERE id = $1")
        .bind(meeting_id)
        .fetch_optional(db)
        .await?
        .ok_or_else(|| anyhow!("Meeting not found"))?;

    if status != "active" {
        return Err(anyhow!("Meeting is not active"));
    }

    sqlx::query(
        r#"
        INSERT INTO meeting_participants (id, meeting_id, user_id, role)
        VALUES ($1, $2, $3, 'participant')
        ON CONFLICT (meeting_id, user_id)
        DO UPDATE SET joined_at = now(), left_at = NULL
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(meeting_id)
    .bind(user_id)
    .execute(db)
    .await?;

    list_active_participants(db, meeting_id).await
}

pub async fn leave_meeting(db: &Db, meeting_id: Uuid, user_id: Uuid) -> Result<()> {
    sqlx::query(
        "UPDATE meeting_participants SET left_at = now() WHERE meeting_id = $1 AND user_id = $2",
    )
    .bind(meeting_id)
    .bind(user_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn end_meeting(db: &Db, meeting_id: Uuid, user_id: Uuid) -> Result<()> {
    let role = sqlx::query_scalar::<_, String>(
        "SELECT role FROM meeting_participants WHERE meeting_id = $1 AND user_id = $2",
    )
    .bind(meeting_id)
    .bind(user_id)
    .fetch_optional(db)
    .await?;

    if role.as_deref() != Some("host") {
        return Err(anyhow!("Only the host can end this meeting"));
    }

    sqlx::query("UPDATE meetings SET status = 'ended', ended_at = now() WHERE id = $1")
        .bind(meeting_id)
        .execute(db)
        .await?;

    sqlx::query(
        "UPDATE meeting_participants SET left_at = now() WHERE meeting_id = $1 AND left_at IS NULL",
    )
    .bind(meeting_id)
    .execute(db)
    .await?;

    Ok(())
}

pub async fn list_active_participants(
    db: &Db,
    meeting_id: Uuid,
) -> Result<Vec<ParticipantResponse>> {
    let participants = sqlx::query_as::<_, ParticipantResponse>(
        r#"
        SELECT mp.user_id, CONCAT(p.first_name, ' ', p.last_name) as display_name, u.email,
            mp.role, mp.joined_at
        FROM meeting_participants mp
        JOIN users u ON mp.user_id = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        WHERE mp.meeting_id = $1 AND mp.left_at IS NULL
        ORDER BY mp.joined_at ASC
        "#,
    )
    .bind(meeting_id)
    .fetch_all(db)
    .await?;

    Ok(participants)
}

pub async fn list_my_meetings(db: &Db, user_id: Uuid) -> Result<Vec<MeetingResponse>> {
    let meetings = sqlx::query_as::<_, MeetingResponse>(
        r#"
        SELECT m.id, m.title, m.channel_id, m.host_id,
            CONCAT(p.first_name, ' ', p.last_name) as host_name,
            m.status, m.created_at, m.ended_at
        FROM meetings m
        LEFT JOIN users u ON m.host_id = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        INNER JOIN meeting_participants mp ON mp.meeting_id = m.id
        WHERE mp.user_id = $1
        ORDER BY m.created_at DESC
        LIMIT 20
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    Ok(meetings)
}
