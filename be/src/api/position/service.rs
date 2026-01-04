use crate::models::position::Position;
use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::dto::{CreatePositionDto, UpdatePositionDto};

pub async fn create_position(
    pool: &PgPool,
    dto: CreatePositionDto,
) -> Result<Position> {
    let position = sqlx::query_as::<_, Position>(
        r#"
        INSERT INTO positions (name, description, created_at, updated_at)
        VALUES ($1, $2, $3, $3)
        RETURNING *
        "#,
    )
    .bind(&dto.name)
    .bind(&dto.description)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await?;

    Ok(position)
}

pub async fn get_positions(
    pool: &PgPool,
    is_active: Option<bool>,
) -> Result<Vec<Position>> {
    let query = match is_active {
        Some(active) => {
            sqlx::query_as::<_, Position>("SELECT * FROM positions WHERE is_active = $1 ORDER BY name")
                .bind(active)
        }
        None => sqlx::query_as::<_, Position>("SELECT * FROM positions ORDER BY name"),
    };

    let positions = query.fetch_all(pool).await?;
    Ok(positions)
}

pub async fn get_position_by_id(pool: &PgPool, id: Uuid) -> Result<Position> {
    let position = sqlx::query_as::<_, Position>("SELECT * FROM positions WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Position not found"))?;

    Ok(position)
}

pub async fn update_position(
    pool: &PgPool,
    id: Uuid,
    dto: UpdatePositionDto,
) -> Result<Position> {
    let current = get_position_by_id(pool, id).await?;

    let position = sqlx::query_as::<_, Position>(
        r#"
        UPDATE positions
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            is_active = COALESCE($3, is_active),
            updated_at = $4
        WHERE id = $5
        RETURNING *
        "#,
    )
    .bind(dto.name.as_ref().unwrap_or(&current.name))
    .bind(dto.description.or(current.description))
    .bind(dto.is_active.unwrap_or(current.is_active))
    .bind(Utc::now().naive_utc())
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(position)
}

pub async fn delete_position(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("UPDATE positions SET is_active = false, updated_at = $1 WHERE id = $2")
        .bind(Utc::now().naive_utc())
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
