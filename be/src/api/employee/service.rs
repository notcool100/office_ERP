use crate::{
    api::employee::dto::{
        CreateEmployeeRequest, EmployeeResponse, ListEmployeesQuery, ListEmployeesResponse,
        UpdateEmployeeRequest,
    },
    db::Db,
    models::employee::EmployeeWithPerson,
};
use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::types::BigDecimal;
use std::str::FromStr;
use uuid::Uuid;

pub async fn create_employee(db: &Db, req: CreateEmployeeRequest) -> Result<EmployeeResponse> {
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

    // Check if employee_id is already taken
    let employee_id_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM employees WHERE employee_id = $1)"
    )
    .bind(&req.employee_id)
    .fetch_one(db)
    .await?;

    if employee_id_exists {
        return Err(anyhow!("Employee ID already exists"));
    }

    let salary = req.salary.map(|s| BigDecimal::from_str(&s.to_string()).unwrap());

    let employee = sqlx::query_as::<_, EmployeeWithPerson>(
        r#"
        WITH new_emp AS (
            INSERT INTO employees (employee_id, person_id, department_id, position_id, hire_date, 
                                   employment_type, salary, manager_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'active', NOW(), NOW())
            RETURNING *
        )
        SELECT e.id, e.employee_id, e.person_id, 
               p.first_name, p.middle_name, p.last_name,
               pc.email, pc.phone,
               e.department_id, e.position_id, e.hire_date,
               e.employment_type, e.salary, e.manager_id,
               e.status, e.created_at, e.updated_at
        FROM new_emp e
        JOIN persons p ON p.id = e.person_id
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        "#,
    )
    .bind(&req.employee_id)
    .bind(req.person_id)
    .bind(&req.department)
    .bind(&req.position)
    .bind(req.hire_date)
    .bind(&req.employment_type)
    .bind(salary)
    .bind(req.manager_id)
    .fetch_one(db)
    .await?;

    Ok(map_employee_to_response(employee))
}

pub async fn get_employee(db: &Db, id: Uuid) -> Result<EmployeeResponse> {
    let employee = sqlx::query_as::<_, EmployeeWithPerson>(
        r#"
        SELECT e.id, e.employee_id, e.person_id, 
               p.first_name, p.middle_name, p.last_name,
               pc.email, pc.phone,
               e.department_id, e.position_id, e.hire_date,
               e.employment_type, e.salary, e.manager_id,
               e.status, e.created_at, e.updated_at
        FROM employees e
        JOIN persons p ON p.id = e.person_id
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        WHERE e.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Employee not found"))?;

    Ok(map_employee_to_response(employee))
}

pub async fn list_employees(
    db: &Db,
    query: ListEmployeesQuery,
) -> Result<ListEmployeesResponse> {
    let page = query.page.unwrap_or(1).max(1);
    let page_size = query.page_size.unwrap_or(10).min(100);
    let offset = (page - 1) * page_size;

    let mut conditions: Vec<String> = vec!["1=1".to_string()];
    let mut count_query = "SELECT COUNT(*) FROM employees e".to_string();
    let mut select_query = format!(
        r#"
        SELECT e.id, e.employee_id, e.person_id, 
               p.first_name, p.middle_name, p.last_name,
               pc.email, pc.phone,
               e.department_id, e.position_id, e.hire_date,
               e.employment_type, e.salary, e.manager_id,
               e.status, e.created_at, e.updated_at
        FROM employees e
        JOIN persons p ON p.id = e.person_id
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        WHERE 1=1
        "#
    );

    if query.department.is_some() {
        conditions.push("e.department_id = $3".to_string());
    }

    if query.status.is_some() {
        let idx = if query.department.is_some() { 4 } else { 3 };
        conditions.push(format!("e.status = ${}", idx));
    }

    if let Some(_search) = &query.search {
        let idx = 3 + conditions.len() as i32 - 1;
        conditions.push(format!(
            "(p.first_name ILIKE ${0} OR p.last_name ILIKE ${0} OR e.employee_id ILIKE ${0})",
            idx
        ));
    }

    if conditions.len() > 1 {
        let where_clause = format!(" AND {}", conditions[1..].join(" AND "));
        count_query = format!("{} {}", count_query, where_clause);
        select_query = format!("{} {}", select_query, where_clause);
    }

    select_query = format!(
        "{} ORDER BY e.created_at DESC LIMIT $1 OFFSET $2",
        select_query
    );

    let mut count_q = sqlx::query_scalar::<_, i64>(&count_query);
    let mut select_q = sqlx::query_as::<_, EmployeeWithPerson>(&select_query)
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
    let employees = select_q.fetch_all(db).await?;

    Ok(ListEmployeesResponse {
        employees: employees.into_iter().map(map_employee_to_response).collect(),
        total,
        page,
        page_size,
    })
}

pub async fn update_employee(
    db: &Db,
    id: Uuid,
    req: UpdateEmployeeRequest,
) -> Result<EmployeeResponse> {
    let salary = req.salary.map(|s| BigDecimal::from_str(&s.to_string()).unwrap());

    let employee = sqlx::query_as::<_, EmployeeWithPerson>(
        r#"
        UPDATE employees e
        SET department_id = COALESCE($2, e.department_id),
            position_id = COALESCE($3, e.position_id),
            employment_type = COALESCE($4, e.employment_type),
            salary = COALESCE($5, e.salary),
            manager_id = COALESCE($6, e.manager_id),
            status = COALESCE($7, e.status),
            updated_at = NOW()
        FROM persons p
        LEFT JOIN person_contacts pc ON pc.person_id = p.id
        WHERE e.id = $1 AND p.id = e.person_id
        RETURNING e.id, e.employee_id, e.person_id, 
                  p.first_name, p.middle_name, p.last_name,
                  pc.email, pc.phone,
                  e.department_id, e.position_id, e.hire_date,
                  e.employment_type, e.salary, e.manager_id,
                  e.status, e.created_at, e.updated_at
        "#,
    )
    .bind(id)
    .bind(&req.department)
    .bind(&req.position)
    .bind(&req.employment_type)
    .bind(salary)
    .bind(req.manager_id)
    .bind(&req.status)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Employee not found"))?;

    Ok(map_employee_to_response(employee))
}

pub async fn delete_employee(db: &Db, id: Uuid) -> Result<()> {
    let result = sqlx::query("UPDATE employees SET status = 'inactive', updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow!("Employee not found"));
    }

    Ok(())
}

fn map_employee_to_response(emp: EmployeeWithPerson) -> EmployeeResponse {
    EmployeeResponse {
        id: emp.id,
        employee_id: emp.employee_id,
        person_id: emp.person_id,
        first_name: emp.first_name,
        middle_name: emp.middle_name,
        last_name: emp.last_name,
        email: emp.email,
        phone: emp.phone,
        department: emp.department_id.map(|id| id.to_string()),
        position: emp.position_id.map(|id| id.to_string()),
        hire_date: emp.hire_date,
        employment_type: emp.employment_type,
        salary: emp.salary.and_then(|s| s.to_string().parse().ok()),
        manager_id: emp.manager_id,
        status: emp.status,
    }
}
