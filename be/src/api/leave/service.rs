use crate::{
    api::leave::dto::{
        ApproveRejectLeaveRequest, CreateLeaveRequestRequest, LeaveBalanceResponse,
        LeaveRequestResponse, LeaveTypeResponse, ListLeaveRequestsQuery,
        ListLeaveRequestsResponse,
    },
    db::Db,
    models::leave::{LeaveRequestWithDetails, LeaveType},
};
use anyhow::{anyhow, Result};

use sqlx::types::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

pub async fn create_leave_request(
    db: &Db,
    req: CreateLeaveRequestRequest,
) -> Result<LeaveRequestResponse> {
    // Validate dates
    if req.end_date < req.start_date {
        return Err(anyhow!("End date must be after start date"));
    }

    // Calculate total days
    let total_days = (req.end_date - req.start_date).num_days() as f64 + 1.0;

    let leave_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO leave_requests (employee_id, leave_type_id, start_date, end_date, 
                                    total_days, reason, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 'pending', NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(req.employee_id)
    .bind(req.leave_type_id)
    .bind(req.start_date)
    .bind(req.end_date)
    .bind(BigDecimal::from_str(&total_days.to_string()).unwrap())
    .bind(&req.reason)
    .fetch_one(db)
    .await?;

    // Fetch the complete leave request with details
    get_leave_request(db, leave_id).await
}

pub async fn get_leave_request(db: &Db, id: Uuid) -> Result<LeaveRequestResponse> {
    let leave_request = sqlx::query_as::<_, LeaveRequestWithDetails>(
        r#"
        SELECT lr.id, lr.employee_id,
               CONCAT(p.first_name, ' ', p.last_name) as employee_name,
               lr.leave_type_id, lt.name as leave_type_name,
               lr.start_date, lr.end_date, lr.total_days, lr.reason,
               lr.status, lr.approved_by,
               CONCAT(ap.first_name, ' ', ap.last_name) as approver_name,
               lr.approved_at, lr.notes, lr.created_at, lr.updated_at
        FROM leave_requests lr
        JOIN employees e ON e.id = lr.employee_id
        JOIN persons p ON p.id = e.person_id
        JOIN leave_types lt ON lt.id = lr.leave_type_id
        LEFT JOIN users u ON u.id = lr.approved_by
        LEFT JOIN persons ap ON ap.id = u.person_id
        WHERE lr.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Leave request not found"))?;

    Ok(map_leave_request_to_response(leave_request))
}

pub async fn list_leave_requests(
    db: &Db,
    query: ListLeaveRequestsQuery,
) -> Result<ListLeaveRequestsResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).min(100);
    let offset = (page - 1) * page_size;

    let mut where_clauses: Vec<String> = vec!["1=1".to_string()];
    let mut param_index = 1;

    if query.employee_id.is_some() {
        where_clauses.push(format!("lr.employee_id = ${}", param_index));
        param_index += 1;
    }

    if query.status.is_some() {
        where_clauses.push(format!("lr.status = ${}", param_index));
        param_index += 1;
    }

    if query.start_date.is_some() {
        where_clauses.push(format!("lr.start_date >= ${}", param_index));
        param_index += 1;
    }

    if query.end_date.is_some() {
        where_clauses.push(format!("lr.end_date <= ${}", param_index));
        param_index += 1;
    }

    let where_sql = where_clauses.join(" AND ");

    let count_query = format!("SELECT COUNT(*) FROM leave_requests lr WHERE {}", where_sql);
    let select_query = format!(
        r#"
        SELECT lr.id, lr.employee_id,
               CONCAT(p.first_name, ' ', p.last_name) as employee_name,
               lr.leave_type_id, lt.name as leave_type_name,
               lr.start_date, lr.end_date, lr.total_days, lr.reason,
               lr.status, lr.approved_by,
               CONCAT(ap.first_name, ' ', ap.last_name) as approver_name,
               lr.approved_at, lr.notes, lr.created_at, lr.updated_at
        FROM leave_requests lr
        JOIN employees e ON e.id = lr.employee_id
        JOIN persons p ON p.id = e.person_id
        JOIN leave_types lt ON lt.id = lr.leave_type_id
        LEFT JOIN users u ON u.id = lr.approved_by
        LEFT JOIN persons ap ON ap.id = u.person_id
        WHERE {}
        ORDER BY lr.created_at DESC
        LIMIT ${} OFFSET ${}
        "#,
        where_sql, param_index, param_index + 1
    );

    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    let mut select_q = sqlx::query_as::<_, LeaveRequestWithDetails>(&select_query);

    if let Some(emp_id) = query.employee_id {
        count_q = count_q.bind(emp_id);
        select_q = select_q.bind(emp_id);
    }

    if let Some(status) = &query.status {
        count_q = count_q.bind(status);
        select_q = select_q.bind(status);
    }

    if let Some(start_date) = query.start_date {
        count_q = count_q.bind(start_date);
        select_q = select_q.bind(start_date);
    }

    if let Some(end_date) = query.end_date {
        count_q = count_q.bind(end_date);
        select_q = select_q.bind(end_date);
    }

    // Bind limit/offset only to select_q
    select_q = select_q.bind(page_size).bind(offset);

    let total = count_q.fetch_one(db).await?;
    let requests = select_q.fetch_all(db).await?;

    Ok(ListLeaveRequestsResponse {
        requests: requests.into_iter().map(map_leave_request_to_response).collect(),
        total,
        page,
        page_size,
    })
}

