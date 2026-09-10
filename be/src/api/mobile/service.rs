use anyhow::{Result, anyhow};
use chrono::{Datelike, FixedOffset, NaiveDate, NaiveTime, Utc};
use sqlx::types::BigDecimal;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    api::{
        attendance::{
            self,
            dto::{
                AttendanceResponse, CheckInRequest, CheckOutRequest, ListAttendanceQuery,
                ListAttendanceResponse,
            },
        },
        company_settings::service as company_settings_service,
        documents::dto::DocCategory,
        leave::{
            self,
            dto::{
                ApproveRejectLeaveRequest, CreateLeaveRequestRequest, LeaveBalanceResponse,
                LeaveRequestResponse, LeaveTypeResponse, ListLeaveRequestsQuery,
            },
        },
        meetings::{self, dto::MeetingResponse},
        notifications::{
            dto::{NewNotification, kind},
            service as notifications_service,
        },
    },
    db::Db,
    middlewares::rbac,
    models::user::User,
    ws::hub::Hub,
};

use super::dto::{
    BootstrapResponse, ClientOption, DashboardEvent, DashboardResponse, DateRangeQuery,
    DirectoryEntry, LeaveDecisionRequest, MobileCapabilities, MobileCheckInRequest,
    MobileCheckOutRequest, MobileCreateLeaveRequest, MobileEmployee, MobileOffice, MobilePerson,
    MobileUser, MonthSummary, ScheduleDay, UpdateProfileRequest,
};

/// Kept in step with `attendance::service::MAX_GEOFENCE_METERS`, which is
/// private. Only ever sent to the client as a display hint — the server's
/// own constant is what actually rejects an out-of-range check-in.
const GEOFENCE_METERS: f64 = 300.0;

const COMPANY_DOCS_NAV: &str = "/admin/documents/company";
const CLIENT_DOCS_NAV: &str = "/admin/documents/client";

/// Attendance is recorded against the Nepal working day, so "today" on the
/// dashboard has to be resolved in the same offset or a late-evening
/// check-in would land on the wrong card.
fn nepal_offset() -> FixedOffset {
    FixedOffset::east_opt(5 * 3600 + 45 * 60).expect("Nepal UTC offset should be valid")
}

pub fn today_in_nepal() -> NaiveDate {
    Utc::now().with_timezone(&nepal_offset()).date_naive()
}

/// The caller's employee row, resolved from the JWT. Everything
/// employee-scoped in this module goes through here, which is what keeps a
/// request from addressing anybody else's records.
pub struct CallerEmployee {
    pub id: Uuid,
    pub code: String,
}

pub async fn resolve_employee(db: &Db, user: &User) -> Result<CallerEmployee> {
    let row = sqlx::query_as::<_, (Uuid, String)>(
        "SELECT id, employee_id FROM employees WHERE person_id = $1 LIMIT 1",
    )
    .bind(user.person_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("No employee record is linked to this account"))?;

    Ok(CallerEmployee {
        id: row.0,
        code: row.1,
    })
}

// ---------------------------------------------------------------- bootstrap

