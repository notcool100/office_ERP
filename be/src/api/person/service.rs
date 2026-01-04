use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::person::Person;
use super::dto::{CreatePersonDto, ListPersonsQuery, ListPersonsResponse, PersonResponseDto, UpdatePersonDto};

pub async fn create_person(pool: &PgPool, dto: CreatePersonDto) -> Result<Person> {
    let person = sqlx::query_as::<_, Person>(
        r#"
        INSERT INTO persons (id, first_name, middle_name, last_name, created_at)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&dto.first_name)
    .bind(&dto.middle_name)
    .bind(&dto.last_name)
    .bind(Utc::now().naive_utc())
    .fetch_one(pool)
    .await?;

    Ok(person)
}

pub async fn get_person_by_id(pool: &PgPool, id: Uuid) -> Result<Person> {
    let person = sqlx::query_as::<_, Person>("SELECT * FROM persons WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Person not found"))?;

    Ok(person)
}

pub async fn list_persons(pool: &PgPool, query: ListPersonsQuery) -> Result<ListPersonsResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).min(100);
    let offset = (page - 1) * page_size;

    let mut conditions: Vec<String> = vec!["1=1".to_string()];
    let mut count_query = "SELECT COUNT(*) FROM persons".to_string();
    let mut select_query = "SELECT * FROM persons".to_string();

    if let Some(_search) = &query.search {
        conditions.push("(first_name ILIKE $1 OR last_name ILIKE $1 OR middle_name ILIKE $1)".to_string());
    }

    if conditions.len() > 1 {
        let where_clause = format!(" WHERE {}", conditions[1..].join(" AND "));
        count_query.push_str(&where_clause);
        select_query.push_str(&where_clause);
    }

    select_query.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", if query.search.is_some() { 2 } else { 1 }, if query.search.is_some() { 3 } else { 2 }));

    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    let mut select_q = sqlx::query_as::<_, Person>(&select_query);

    let pattern = if let Some(search) = &query.search {
        Some(format!("%{}%", search))
    } else {
        None
    };

    if let Some(p) = &pattern {
        count_q = count_q.bind(p);
        select_q = select_q.bind(p);
    }

    select_q = select_q.bind(page_size).bind(offset);

    let total = count_q.fetch_one(pool).await?;
    let persons = select_q.fetch_all(pool).await?;

    Ok(ListPersonsResponse {
        persons: persons.into_iter().map(map_person_to_response).collect(),
        total,
        page,
        page_size,
    })
}

pub async fn update_person(pool: &PgPool, id: Uuid, dto: UpdatePersonDto) -> Result<Person> {
    let current = get_person_by_id(pool, id).await?;

    let person = sqlx::query_as::<_, Person>(
        r#"
        UPDATE persons
        SET first_name = COALESCE($1, first_name),
            middle_name = COALESCE($2, middle_name),
            last_name = COALESCE($3, last_name)
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(dto.first_name.as_ref().unwrap_or(&current.first_name))
    // For middle_name, we want to allow updating TO null if needed, but current logic COALESCEs if None.
    // If the DTO has None, it means "don't update". If it has Some(None), it's not possible with this DTO structure.
    // The UpdatePersonDto has Option<String>. If we want to support clearing, we need Option<Option<String>>.
    // For simplicity, we assume None means no change. If user sends Some(""), we can treat as None in DB if we want, but let's stick to simple update.
    .bind(dto.middle_name.or(current.middle_name)) 
    .bind(dto.last_name.as_ref().unwrap_or(&current.last_name))
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(person)
}

pub async fn delete_person(pool: &PgPool, id: Uuid) -> Result<()> {
    // Check if linked to anything? DB constraints (ON DELETE CASCADE) will handle it or fail.
    // Our DB schema says ON DELETE CASCADE for foreign keys, so deleting person deletes employee/user/etc.
    // That's dangerous! Ideally we should check first.
    // But per instructions and schema, we'll allow it.
    let result = sqlx::query("DELETE FROM persons WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow!("Person not found"));
    }

    Ok(())
}

pub fn map_person_to_response(person: Person) -> PersonResponseDto {
    PersonResponseDto {
        id: person.id,
        first_name: person.first_name,
        middle_name: person.middle_name,
        last_name: person.last_name,
        created_at: person.created_at,
    }
}
