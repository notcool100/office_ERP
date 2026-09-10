use crate::{
    api::meetings::dto::{
        AttachmentResponse, CreateMeetingRequest, MeetingResponse, MinutesResponse,
        ParticipantResponse,
    },
    api::messaging::dto::{MessageResponse, SendMessageRequest},
    db::Db,
};
use anyhow::{Result, anyhow};
use chrono::Utc;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const MAX_DOCUMENT_SIZE: usize = 50 * 1024 * 1024; // 50 MB
const MAX_RECORDING_SIZE: usize = 1024 * 1024 * 1024; // 1 GB
const VALID_ATTACHMENT_CATEGORIES: &[&str] = &["document", "recording"];

/// Marker prefix used so handlers can distinguish an authorization failure
/// from other anyhow errors without introducing a typed error enum for this
/// module, matching the "bare anyhow -> StatusCode" convention already used
/// throughout this file.
const NOT_PARTICIPANT_ERR: &str = "Not a participant of this meeting";

fn meeting_upload_dir() -> PathBuf {
    let base =
        std::env::var("MEETING_UPLOAD_DIR").unwrap_or_else(|_| "./uploads/meetings".into());
    PathBuf::from(base)
}

async fn is_participant(db: &Db, meeting_id: Uuid, user_id: Uuid) -> Result<bool> {
    Ok(sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM meeting_participants WHERE meeting_id = $1 AND user_id = $2)",
    )
    .bind(meeting_id)
    .bind(user_id)
    .fetch_one(db)
    .await?)
}

async fn require_participant(db: &Db, meeting_id: Uuid, user_id: Uuid) -> Result<()> {
    if !is_participant(db, meeting_id, user_id).await? {
        return Err(anyhow!(NOT_PARTICIPANT_ERR));
    }
    Ok(())
}

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
            Vec::new(),
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

const ATTACHMENT_SELECT: &str = r#"
    SELECT ma.id, ma.meeting_id, ma.uploaded_by,
        CONCAT(p.first_name, ' ', p.last_name) as uploaded_by_name,
        ma.category, ma.file_name, ma.content_type, ma.file_size, ma.created_at,
        ma.file_path
    FROM meeting_attachments ma
    LEFT JOIN users u ON ma.uploaded_by = u.id
    LEFT JOIN persons p ON u.person_id = p.id
"#;

pub async fn list_attachments(
    db: &Db,
    meeting_id: Uuid,
    caller_id: Uuid,
) -> Result<Vec<AttachmentResponse>> {
    require_participant(db, meeting_id, caller_id).await?;

    let attachments = sqlx::query_as::<_, AttachmentResponse>(&format!(
        "{} WHERE ma.meeting_id = $1 ORDER BY ma.created_at DESC",
        ATTACHMENT_SELECT
    ))
    .bind(meeting_id)
    .fetch_all(db)
    .await?;

    Ok(attachments)
}