pub async fn bootstrap(db: &Db, user: &User) -> Result<BootstrapResponse> {
    let person = sqlx::query_as::<_, (String, Option<String>, String, Option<String>, Option<String>)>(
        r#"
        SELECT p.first_name, p.middle_name, p.last_name, pc.email, pc.phone
        FROM persons p
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        WHERE p.id = $1
        "#,
    )
    .bind(user.person_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Profile not found"))?;

    let (first_name, middle_name, last_name, email, phone) = person;
    let full_name = format!("{} {}", first_name, last_name);
    let initials = initials_of(&first_name, &last_name);

    let employee = sqlx::query_as::<
        _,
        (
            Uuid,
            String,
            Option<String>,
            Option<String>,
            NaiveDate,
            Option<String>,
            String,
            Option<Uuid>,
            Option<String>,
        ),
    >(
        r#"
        SELECT e.id, e.employee_id, d.name, pos.name, e.hire_date,
               e.employment_type, e.status, e.manager_id,
               CASE WHEN mp.id IS NULL THEN NULL
                    ELSE CONCAT(mp.first_name, ' ', mp.last_name) END
        FROM employees e
        LEFT JOIN departments d ON d.id = e.department_id
        LEFT JOIN positions pos ON pos.id = e.position_id
        LEFT JOIN employees m ON m.id = e.manager_id
        LEFT JOIN persons mp ON mp.id = m.person_id
        WHERE e.person_id = $1
        LIMIT 1
        "#,
    )
    .bind(user.person_id)
    .fetch_optional(db)
    .await?
    .map(
        |(
            id,
            employee_code,
            department,
            position,
            hire_date,
            employment_type,
            status,
            manager_id,
            manager_name,
        )| MobileEmployee {
            id,
            employee_code,
            department,
            position,
            hire_date,
            employment_type,
            status,
            manager_id,
            manager_name,
        },
    );

    let leave_approvals = match &employee {
        Some(emp) => user.is_admin || has_direct_reports(db, emp.id).await?,
        None => user.is_admin,
    };

    let settings = company_settings_service::get_settings(db).await.ok();

    Ok(BootstrapResponse {
        user: MobileUser {
            id: user.id,
            user_name: user.user_name.clone(),
            email: user.email.clone(),
            phone: user.phone.clone(),
            is_admin: user.is_admin,
        },
        person: MobilePerson {
            id: user.person_id,
            first_name,
            middle_name,
            last_name,
            full_name,
            initials,
            email,
            phone,
        },
        employee,
        capabilities: MobileCapabilities {
            company_documents: rbac::user_can_read(db, user, COMPANY_DOCS_NAV).await,
            client_documents: rbac::user_can_read(db, user, CLIENT_DOCS_NAV).await,
            leave_approvals,
        },
        office: MobileOffice {
            name: settings
                .as_ref()
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "Office".to_string()),
            latitude: settings.as_ref().and_then(|s| s.latitude),
            longitude: settings.as_ref().and_then(|s| s.longitude),
            location_name: settings.as_ref().and_then(|s| s.location_name.clone()),
            geofence_meters: GEOFENCE_METERS,
        },
    })
}

fn initials_of(first: &str, last: &str) -> String {
    let take = |s: &str| s.chars().next().map(|c| c.to_ascii_uppercase());
    match (take(first), take(last)) {
        (Some(a), Some(b)) => format!("{}{}", a, b),
        (Some(a), None) | (None, Some(a)) => a.to_string(),
        _ => "?".to_string(),
    }
}

async fn has_direct_reports(db: &Db, employee_id: Uuid) -> Result<bool> {
    let exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM employees WHERE manager_id = $1)",
    )
    .bind(employee_id)
    .fetch_one(db)
    .await?;
    Ok(exists)
}

// ---------------------------------------------------------------- dashboard

pub async fn dashboard(db: &Db, user: &User) -> Result<DashboardResponse> {
    let today = today_in_nepal();
    let employee = resolve_employee(db, user).await.ok();

    let greeting_name = sqlx::query_scalar::<_, String>("SELECT first_name FROM persons WHERE id = $1")
        .bind(user.person_id)
        .fetch_optional(db)
        .await?
        .unwrap_or_else(|| user.user_name.clone());

    let (attendance, month_summary) = match &employee {
        Some(emp) => (
            today_attendance(db, emp.id, today).await?,
            month_summary(db, emp.id, today).await?,
        ),
        None => (None, MonthSummary::default()),
    };

    let events = todays_events(db, user, today).await?;

    let meetings = meetings::service::list_my_meetings(db, user.id)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|m| m.status == "active" || m.created_at.with_timezone(&nepal_offset()).date_naive() == today)
        .collect::<Vec<MeetingResponse>>();

    let pending_approvals = match &employee {
        Some(emp) => pending_approvals(db, user, emp.id).await?,
        None if user.is_admin => admin_pending_approvals(db).await?,
        None => Vec::new(),
    };

    let unread_notifications = notifications_service::unread_count(db, user.id).await?;

    Ok(DashboardResponse {
        date: today,
        greeting_name,
        attendance,
        month_summary,
        events,
        meetings,
        pending_approvals,
        unread_notifications,
    })
}

impl Default for MonthSummary {
    fn default() -> Self {
        Self {
            present_days: 0,
            late_days: 0,
            absent_days: 0,
            leave_days: 0.0,
            total_hours: 0.0,
        }
    }
}

