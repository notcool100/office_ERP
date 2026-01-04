use crate::{
    api::attendance::dto::{
        AttendanceResponse, AttendanceSummary, CheckInRequest, CheckOutRequest,
        ListAttendanceQuery, ListAttendanceResponse,
    },
    db::Db,
    models::attendance::AttendanceWithEmployee,
};
use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDate, Utc};
use sqlx::types::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

pub async fn check_in(db: &Db, req: CheckInRequest) -> Result<AttendanceResponse> {
    let today = Local::now().date_naive();

    // Lookup employee UUID from string code
    let employee_uuid = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM employees WHERE employee_id = $1"
    )
    .bind(&req.employee_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Employee not found with ID: {}", req.employee_id))?;

    // Check if already checked in today
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM attendance_records WHERE employee_id = $1 AND date = $2)"
    )
    .bind(employee_uuid)
    .bind(today)
    .fetch_one(db)
    .await?;

    if exists {
        return Err(anyhow!("Already checked in today"));
    }

    let attendance = sqlx::query_as::<_, AttendanceWithEmployee>(
        r#"
        WITH new_attendance AS (
            INSERT INTO attendance_records (employee_id, date, check_in, status, notes, created_at, updated_at)
            VALUES ($1, $2, NOW(), 'present', $3, NOW(), NOW())
            RETURNING *
        )
        SELECT na.id, na.employee_id,
               CONCAT(p.first_name, ' ', p.last_name) as employee_name,
               na.date, na.check_in, na.check_out, na.total_hours,
               na.status, na.notes, na.created_at, na.updated_at
        FROM new_attendance na
        JOIN employees e ON e.id = na.employee_id
        JOIN persons p ON p.id = e.person_id
        "#,
    )
    .bind(employee_uuid)
    .bind(today)
    .bind(&req.notes)
    .fetch_one(db)
    .await?;

    Ok(map_attendance_to_response(attendance))
}

pub async fn check_out(db: &Db, employee_id: String, req: CheckOutRequest) -> Result<AttendanceResponse> {
    let today = Local::now().date_naive();

    // Lookup employee UUID from string code
    let employee_uuid = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM employees WHERE employee_id = $1"
    )
    .bind(&employee_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Employee not found with ID: {}", employee_id))?;

    let attendance = sqlx::query_as::<_, AttendanceWithEmployee>(
        r#"
        UPDATE attendance_records ar
        SET check_out = NOW(),
            total_hours = EXTRACT(EPOCH FROM (NOW() - ar.check_in)) / 3600,
            notes = COALESCE($3, ar.notes),
            updated_at = NOW()
        FROM employees e, persons p
        WHERE ar.employee_id = $1 
          AND ar.date = $2 
          AND ar.check_out IS NULL
          AND e.id = ar.employee_id
          AND p.id = e.person_id
        RETURNING ar.id, ar.employee_id,
                  CONCAT(p.first_name, ' ', p.last_name) as employee_name,
                  ar.date, ar.check_in, ar.check_out, ar.total_hours,
                  ar.status, ar.notes, ar.created_at, ar.updated_at
        "#,
    )
    .bind(employee_uuid)
    .bind(today)
    .bind(&req.notes)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("No active check-in found for today"))?;

    Ok(map_attendance_to_response(attendance))
}

