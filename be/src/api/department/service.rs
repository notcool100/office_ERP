use crate::models::department::Department;
use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::dto::{CreateDepartmentDto, UpdateDepartmentDto};

pub async fn create_department(
    pool: &PgPool,
    dto: CreateDepartmentDto,
) -> Result<Department> {
    let department = sqlx::query_as::<_, Department>(
        r#"
        INSERT INTO departments (name, description, created_at, updated_at)
        VALUES ($1, $2, $3, $3)
        RETURNING *
        "#,
    )
    .bind(&dto.name)
    .bind(&dto.description)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await?;

    Ok(department)
}

pub async fn get_departments(
    pool: &PgPool,
    is_active: Option<bool>,
) -> Result<Vec<Department>> {
    let query = match is_active {
        Some(active) => sqlx::query_as::<_, Department>(
            "SELECT * FROM departments WHERE is_active = $1 ORDER BY name",
        )
        .bind(active),
        None => sqlx::query_as::<_, Department>("SELECT * FROM departments ORDER BY name"),
    };

    let departments = query.fetch_all(pool).await?;
    Ok(departments)
}

pub async fn get_department_by_id(pool: &PgPool, id: Uuid) -> Result<Department> {
    let department = sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Department not found"))?;

    Ok(department)
}

pub async fn update_department(
    pool: &PgPool,
    id: Uuid,
    dto: UpdateDepartmentDto,
) -> Result<Department> {
    let current = get_department_by_id(pool, id).await?;

    let department = sqlx::query_as::<_, Department>(
        r#"
        UPDATE departments
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

    Ok(department)
}

pub async fn delete_department(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("UPDATE departments SET is_active = false, updated_at = $1 WHERE id = $2")
        .bind(Utc::now().naive_utc())
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