pub async fn today_attendance(
    db: &Db,
    employee_id: Uuid,
    date: NaiveDate,
) -> Result<Option<AttendanceResponse>> {
    let records = attendance::service::get_attendance_records(
        db,
        ListAttendanceQuery {
            page: Some(1),
            page_size: Some(1),
            employee_id: Some(employee_id),
            start_date: Some(date),
            end_date: Some(date),
            status: None,
        },
    )
    .await?;

    Ok(records.records.into_iter().next())
}

async fn month_summary(db: &Db, employee_id: Uuid, today: NaiveDate) -> Result<MonthSummary> {
    let month_start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
        .ok_or_else(|| anyhow!("Invalid month boundary"))?;

    let row = sqlx::query_as::<_, (i64, i64, i64, Option<BigDecimal>)>(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE status = 'present') AS present_days,
            COUNT(*) FILTER (WHERE status = 'late') AS late_days,
            COUNT(*) FILTER (WHERE status = 'absent') AS absent_days,
            COALESCE(SUM(total_hours), 0) AS total_hours
        FROM attendance_records
        WHERE employee_id = $1 AND date BETWEEN $2 AND $3
        "#,
    )
    .bind(employee_id)
    .bind(month_start)
    .bind(today)
    .fetch_one(db)
    .await?;

    // Approved leave overlapping the month, clipped to the month window so
    // a leave that straddles the boundary isn't counted twice.
    let leave_days = sqlx::query_scalar::<_, Option<BigDecimal>>(
        r#"
        SELECT COALESCE(SUM(
            LEAST(end_date, $3) - GREATEST(start_date, $2) + 1
        ), 0)::numeric
        FROM leave_requests
        WHERE employee_id = $1
          AND status = 'approved'
          AND start_date <= $3
          AND end_date >= $2
        "#,
    )
    .bind(employee_id)
    .bind(month_start)
    .bind(today)
    .fetch_one(db)
    .await?;

    Ok(MonthSummary {
        present_days: row.0,
        late_days: row.1,
        absent_days: row.2,
        leave_days: decimal_to_f64(leave_days),
        total_hours: decimal_to_f64(row.3),
    })
}

fn decimal_to_f64(value: Option<BigDecimal>) -> f64 {
    value
        .and_then(|d| d.to_string().parse::<f64>().ok())
        .unwrap_or(0.0)
}

async fn todays_events(db: &Db, user: &User, today: NaiveDate) -> Result<Vec<DashboardEvent>> {
    let events = crate::api::calendar::service::list_events(
        db,
        user,
        crate::api::calendar::dto::ListCalendarEventsQuery {
            start_date: Some(today),
            end_date: Some(today),
        },
    )
    .await
    .map_err(|e| anyhow!("{}", e))?;

    Ok(events
        .into_iter()
        .map(|e| DashboardEvent {
            id: e.id,
            title: e.title,
            location: e.location,
            start_at: e.start_at,
            end_at: e.end_at,
            all_day: e.all_day,
            scope: e.scope,
        })
        .collect())
}

// -------------------------------------------------------------- attendance

pub async fn check_in(
    db: &Db,
    user: &User,
    req: MobileCheckInRequest,
) -> Result<AttendanceResponse> {
    let employee = resolve_employee(db, user).await?;

    // Always FACE: that is the branch of the attendance service that
    // enforces the office geofence, and a phone check-in is exactly the
    // case it exists for. A mobile client must never be able to opt out
    // by choosing the method itself, so it isn't part of the request DTO.
    attendance::service::check_in(
        db,
        CheckInRequest {
            employee_id: employee.code,
            notes: req.notes,
            image: req.image,
            latitude: req.latitude,
            longitude: req.longitude,
            method: Some("FACE".to_string()),
        },
    )
    .await
}

pub async fn check_out(
    db: &Db,
    user: &User,
    req: MobileCheckOutRequest,
) -> Result<AttendanceResponse> {
    let employee = resolve_employee(db, user).await?;

    attendance::service::check_out(
        db,
        employee.code,
        CheckOutRequest {
            notes: req.notes,
            latitude: req.latitude,
            longitude: req.longitude,
            method: Some("FACE".to_string()),
        },
    )
    .await
}

pub async fn attendance_records(
    db: &Db,
    user: &User,
    range: DateRangeQuery,
) -> Result<ListAttendanceResponse> {
    let employee = resolve_employee(db, user).await?;
    let today = today_in_nepal();
    let start = range
        .start_date
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap_or(today));

    attendance::service::get_attendance_records(
        db,
        ListAttendanceQuery {
            page: Some(1),
            page_size: Some(100),
            // Forced to the caller's own employee id: the query struct
            // accepts an arbitrary one, and this is the only place that
            // decides which id is used.
            employee_id: Some(employee.id),
            start_date: Some(start),
            end_date: Some(range.end_date.unwrap_or(today)),
            status: None,
        },
    )
    .await
}

