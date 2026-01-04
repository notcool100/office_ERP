use anyhow::{anyhow, Result};
use crate::models::role_permission::RolePermission;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use super::dto::{AssignPermissionDto, PermissionResponseDto};

pub async fn assign_permission(
    pool: &PgPool,
    dto: AssignPermissionDto,
) -> Result<RolePermission> {
    // Validate that at least one of department_id or position_id is provided
    if dto.department_id.is_none() && dto.position_id.is_none() {
        return Err(anyhow!(
            "Either department_id or position_id must be provided"
        ));
    }

    // Use INSERT ON CONFLICT to update if permission already exists
    let permission = sqlx::query_as::<_, RolePermission>(
        r#"
        INSERT INTO role_permissions (
            department_id, position_id, navigation_item_id,
            can_create, can_read, can_update, can_delete, created_at
        )VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (department_id, position_id, navigation_item_id)
        DO UPDATE SET
            can_create = EXCLUDED.can_create,
            can_read = EXCLUDED.can_read,
            can_update = EXCLUDED.can_update,
            can_delete = EXCLUDED.can_delete
        RETURNING *
        "#,
    )
    .bind(dto.department_id)
    .bind(dto.position_id)
    .bind(dto.navigation_item_id)
    .bind(dto.can_create)
    .bind(dto.can_read)
    .bind(dto.can_update)
    .bind(dto.can_delete)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await
    ?;

    Ok(permission)
}

#[derive(sqlx::FromRow)]
struct PermissionWithNames {
    id: Uuid,
    department_id: Option<Uuid>,
    department_name: Option<String>,
    position_id: Option<Uuid>,
    position_name: Option<String>,
    navigation_item_id: Uuid,
    navigation_name: String,
    navigation_path: String,
    can_create: bool,
    can_read: bool,
    can_update: bool,
    can_delete: bool,
    created_at: chrono::NaiveDateTime,
}

pub async fn get_permissions(
    pool: &PgPool,
    department_id: Option<Uuid>,
    position_id: Option<Uuid>,
    navigation_item_id: Option<Uuid>,
) -> Result<Vec<PermissionResponseDto>> {
    let mut query = String::from(
        r#"
        SELECT 
            rp.id,
            rp.department_id,
            d.name as department_name,
            rp.position_id,
            p.name as position_name,
            rp.navigation_item_id,
            n.name as navigation_name,
            n.path as navigation_path,
            rp.can_create,
            rp.can_read,
            rp.can_update,
            rp.can_delete,
            rp.created_at
        FROM role_permissions rp
        LEFT JOIN departments d ON rp.department_id = d.id
        LEFT JOIN positions p ON rp.position_id = p.id
        INNER JOIN navigation_items n ON rp.navigation_item_id = n.id
        WHERE 1=1
        "#,
    );

    let mut bind_count = 0;
    if department_id.is_some() {
        bind_count += 1;
        query.push_str(&format!(" AND rp.department_id = ${}", bind_count));
    }
    if position_id.is_some() {
        bind_count += 1;
        query.push_str(&format!(" AND rp.position_id = ${}", bind_count));
    }
    if navigation_item_id.is_some() {
        bind_count += 1;
        query.push_str(&format!(" AND rp.navigation_item_id = ${}", bind_count));
    }

    query.push_str(" ORDER BY d.name, p.name, n.name");

    let mut query_builder = sqlx::query_as::<_, PermissionWithNames>(&query);

    if let Some(dept_id) = department_id {
        query_builder = query_builder.bind(dept_id);
    }
    if let Some(pos_id) = position_id {
        query_builder = query_builder.bind(pos_id);
    }
    if let Some(nav_id) = navigation_item_id {
        query_builder = query_builder.bind(nav_id);
    }

    let perms = query_builder
        .fetch_all(pool)
        .await
        ?;

    let response = perms
        .into_iter()
        .map(|p| PermissionResponseDto {
            id: p.id,
            department_id: p.department_id,
            department_name: p.department_name,
            position_id: p.position_id,
            position_name: p.position_name,
            navigation_item_id: p.navigation_item_id,
            navigation_name: p.navigation_name,
            navigation_path: p.navigation_path,
            can_create: p.can_create,
            can_read: p.can_read,
            can_update: p.can_update,
            can_delete: p.can_delete,
            created_at: p.created_at,
        })
        .collect();

    Ok(response)
}

pub async fn delete_permission(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM role_permissions WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        ?;

    Ok(())
}
