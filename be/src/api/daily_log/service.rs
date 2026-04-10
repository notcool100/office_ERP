use super::dto::{
    CreateDailyLogDto, DailyLogLinkResponse, DailyLogResponse, ListDailyLogQuery, UpdateDailyLogDto,
};
use crate::db::Db;
use crate::models::daily_log::{DailyLog, DailyLogWithUser};
use anyhow::Result;
use sqlx::Row;
use uuid::Uuid;

pub async fn create_daily_log(
    db: &Db,
    user_id: Uuid,
    dto: CreateDailyLogDto,
) -> Result<DailyLogResponse> {
    let mut tx = db.begin().await?;

    let log = sqlx::query_as::<_, DailyLog>(
        r#"
        INSERT INTO daily_logs (user_id, log_date, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(dto.log_date)
    .bind(&dto.content)
    .fetch_one(&mut *tx)
    .await?;

    if let Some(card_ids) = dto.card_ids {
        for card_id in card_ids {
            sqlx::query("INSERT INTO daily_log_links (daily_log_id, card_id) VALUES ($1, $2)")
                .bind(log.id)
                .bind(card_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    tx.commit().await?;

    get_daily_log_response(db, log.id).await
}

pub async fn update_daily_log(
    db: &Db,
    user_id: Uuid,
    id: Uuid,
    dto: UpdateDailyLogDto,
) -> Result<DailyLogResponse> {
    let mut tx = db.begin().await?;

    // Verify ownership
    let existing_user_id = sqlx::query("SELECT user_id FROM daily_logs WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *tx)
        .await?
        .try_get::<Uuid, _>("user_id")?;

    if existing_user_id != user_id {
        return Err(anyhow::anyhow!("Unauthorized"));
    }

    if dto.log_date.is_some() || dto.content.is_some() {
        sqlx::query(
            r#"
            UPDATE daily_logs
            SET log_date = COALESCE($1, log_date),
                content = COALESCE($2, content),
                updated_at = NOW()
            WHERE id = $3
            "#,
        )
        .bind(dto.log_date)
        .bind(dto.content)
        .bind(id)
        .execute(&mut *tx)
        .await?;
    }

    if let Some(card_ids) = dto.card_ids {
        // Simple strategy: Clear and re-insert
        sqlx::query("DELETE FROM daily_log_links WHERE daily_log_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        for card_id in card_ids {
            sqlx::query("INSERT INTO daily_log_links (daily_log_id, card_id) VALUES ($1, $2)")
                .bind(id)
                .bind(card_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    tx.commit().await?;

    get_daily_log_response(db, id).await
}

pub async fn delete_daily_log(db: &Db, user_id: Uuid, id: Uuid) -> Result<()> {
    // Verify ownership
    let existing_user_id = sqlx::query("SELECT user_id FROM daily_logs WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await?
        .try_get::<Uuid, _>("user_id")?;

    if existing_user_id != user_id {
        return Err(anyhow::anyhow!("Unauthorized"));
    }

    sqlx::query("DELETE FROM daily_logs WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn list_daily_logs(db: &Db, query: ListDailyLogQuery) -> Result<Vec<DailyLogResponse>> {
    let mut sql = r#"
        SELECT dl.*, u.user_name
        FROM daily_logs dl
        JOIN users u ON u.id = dl.user_id
        WHERE 1=1
    "#
    .to_string();

    if query.user_id.is_some() {
        sql.push_str(" AND dl.user_id = $1");
    }
    if query.start_date.is_some() {
        sql.push_str(" AND dl.log_date >= $2");
    }
    if query.end_date.is_some() {
        sql.push_str(" AND dl.log_date <= $3");
    }

    sql.push_str(" ORDER BY dl.log_date DESC, dl.created_at DESC");

    let mut q = sqlx::query_as::<_, DailyLogWithUser>(&sql);
    if let Some(user_id) = query.user_id {
        q = q.bind(user_id);
    }
    if let Some(start_date) = query.start_date {
        q = q.bind(start_date);
    }
    if let Some(end_date) = query.end_date {
        q = q.bind(end_date);
    }

    let logs = q.fetch_all(db).await?;
    let mut responses = Vec::new();

    for log in logs {
        let links = fetch_links(db, log.id).await?;
        responses.push(DailyLogResponse {
            id: log.id,
            user_id: log.user_id,
            user_name: log.user_name,
            log_date: log.log_date,
            content: log.content,
            created_at: log.created_at,
            updated_at: log.updated_at,
            links,
        });
    }

    Ok(responses)
}

async fn get_daily_log_response(db: &Db, id: Uuid) -> Result<DailyLogResponse> {
    let log = sqlx::query_as::<_, DailyLogWithUser>(
        r#"
        SELECT dl.*, u.user_name
        FROM daily_logs dl
        JOIN users u ON u.id = dl.user_id
        WHERE dl.id = $1
        "#,
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    let links = fetch_links(db, id).await?;

    Ok(DailyLogResponse {
        id: log.id,
        user_id: log.user_id,
        user_name: log.user_name,
        log_date: log.log_date,
        content: log.content,
        created_at: log.created_at,
        updated_at: log.updated_at,
        links,
    })
}

async fn fetch_links(db: &Db, daily_log_id: Uuid) -> Result<Vec<DailyLogLinkResponse>> {
    let links = sqlx::query_as::<_, DailyLogLinkResponse>(
        r#"
        SELECT dll.id, c.id as card_id, c.card_key, c.title as card_title
        FROM daily_log_links dll
        JOIN cards c ON c.id = dll.card_id
        WHERE dll.daily_log_id = $1
        "#,
    )
    .bind(daily_log_id)
    .fetch_all(db)
    .await?;

    Ok(links)
}