// ------------------------------------------------------------------- leave

pub async fn leave_types(db: &Db) -> Result<Vec<LeaveTypeResponse>> {
    leave::service::get_leave_types(db).await
}

pub async fn leave_balance(db: &Db, user: &User) -> Result<Vec<LeaveBalanceResponse>> {
    let employee = resolve_employee(db, user).await?;
    leave::service::get_leave_balance(db, employee.id).await
}

pub async fn my_leave_requests(db: &Db, user: &User) -> Result<Vec<LeaveRequestResponse>> {
    let employee = resolve_employee(db, user).await?;

    let response = leave::service::list_leave_requests(
        db,
        ListLeaveRequestsQuery {
            page: Some(1),
            page_size: Some(100),
            employee_id: Some(employee.id),
            status: None,
            start_date: None,
            end_date: None,
        },
    )
    .await?;

    Ok(response.requests)
}

pub async fn create_leave_request(
    db: &Db,
    hub: Option<&Arc<Hub>>,
    user: &User,
    req: MobileCreateLeaveRequest,
) -> Result<LeaveRequestResponse> {
    let employee = resolve_employee(db, user).await?;

    if req.end_date < req.start_date {
        return Err(anyhow!("The end date cannot be before the start date"));
    }

    let created = leave::service::create_leave_request(
        db,
        CreateLeaveRequestRequest {
            employee_id: Some(employee.id),
            leave_type_id: req.leave_type_id,
            start_date: req.start_date,
            end_date: req.end_date,
            reason: req.reason,
        },
    )
    .await?;

    // Tell the approver there's something waiting, rather than making them
    // discover it by opening the app.
    if let Some(manager_user_id) = manager_user_id(db, employee.id).await? {
        let _ = notifications_service::notify(
            db,
            hub,
            manager_user_id,
            NewNotification::new(kind::LEAVE, format!("{} requested leave", created.employee_name))
                .body(format!(
                    "{} · {} to {}",
                    created.leave_type_name, created.start_date, created.end_date
                ))
                .entity("leave_request", created.id),
        )
        .await;
    }

    Ok(created)
}

async fn manager_user_id(db: &Db, employee_id: Uuid) -> Result<Option<Uuid>> {
    let id = sqlx::query_scalar::<_, Uuid>(
        r#"
        SELECT u.id
        FROM employees e
        JOIN employees m ON m.id = e.manager_id
        JOIN users u ON u.person_id = m.person_id
        WHERE e.id = $1
        "#,
    )
    .bind(employee_id)
    .fetch_optional(db)
    .await?;
    Ok(id)
}

/// Leave requests the caller is responsible for: those raised by their
/// direct reports. Admins see everything pending, matching the web console.
pub async fn pending_approvals(
    db: &Db,
    user: &User,
    employee_id: Uuid,
) -> Result<Vec<LeaveRequestResponse>> {
    if user.is_admin {
        return admin_pending_approvals(db).await;
    }

    let rows = sqlx::query_as::<_, LeaveApprovalRow>(
        r#"
        SELECT lr.id, lr.employee_id,
               CONCAT(p.first_name, ' ', p.last_name) AS employee_name,
               lr.leave_type_id, lt.name AS leave_type_name,
               lr.start_date, lr.end_date, lr.total_days, lr.reason,
               lr.status, lr.approved_by, lr.notes
        FROM leave_requests lr
        JOIN employees e ON e.id = lr.employee_id
        JOIN persons p ON p.id = e.person_id
        JOIN leave_types lt ON lt.id = lr.leave_type_id
        WHERE e.manager_id = $1 AND lr.status = 'pending'
        ORDER BY lr.created_at DESC
        "#,
    )
    .bind(employee_id)
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(Into::into).collect())
}

async fn admin_pending_approvals(db: &Db) -> Result<Vec<LeaveRequestResponse>> {
    let response = leave::service::list_leave_requests(
        db,
        ListLeaveRequestsQuery {
            page: Some(1),
            page_size: Some(50),
            employee_id: None,
            status: Some("pending".to_string()),
            start_date: None,
            end_date: None,
        },
    )
    .await?;
    Ok(response.requests)
}