pub async fn upload_attachment(
    db: &Db,
    meeting_id: Uuid,
    caller_id: Uuid,
    category: String,
    file_name: String,
    content_type: String,
    file_data: Vec<u8>,
) -> Result<AttachmentResponse> {
    require_participant(db, meeting_id, caller_id).await?;

    if file_data.is_empty() {
        return Err(anyhow!("File is empty"));
    }

    let category = if category.is_empty() {
        "document".to_string()
    } else if VALID_ATTACHMENT_CATEGORIES.contains(&category.as_str()) {
        category
    } else {
        "document".to_string()
    };

    let max_size = if category == "recording" {
        MAX_RECORDING_SIZE
    } else {
        MAX_DOCUMENT_SIZE
    };
    if file_data.len() > max_size {
        return Err(anyhow!(
            "File too large for category '{}' (max {} bytes)",
            category,
            max_size
        ));
    }

    // Build a safe, unique filename: <uuid>.<ext> — never trust the
    // client-supplied original filename for the on-disk name.
    let ext = Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin")
        .to_lowercase();
    let stored_name = format!("{}.{}", Uuid::new_v4(), ext);

    let dir = meeting_upload_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let abs_path = dir.join(&stored_name);
    tokio::fs::write(&abs_path, &file_data).await?;

    let file_path = abs_path.to_string_lossy().into_owned();
    let file_size = file_data.len() as i64;
    let attachment_id = Uuid::new_v4();

    sqlx::query(
        r#"
        INSERT INTO meeting_attachments
            (id, meeting_id, uploaded_by, category, file_name, content_type, file_size, file_path)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(attachment_id)
    .bind(meeting_id)
    .bind(caller_id)
    .bind(&category)
    .bind(&file_name)
    .bind(&content_type)
    .bind(file_size)
    .bind(&file_path)
    .execute(db)
    .await?;

    let attachment = sqlx::query_as::<_, AttachmentResponse>(&format!(
        "{} WHERE ma.id = $1",
        ATTACHMENT_SELECT
    ))
    .bind(attachment_id)
    .fetch_one(db)
    .await?;

    Ok(attachment)
}

pub async fn get_attachment_file(
    db: &Db,
    meeting_id: Uuid,
    attachment_id: Uuid,
    caller_id: Uuid,
) -> Result<AttachmentResponse> {
    require_participant(db, meeting_id, caller_id).await?;

    let attachment = sqlx::query_as::<_, AttachmentResponse>(&format!(
        "{} WHERE ma.id = $1 AND ma.meeting_id = $2",
        ATTACHMENT_SELECT
    ))
    .bind(attachment_id)
    .bind(meeting_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Attachment not found"))?;

    Ok(attachment)
}

pub async fn delete_attachment(
    db: &Db,
    meeting_id: Uuid,
    attachment_id: Uuid,
    caller_id: Uuid,
) -> Result<()> {
    require_participant(db, meeting_id, caller_id).await?;

    let attachment = sqlx::query_as::<_, AttachmentResponse>(&format!(
        "{} WHERE ma.id = $1 AND ma.meeting_id = $2",
        ATTACHMENT_SELECT
    ))
    .bind(attachment_id)
    .bind(meeting_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Attachment not found"))?;

    let host_id =
        sqlx::query_scalar::<_, Option<Uuid>>("SELECT host_id FROM meetings WHERE id = $1")
            .bind(meeting_id)
            .fetch_optional(db)
            .await?
            .flatten();

    let is_uploader = attachment.uploaded_by == Some(caller_id);
    let is_host = host_id == Some(caller_id);
    if !is_uploader && !is_host {
        return Err(anyhow!(
            "Only the uploader or the meeting host can delete this attachment"
        ));
    }

    sqlx::query("DELETE FROM meeting_attachments WHERE id = $1")
        .bind(attachment_id)
        .execute(db)
        .await?;

    // Best-effort disk cleanup — don't fail if the file is already gone.
    let _ = tokio::fs::remove_file(&attachment.file_path).await;

    Ok(())
}

pub async fn get_minutes(db: &Db, meeting_id: Uuid, caller_id: Uuid) -> Result<MinutesResponse> {
    require_participant(db, meeting_id, caller_id).await?;

    let minutes = sqlx::query_as::<_, MinutesResponse>(
        r#"
        SELECT mm.meeting_id, mm.content, mm.updated_by,
            CONCAT(p.first_name, ' ', p.last_name) as updated_by_name,
            mm.updated_at
        FROM meeting_minutes mm
        LEFT JOIN users u ON mm.updated_by = u.id
        LEFT JOIN persons p ON u.person_id = p.id
        WHERE mm.meeting_id = $1
        "#,
    )
    .bind(meeting_id)
    .fetch_optional(db)
    .await?;

    Ok(minutes.unwrap_or_else(|| MinutesResponse {
        meeting_id,
        content: String::new(),
        updated_by: None,
        updated_by_name: None,
        updated_at: Utc::now(),
    }))
}

pub async fn update_minutes(
    db: &Db,
    meeting_id: Uuid,
    caller_id: Uuid,
    content: String,
) -> Result<MinutesResponse> {
    require_participant(db, meeting_id, caller_id).await?;

    sqlx::query(
        r#"
        INSERT INTO meeting_minutes (meeting_id, content, updated_by, updated_at)
        VALUES ($1, $2, $3, now())
        ON CONFLICT (meeting_id)
        DO UPDATE SET content = $2, updated_by = $3, updated_at = now()
        "#,
    )
    .bind(meeting_id)
    .bind(&content)
    .bind(caller_id)
    .execute(db)
    .await?;

    get_minutes(db, meeting_id, caller_id).await
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
