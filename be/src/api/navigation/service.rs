use anyhow::Result;
use crate::models::navigation_item::NavigationItem;
use chrono::Utc;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use super::dto::{CreateNavigationItemDto, UpdateNavigationItemDto, UserNavigationItemDto};

pub async fn create_navigation_item(
    pool: &PgPool,
    dto: CreateNavigationItemDto,
) -> Result<NavigationItem> {
    let nav_item = sqlx::query_as::<_, NavigationItem>(
        r#"
        INSERT INTO navigation_items (name, path, icon, parent_id, display_order, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $6)
        RETURNING *
        "#,
    )
    .bind(&dto.name)
    .bind(&dto.path)
    .bind(&dto.icon)
    .bind(&dto.parent_id)
    .bind(dto.display_order.unwrap_or(0))
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await
    ?;

    Ok(nav_item)
}

pub async fn get_navigation_items(
    pool: &PgPool,
    is_active: Option<bool>,
) -> Result<Vec<NavigationItem>> {
    let query = match is_active {
        Some(active) => sqlx::query_as::<_, NavigationItem>(
            "SELECT * FROM navigation_items WHERE is_active = $1 ORDER BY display_order, name",
        )
        .bind(active),
        None => sqlx::query_as::<_, NavigationItem>(
            "SELECT * FROM navigation_items ORDER BY display_order, name",
        ),
    };

    let items = query
        .fetch_all(pool)
        .await
        ?;

    Ok(items)
}

pub async fn get_navigation_item_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<NavigationItem> {
    let item = sqlx::query_as::<_, NavigationItem>("SELECT * FROM navigation_items WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        ?;

    Ok(item)
}

pub async fn update_navigation_item(
    pool: &PgPool,
    id: Uuid,
    dto: UpdateNavigationItemDto,
) -> Result<NavigationItem> {
    let current = get_navigation_item_by_id(pool, id).await?;

    let item = sqlx::query_as::<_, NavigationItem>(
        r#"
        UPDATE navigation_items
        SET name = $1,
            path = $2,
            icon = $3,
            parent_id = $4,
            display_order = $5,
            is_active = $6,
            updated_at = $7
        WHERE id = $8
        RETURNING *
        "#,
    )
    .bind(dto.name.unwrap_or(current.name))
    .bind(dto.path.unwrap_or(current.path))
    .bind(dto.icon.or(current.icon))
    .bind(dto.parent_id.or(current.parent_id))
    .bind(dto.display_order.unwrap_or(current.display_order))
    .bind(dto.is_active.unwrap_or(current.is_active))
    .bind(Utc::now().naive_utc())
    .bind(id)
    .fetch_one(pool)
    .await
    ?;

    Ok(item)
}

pub async fn delete_navigation_item(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("UPDATE navigation_items SET is_active = false, updated_at = $1 WHERE id = $2")
        .bind(Utc::now().naive_utc())
        .bind(id)
        .execute(pool)
        .await
        ?;

    Ok(())
}

#[derive(sqlx::FromRow)]
struct NavigationWithPermissions {
    id: Uuid,
    name: String,
    path: String,
    icon: Option<String>,
    parent_id: Option<Uuid>,
    display_order: i32,
    can_create: Option<bool>,
    can_read: Option<bool>,
    can_update: Option<bool>,
    can_delete: Option<bool>,
}

pub async fn get_user_navigation(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<UserNavigationItemDto>> {
    // Get user's department and position from employees or interns
    let role_info = sqlx::query!(
        r#"
        SELECT department_id, position_id FROM employees 
        WHERE person_id = (SELECT person_id FROM users WHERE id = $1)
        UNION
        SELECT department_id, position_id FROM interns 
        WHERE person_id = (SELECT person_id FROM users WHERE id = $1)
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
    ?;

    if role_info.is_none() {
        // User has no employee/intern record - return empty navigation
        return Ok(vec![]);
    }

    let role = role_info.unwrap();

    // Get navigation items with aggregated permissions
    let nav_with_perms = sqlx::query_as::<_, NavigationWithPermissions>(
        r#"
        SELECT DISTINCT 
            n.id,
            n.name,
            n.path,
            n.icon,
            n.parent_id,
            n.display_order,
            COALESCE(MAX(rp.can_create::int), 0)::bool as can_create,
            COALESCE(MAX(rp.can_read::int), 0)::bool as can_read,
            COALESCE(MAX(rp.can_update::int), 0)::bool as can_update,
            COALESCE(MAX(rp.can_delete::int), 0)::bool as can_delete
        FROM navigation_items n
        INNER JOIN role_permissions rp ON n.id = rp.navigation_item_id
        WHERE n.is_active = true
        AND (
            (rp.department_id = $1 OR rp.position_id = $2)
        )
        GROUP BY n.id, n.name, n.path, n.icon, n.parent_id, n.display_order
        ORDER BY n.display_order, n.name
        "#,
    )
    .bind(role.department_id)
    .bind(role.position_id)
    .fetch_all(pool)
    .await
    ?;

    // Build hierarchical structure
    let mut items_map: HashMap<Uuid, UserNavigationItemDto> = HashMap::new();
    let mut root_items: Vec<UserNavigationItemDto> = Vec::new();

    // First pass: create all items
    for nav in &nav_with_perms {
        let item = UserNavigationItemDto {
            id: nav.id,
            name: nav.name.clone(),
            path: nav.path.clone(),
            icon: nav.icon.clone(),
            parent_id: nav.parent_id,
            display_order: nav.display_order,
            can_create: nav.can_create.unwrap_or(false),
            can_read: nav.can_read.unwrap_or(false),
            can_update: nav.can_update.unwrap_or(false),
            can_delete: nav.can_delete.unwrap_or(false),
            children: vec![],
        };
        items_map.insert(nav.id, item);
    }

    // Second pass: build hierarchy
    for nav in &nav_with_perms {
        if let Some(parent_id) = nav.parent_id {
            if let Some(item) = items_map.remove(&nav.id) {
                if let Some(parent) = items_map.get_mut(&parent_id) {
                    parent.children.push(item);
                }
            }
        }
    }

    // Collect root items
    for nav in &nav_with_perms {
        if nav.parent_id.is_none() {
            if let Some(item) = items_map.remove(&nav.id) {
                root_items.push(item);
            }
        }
    }

    root_items.sort_by_key(|item| item.display_order);

    Ok(root_items)
}