#[derive(sqlx::FromRow)]
struct LeaveApprovalRow {
    id: Uuid,
    employee_id: Uuid,
    employee_name: String,
    leave_type_id: Uuid,
    leave_type_name: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    total_days: BigDecimal,
    reason: Option<String>,
    status: String,
    approved_by: Option<Uuid>,
    notes: Option<String>,
}

impl From<LeaveApprovalRow> for LeaveRequestResponse {
    fn from(row: LeaveApprovalRow) -> Self {
        LeaveRequestResponse {
            id: row.id,
            employee_id: row.employee_id,
            employee_name: row.employee_name,
            leave_type_id: row.leave_type_id,
            leave_type_name: row.leave_type_name,
            start_date: row.start_date,
            end_date: row.end_date,
            total_days: row.total_days.to_string().parse().unwrap_or(0.0),
            reason: row.reason,
            status: row.status,
            approved_by: row.approved_by,
            approver_name: None,
            notes: row.notes,
        }
    }
}

/// True when `user` may decide `request_id` — they're an admin, or the
/// requester reports to them. Checked here rather than relying on a nav
/// permission, because "can approve leave" is a fact about the org chart.
async fn can_decide_leave(db: &Db, user: &User, request_id: Uuid) -> Result<bool> {
    if user.is_admin {
        return Ok(true);
    }

    let allowed = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM leave_requests lr
            JOIN employees e ON e.id = lr.employee_id
            JOIN employees m ON m.id = e.manager_id
            WHERE lr.id = $1 AND m.person_id = $2
        )
        "#,
    )
    .bind(request_id)
    .bind(user.person_id)
    .fetch_one(db)
    .await?;

    Ok(allowed)
}

pub enum LeaveDecision {
    Approve,
    Reject,
}

pub async fn decide_leave(
    db: &Db,
    hub: Option<&Arc<Hub>>,
    user: &User,
    request_id: Uuid,
    decision: LeaveDecision,
    payload: LeaveDecisionRequest,
) -> Result<LeaveRequestResponse> {
    if !can_decide_leave(db, user, request_id).await? {
        return Err(anyhow!("You cannot decide this leave request"));
    }

    let req = ApproveRejectLeaveRequest {
        notes: payload.notes,
    };

    let updated = match decision {
        LeaveDecision::Approve => leave::service::approve_leave(db, request_id, user.id, req).await?,
        LeaveDecision::Reject => leave::service::reject_leave(db, request_id, user.id, req).await?,
    };

    if let Some(requester_user_id) = employee_user_id(db, updated.employee_id).await? {
        let verdict = match decision {
            LeaveDecision::Approve => "approved",
            LeaveDecision::Reject => "rejected",
        };
        let _ = notifications_service::notify(
            db,
            hub,
            requester_user_id,
            NewNotification::new(
                kind::LEAVE,
                format!("Your leave request was {}", verdict),
            )
            .body(format!(
                "{} · {} to {}",
                updated.leave_type_name, updated.start_date, updated.end_date
            ))
            .entity("leave_request", updated.id),
        )
        .await;
    }

    Ok(updated)
}

async fn employee_user_id(db: &Db, employee_id: Uuid) -> Result<Option<Uuid>> {
    let id = sqlx::query_scalar::<_, Uuid>(
        "SELECT u.id FROM employees e JOIN users u ON u.person_id = e.person_id WHERE e.id = $1",
    )
    .bind(employee_id)
    .fetch_optional(db)
    .await?;
    Ok(id)
}

// ---------------------------------------------------------------- schedule

/// Nepal's working week: Sunday through Friday, with Saturday off.
const DEFAULT_START: (u32, u32) = (9, 30);
const DEFAULT_END: (u32, u32) = (18, 30);
const DEFAULT_OFF_DAY: i16 = 6; // Saturday, ISO-8601