pub async fn get_attendance_records(
    db: &Db,
    query: ListAttendanceQuery,
) -> Result<ListAttendanceResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).min(100);
    let offset = (page - 1) * page_size;

    let mut where_clauses: Vec<String> = vec!["1=1".to_string()];
    let mut param_count = 3;

    if query.employee_id.is_some() {
        where_clauses.push(format!("ar.employee_id = ${}", param_count));
        param_count += 1;
    }

    if query.start_date.is_some() {
        where_clauses.push(format!("ar.date >= ${}", param_count));
        param_count += 1;
    }

    if query.end_date.is_some() {
        where_clauses.push(format!("ar.date <= ${}", param_count));
        param_count += 1;
    }

    if query.status.is_some() {
        where_clauses.push(format!("ar.status = ${}", param_count));
    }

    let where_sql = where_clauses.join(" AND ");

    let count_query = format!("SELECT COUNT(*) FROM attendance_records ar WHERE {}", where_sql);
    let select_query = format!(
        r#"
        SELECT ar.id, ar.employee_id,
               CONCAT(p.first_name, ' ', p.last_name) as employee_name,
               ar.date, ar.check_in, ar.check_out, ar.total_hours,
               ar.status, ar.notes, ar.created_at, ar.updated_at
        FROM attendance_records ar
        JOIN employees e ON e.id = ar.employee_id
        JOIN persons p ON p.id = e.person_id
        WHERE {}
        ORDER BY ar.date DESC, ar.created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        where_sql
    );

    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    let mut select_q = sqlx::query_as::<_, AttendanceWithEmployee>(&select_query)
        .bind(page_size)
        .bind(offset);

    if let Some(emp_id) = query.employee_id {
        count_q = count_q.bind(emp_id);
        select_q = select_q.bind(emp_id);
    }

    if let Some(start_date) = query.start_date {
        count_q = count_q.bind(start_date);
        select_q = select_q.bind(start_date);
    }

    if let Some(end_date) = query.end_date {
        count_q = count_q.bind(end_date);
        select_q = select_q.bind(end_date);
    }

    if let Some(status) = &query.status {
        count_q = count_q.bind(status);
        select_q = select_q.bind(status);
    }

    let total = count_q.fetch_one(db).await?;
    let records = select_q.fetch_all(db).await?;

    Ok(ListAttendanceResponse {
        records: records.into_iter().map(map_attendance_to_response).collect(),
        total,
        page,
        page_size,
    })
}

pub async fn get_attendance_summary(
    db: &Db,
    employee_id: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<AttendanceSummary> {
    // Lookup employee UUID from string code
    let employee_uuid = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM employees WHERE employee_id = $1"
    )
    .bind(&employee_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Employee not found with ID: {}", employee_id))?;
    let summary = sqlx::query_as::<_, (String, i64, i64, i64, i64, Option<BigDecimal>)>(
        r#"
        SELECT 
            CONCAT(p.first_name, ' ', p.last_name) as employee_name,
            COUNT(*) as total_days,
            COUNT(*) FILTER (WHERE ar.status = 'present') as present_days,
            COUNT(*) FILTER (WHERE ar.status = 'late') as late_days,
            COUNT(*) FILTER (WHERE ar.status = 'absent') as absent_days,
            COALESCE(SUM(ar.total_hours), 0) as total_hours
        FROM attendance_records ar
        JOIN employees e ON e.id = ar.employee_id
        JOIN persons p ON p.id = e.person_id
        WHERE ar.employee_id = $1
          AND ar.date >= $2
          AND ar.date <= $3
        GROUP BY p.first_name, p.last_name
        "#,
    )
    .bind(employee_uuid)
    .bind(start_date)
    .bind(end_date)
    .fetch_optional(db)
    .await?;

    if let Some((name, total, present, late, absent, hours)) = summary {
        Ok(AttendanceSummary {
            employee_id: employee_uuid,
            employee_name: name,
            total_days: total,
            present_days: present,
            late_days: late,
            absent_days: absent,
            total_hours: hours.and_then(|h| h.to_string().parse().ok()).unwrap_or(0.0),
        })
    } else {
        Err(anyhow!("No attendance records found"))
    }
}

fn map_attendance_to_response(att: AttendanceWithEmployee) -> AttendanceResponse {
    AttendanceResponse {
        id: att.id,
        employee_id: att.employee_id,
        employee_name: att.employee_name,
        date: att.date,
        check_in: att.check_in.map(|dt| dt.and_utc()),
        check_out: att.check_out.map(|dt| dt.and_utc()),
        total_hours: att.total_hours.and_then(|h| h.to_string().parse().ok()),
        status: att.status,
        notes: att.notes,
    }
}
