use crate::{
    api::intern::dto::{
        CreateInternRequest, InternResponse, ListInternsQuery, ListInternsResponse,
        UpdateInternRequest,
    },
    db::Db,
    models::intern::InternWithPerson,
};
use anyhow::{anyhow, Result};
use sqlx::types::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

pub async fn create_intern(db: &Db, req: CreateInternRequest) -> Result<InternResponse> {
    // Verify person exists
    let person_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM persons WHERE id = $1)"
    )
    .bind(req.person_id)
    .fetch_one(db)
    .await?;

    if !person_exists {
        return Err(anyhow!("Person not found"));
    }

    // Check if intern_id is already taken
    let intern_id_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM interns WHERE intern_id = $1)"
    )
    .bind(&req.intern_id)
    .fetch_one(db)
    .await?;

    if intern_id_exists {
        return Err(anyhow!("Intern ID already exists"));
    }

    let stipend = req.stipend.map(|s| BigDecimal::from_str(&s.to_string()).unwrap());

    let intern = sqlx::query_as::<_, InternWithPerson>(
        r#"
        WITH new_int AS (
            INSERT INTO interns (intern_id, person_id, department, supervisor_id, start_date, 
                                 end_date, stipend, university, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'active', NOW(), NOW())
            RETURNING *
        )
        SELECT i.id, i.intern_id, i.person_id, 
               p.first_name, p.middle_name, p.last_name,
               pc.email, pc.phone,
               i.department, i.supervisor_id, i.start_date,
               i.end_date, i.stipend, i.university,
               i.status, i.created_at, i.updated_at
       FROM new_int i
        JOIN persons p ON p.id = i.person_id
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        "#,
    )
    .bind(&req.intern_id)
    .bind(req.person_id)
    .bind(&req.department)
    .bind(req.supervisor_id)
    .bind(req.start_date)
    .bind(req.end_date)
    .bind(stipend)
    .bind(&req.university)
    .fetch_one(db)
    .await?;

    Ok(map_intern_to_response(intern))
}

pub async fn get_intern(db: &Db, id: Uuid) -> Result<InternResponse> {
    let intern = sqlx::query_as::<_, InternWithPerson>(
        r#"
        SELECT i.id, i.intern_id, i.person_id, 
               p.first_name, p.middle_name, p.last_name,
               pc.email, pc.phone,
               i.department, i.supervisor_id, i.start_date,
               i.end_date, i.stipend, i.university,
               i.status, i.created_at, i.updated_at
        FROM interns i
        JOIN persons p ON p.id = i.person_id
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        WHERE i.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Intern not found"))?;

    Ok(map_intern_to_response(intern))
}

pub async fn list_interns(db: &Db, query: ListInternsQuery) -> Result<ListInternsResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).min(100);
    let offset = (page - 1) * page_size;

    let mut conditions: Vec<String> = vec!["1=1".to_string()];
    let mut count_query = "SELECT COUNT(*) FROM interns i".to_string();
    let mut select_query = format!(
        r#"
        SELECT i.id, i.intern_id, i.person_id, 
               p.first_name, p.middle_name, p.last_name,
               pc.email, pc.phone,
               i.department, i.supervisor_id, i.start_date,
               i.end_date, i.stipend, i.university,
               i.status, i.created_at, i.updated_at
        FROM interns i
        JOIN persons p ON p.id = i.person_id
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        WHERE 1=1
        "#
    );

    if query.department.is_some() {
        conditions.push("i.department = $3".to_string());
    }

    if query.status.is_some() {
        let idx = if query.department.is_some() { 4 } else { 3 };
        conditions.push(format!("i.status = ${}", idx));
    }

    if let Some(_search) = &query.search {
        let idx = 3 + conditions.len() as i32 - 1;
        conditions.push(format!(
            "(p.first_name ILIKE ${0} OR p.last_name ILIKE ${0} OR i.intern_id ILIKE ${0})",
            idx
        ));
    }

    if conditions.len() > 1 {
        let where_clause = format!(" AND {}", conditions[1..].join(" AND "));
        count_query = format!("{} {}", count_query, where_clause);
        select_query = format!("{} {}", select_query, where_clause);
    }

    select_query = format!(
        "{} ORDER BY i.created_at DESC LIMIT $1 OFFSET $2",
        select_query
    );

    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    let mut select_q = sqlx::query_as::<_, InternWithPerson>(&select_query)
        .bind(page_size)
        .bind(offset);

    if let Some(dept) = &query.department {
        count_q = count_q.bind(dept);
        select_q = select_q.bind(dept);
    }

    if let Some(status) = &query.status {
        count_q = count_q.bind(status);
        select_q = select_q.bind(status);
    }

    let search_pattern = query.search.map(|s| format!("%{}%", s));
    if let Some(pattern) = &search_pattern {
        count_q = count_q.bind(pattern);
        select_q = select_q.bind(pattern);
    }

    let total = count_q.fetch_one(db).await?;
    let interns = select_q.fetch_all(db).await?;

    Ok(ListInternsResponse {
        interns: interns.into_iter().map(map_intern_to_response).collect(),
        total,
        page,
        page_size,
    })
}

pub async fn update_intern(db: &Db, id: Uuid, req: UpdateInternRequest) -> Result<InternResponse> {
    let stipend = req.stipend.map(|s| BigDecimal::from_str(&s.to_string()).unwrap());

    let intern = sqlx::query_as::<_, InternWithPerson>(
        r#"
        UPDATE interns i
        SET department = COALESCE($2, i.department),
            supervisor_id = COALESCE($3, i.supervisor_id),
            end_date = COALESCE($4, i.end_date),
            stipend = COALESCE($5, i.stipend),
            university = COALESCE($6, i.university),
            status = COALESCE($7, i.status),
            updated_at = NOW()
        FROM persons p
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        WHERE i.id = $1 AND p.id = i.person_id
        RETURNING i.id, i.intern_id, i.person_id, 
                  p.first_name, p.middle_name, p.last_name,
                  pc.email, pc.phone,
                  i.department, i.supervisor_id, i.start_date,
                  i.end_date, i.stipend, i.university,
                  i.status, i.created_at, i.updated_at
        "#,
    )
    .bind(id)
    .bind(&req.department)
    .bind(req.supervisor_id)
    .bind(req.end_date)
    .bind(stipend)
    .bind(&req.university)
    .bind(&req.status)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Intern not found"))?;

    Ok(map_intern_to_response(intern))
}

pub async fn delete_intern(db: &Db, id: Uuid) -> Result<()> {
    let result = sqlx::query("UPDATE interns SET status = 'terminated', updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow!("Intern not found"));
    }

    Ok(())
}

fn map_intern_to_response(intern: InternWithPerson) -> InternResponse {
    InternResponse {
        id: intern.id,
        intern_id: intern.intern_id,
        person_id: intern.person_id,
        first_name: intern.first_name,
        middle_name: intern.middle_name,
        last_name: intern.last_name,
        email: intern.email,
        phone: intern.phone,
        department: intern.department_id.map(|id| id.to_string()),
        supervisor_id: intern.supervisor_id,
        start_date: intern.start_date,
        end_date: intern.end_date,
        stipend: intern.stipend.and_then(|s| s.to_string().parse().ok()),
        university: intern.university,
        status: intern.status,
    }
}