pub async fn schedule(db: &Db, user: &User) -> Result<Vec<ScheduleDay>> {
    let employee = resolve_employee(db, user).await?;

    let overrides = sqlx::query_as::<_, (i16, Option<NaiveTime>, Option<NaiveTime>, bool)>(
        "SELECT day_of_week, start_time, end_time, is_working \
         FROM employee_schedules WHERE employee_id = $1",
    )
    .bind(employee.id)
    .fetch_all(db)
    .await?;

    let default_start = NaiveTime::from_hms_opt(DEFAULT_START.0, DEFAULT_START.1, 0);
    let default_end = NaiveTime::from_hms_opt(DEFAULT_END.0, DEFAULT_END.1, 0);

    Ok((1..=7i16)
        .map(|day| {
            let found = overrides.iter().find(|(d, _, _, _)| *d == day);
            match found {
                Some((_, start, end, is_working)) => ScheduleDay {
                    day_of_week: day,
                    day_name: day_name(day).to_string(),
                    is_working: *is_working,
                    start_time: *start,
                    end_time: *end,
                },
                None => {
                    let is_working = day != DEFAULT_OFF_DAY;
                    ScheduleDay {
                        day_of_week: day,
                        day_name: day_name(day).to_string(),
                        is_working,
                        start_time: is_working.then_some(default_start).flatten(),
                        end_time: is_working.then_some(default_end).flatten(),
                    }
                }
            }
        })
        .collect())
}

fn day_name(day: i16) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "Sunday",
    }
}

// ----------------------------------------------------------------- profile

/// Only the contact fields are editable from the phone. Name, department
/// and employment details stay an HR-controlled surface.
pub async fn update_profile(db: &Db, user: &User, req: UpdateProfileRequest) -> Result<()> {
    let email = req.email.map(|e| e.trim().to_string()).filter(|e| !e.is_empty());
    let phone = req.phone.map(|p| p.trim().to_string()).filter(|p| !p.is_empty());

    if email.is_none() && phone.is_none() {
        return Ok(());
    }

    let mut tx = db.begin().await?;

    sqlx::query(
        "UPDATE person_contacts SET email = COALESCE($1, email), phone = COALESCE($2, phone) \
         WHERE person_id = $3",
    )
    .bind(&email)
    .bind(&phone)
    .bind(user.person_id)
    .execute(&mut *tx)
    .await?;

    // users.email / users.phone are what login and password reset use, so
    // they have to move together with the contact row.
    sqlx::query("UPDATE users SET email = COALESCE($1, email), phone = COALESCE($2, phone) WHERE id = $3")
        .bind(&email)
        .bind(&phone)
        .bind(user.id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

// --------------------------------------------------------------- directory

pub async fn directory(db: &Db, user: &User, search: Option<String>) -> Result<Vec<DirectoryEntry>> {
    let pattern = search
        .map(|s| format!("%{}%", s.trim().to_lowercase()))
        .unwrap_or_else(|| "%".to_string());

    let entries = sqlx::query_as::<_, DirectoryEntry>(
        r#"
        SELECT u.id AS user_id,
               CONCAT(p.first_name, ' ', p.last_name) AS display_name,
               u.email,
               d.name AS department,
               pos.name AS position
        FROM users u
        JOIN persons p ON p.id = u.person_id
        LEFT JOIN employees e ON e.person_id = p.id
        LEFT JOIN departments d ON d.id = e.department_id
        LEFT JOIN positions pos ON pos.id = e.position_id
        WHERE u.id <> $1
          AND LOWER(CONCAT(p.first_name, ' ', p.last_name)) LIKE $2
        ORDER BY p.first_name, p.last_name
        LIMIT 200
        "#,
    )
    .bind(user.id)
    .bind(pattern)
    .fetch_all(db)
    .await?;

    Ok(entries)
}

// --------------------------------------------------------------- documents

pub fn parse_category(raw: Option<&str>) -> DocCategory {
    match raw {
        Some("client") => DocCategory::Client,
        _ => DocCategory::Company,
    }
}

pub fn nav_path_for(category: DocCategory) -> &'static str {
    match category {
        DocCategory::Client => CLIENT_DOCS_NAV,
        DocCategory::Employee => "/admin/hr/employee",
        DocCategory::Company => COMPANY_DOCS_NAV,
    }
}

/// Active clients, for the "pick a client" step the app shows before
/// browsing Client Documents — a client has to be chosen before `owner_id`
/// means anything to `documents_service::list_location`.
pub async fn list_document_clients(db: &Db) -> Result<Vec<ClientOption>> {
    let clients = sqlx::query_as::<_, ClientOption>(
        "SELECT id, name FROM clients WHERE status = 'active' ORDER BY name",
    )
    .fetch_all(db)
    .await?;
    Ok(clients)
}