pub async fn approve_leave(
    db: &Db,
    id: Uuid,
    approver_id: Uuid,
    req: ApproveRejectLeaveRequest,
) -> Result<LeaveRequestResponse> {
    let leave_request = sqlx::query_as::<_, LeaveRequestWithDetails>(
        r#"
        UPDATE leave_requests lr
        SET status = 'approved',
            approved_by = $2,
            approved_at = NOW(),
            notes = COALESCE($3, lr.notes),
            updated_at = NOW()
        FROM employees e, persons p, leave_types lt
        LEFT JOIN users u ON u.id = $2
        LEFT JOIN persons ap ON ap.id = u.person_id
        WHERE lr.id = $1 
          AND e.id = lr.employee_id 
          AND p.id = e.person_id
          AND lt.id = lr.leave_type_id
        RETURNING lr.id, lr.employee_id,
                  CONCAT(p.first_name, ' ', p.last_name) as employee_name,
                  lr.leave_type_id, lt.name as leave_type_name,
                  lr.start_date, lr.end_date, lr.total_days, lr.reason,
                  lr.status, lr.approved_by,
                  CONCAT(ap.first_name, ' ', ap.last_name) as approver_name,
                  lr.approved_at, lr.notes, lr.created_at, lr.updated_at
        "#,
    )
    .bind(id)
    .bind(approver_id)
    .bind(&req.notes)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Leave request not found"))?;

    Ok(map_leave_request_to_response(leave_request))
}

pub async fn reject_leave(
    db: &Db,
    id: Uuid,
    approver_id: Uuid,
    req: ApproveRejectLeaveRequest,
) -> Result<LeaveRequestResponse> {
    let leave_request = sqlx::query_as::<_, LeaveRequestWithDetails>(
        r#"
        UPDATE leave_requests lr
        SET status = 'rejected',
            approved_by = $2,
            approved_at = NOW(),
            notes = COALESCE($3, lr.notes),
            updated_at = NOW()
        FROM employees e, persons p, leave_types lt
        LEFT JOIN users u ON u.id = $2
        LEFT JOIN persons ap ON ap.id = u.person_id
        WHERE lr.id = $1 
          AND e.id = lr.employee_id 
          AND p.id = e.person_id
          AND lt.id = lr.leave_type_id
        RETURNING lr.id, lr.employee_id,
                  CONCAT(p.first_name, ' ', p.last_name) as employee_name,
                  lr.leave_type_id, lt.name as leave_type_name,
                  lr.start_date, lr.end_date, lr.total_days, lr.reason,
                  lr.status, lr.approved_by,
                  CONCAT(ap.first_name, ' ', ap.last_name) as approver_name,
                  lr.approved_at, lr.notes, lr.created_at, lr.updated_at
        "#,
    )
    .bind(id)
    .bind(approver_id)
    .bind(&req.notes)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Leave request not found"))?;

    Ok(map_leave_request_to_response(leave_request))
}

pub async fn get_leave_types(db: &Db) -> Result<Vec<LeaveTypeResponse>> {
    let leave_types = sqlx::query_as::<_, LeaveType>("SELECT * FROM leave_types ORDER BY name")
        .fetch_all(db)
        .await?;

    Ok(leave_types.into_iter().map(map_leave_type_to_response).collect())
}

pub async fn get_leave_balance(db: &Db, employee_id: Uuid) -> Result<Vec<LeaveBalanceResponse>> {
    let balances = sqlx::query_as::<_, (Uuid, Uuid, String, i32, Option<BigDecimal>)>(
        r#"
        SELECT 
            $1 as employee_id,
            lt.id as leave_type_id,
            lt.name as leave_type_name,
            lt.max_days_per_year as total_allowed,
            COALESCE(SUM(lr.total_days), 0) as used
        FROM leave_types lt
        LEFT JOIN leave_requests lr ON lr.leave_type_id = lt.id 
            AND lr.employee_id = $1 
            AND lr.status = 'approved'
            AND EXTRACT(YEAR FROM lr.start_date) = EXTRACT(YEAR FROM CURRENT_DATE)
        WHERE lt.max_days_per_year IS NOT NULL
        GROUP BY lt.id, lt.name, lt.max_days_per_year
        "#,
    )
    .bind(employee_id)
    .fetch_all(db)
    .await?;

    Ok(balances
        .into_iter()
        .map(|(emp_id, lt_id, lt_name, total, used)| {
            let used_days: f64 = used
                .and_then(|u| u.to_string().parse().ok())
                .unwrap_or(0.0);
            LeaveBalanceResponse {
                employee_id: emp_id,
                leave_type_id: lt_id,
                leave_type_name: lt_name,
                total_allowed: total,
                used: used_days,
                remaining: total as f64 - used_days,
            }
        })
        .collect())
}

fn map_leave_request_to_response(req: LeaveRequestWithDetails) -> LeaveRequestResponse {
    LeaveRequestResponse {
        id: req.id,
        employee_id: req.employee_id,
        employee_name: req.employee_name,
        leave_type_id: req.leave_type_id,
        leave_type_name: req.leave_type_name,
        start_date: req.start_date,
        end_date: req.end_date,
        total_days: req.total_days.to_string().parse().unwrap_or(0.0),
        reason: req.reason,
        status: req.status,
        approved_by: req.approved_by,
        approver_name: req.approver_name,
        notes: req.notes,
    }
}

fn map_leave_type_to_response(lt: LeaveType) -> LeaveTypeResponse {
    LeaveTypeResponse {
        id: lt.id,
        name: lt.name,
        description: lt.description,
        max_days_per_year: lt.max_days_per_year,
        requires_approval: lt.requires_approval,
        carry_forward: lt.carry_forward,
    }
}
