use anyhow::{Result, anyhow};
use chrono::{Datelike, NaiveDate};
use sqlx::types::BigDecimal;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::db::Db;

use super::dto::{
    AttendanceSummaryRow, CreatePayrollRunRequest, EmployeeSalaryRow, MyPayslip,
    PayrollEntryResponse, PayrollRunDetail, PayrollRunSummary, SalaryStructureRequest,
    SalaryStructureResponse, UpdatePayrollEntryRequest,
};

fn bd(val: f64) -> BigDecimal {
    BigDecimal::from_str(&format!("{:.2}", val)).unwrap_or_default()
}

fn f(val: &BigDecimal) -> f64 {
    val.to_string().parse::<f64>().unwrap_or(0.0)
}

// ── Salary Structures ─────────────────────────────────────────────────────────

pub async fn list_salary_structures(db: &Db) -> Result<Vec<SalaryStructureResponse>> {
    let rows = sqlx::query_as::<_, SalaryStructureResponse>(
        r#"
        SELECT
            ss.id,
            ss.employee_id,
            CONCAT(p.first_name, ' ', p.last_name) AS employee_name,
            e.employee_id AS employee_code,
            ss.effective_date,
            ss.basic_salary,
            ss.housing_allowance,
            ss.transport_allowance,
            ss.meal_allowance,
            ss.other_allowance,
            ss.income_tax_pct,
            ss.social_security_pct,
            ss.other_deduction_fixed,
            ss.notes,
            ss.created_at
        FROM salary_structures ss
        JOIN employees e ON e.id = ss.employee_id
        JOIN persons p ON p.id = e.person_id
        ORDER BY p.first_name, p.last_name, ss.effective_date DESC
        "#,
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn create_salary_structure(
    db: &Db,
    req: SalaryStructureRequest,
    created_by: Uuid,
) -> Result<SalaryStructureResponse> {
    let id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO salary_structures (
            employee_id, effective_date,
            basic_salary, housing_allowance, transport_allowance,
            meal_allowance, other_allowance,
            income_tax_pct, social_security_pct, other_deduction_fixed,
            notes, created_by
        ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
        ON CONFLICT (employee_id, effective_date)
        DO UPDATE SET
            basic_salary = EXCLUDED.basic_salary,
            housing_allowance = EXCLUDED.housing_allowance,
            transport_allowance = EXCLUDED.transport_allowance,
            meal_allowance = EXCLUDED.meal_allowance,
            other_allowance = EXCLUDED.other_allowance,
            income_tax_pct = EXCLUDED.income_tax_pct,
            social_security_pct = EXCLUDED.social_security_pct,
            other_deduction_fixed = EXCLUDED.other_deduction_fixed,
            notes = EXCLUDED.notes,
            updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(req.employee_id)
    .bind(req.effective_date)
    .bind(&req.basic_salary)
    .bind(&req.housing_allowance)
    .bind(&req.transport_allowance)
    .bind(&req.meal_allowance)
    .bind(&req.other_allowance)
    .bind(&req.income_tax_pct)
    .bind(&req.social_security_pct)
    .bind(&req.other_deduction_fixed)
    .bind(&req.notes)
    .bind(created_by)
    .fetch_one(db)
    .await?;

    get_salary_structure_by_id(db, id).await
}

pub async fn get_salary_structure_by_id(db: &Db, id: Uuid) -> Result<SalaryStructureResponse> {
    let row = sqlx::query_as::<_, SalaryStructureResponse>(
        r#"
        SELECT
            ss.id, ss.employee_id,
            CONCAT(p.first_name, ' ', p.last_name) AS employee_name,
            e.employee_id AS employee_code,
            ss.effective_date,
            ss.basic_salary, ss.housing_allowance, ss.transport_allowance,
            ss.meal_allowance, ss.other_allowance,
            ss.income_tax_pct, ss.social_security_pct, ss.other_deduction_fixed,
            ss.notes, ss.created_at
        FROM salary_structures ss
        JOIN employees e ON e.id = ss.employee_id
        JOIN persons p ON p.id = e.person_id
        WHERE ss.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Salary structure not found"))?;
    Ok(row)
}

pub async fn delete_salary_structure(db: &Db, id: Uuid) -> Result<()> {
    let rows = sqlx::query("DELETE FROM salary_structures WHERE id = $1")
        .bind(id)
        .execute(db)
        .await?;
    if rows.rows_affected() == 0 {
        return Err(anyhow!("Salary structure not found"));
    }
    Ok(())
}

// ── Payroll Runs ──────────────────────────────────────────────────────────────

pub async fn list_payroll_runs(db: &Db) -> Result<Vec<PayrollRunSummary>> {
    let rows = sqlx::query_as::<_, PayrollRunSummary>(
        r#"
        SELECT
            pr.id, pr.month, pr.year, pr.status, pr.working_days, pr.notes,
            COUNT(pe.id) AS entry_count,
            COALESCE(SUM(pe.net_pay), 0) AS total_net_pay,
            pr.created_at, pr.processed_at, pr.paid_at
        FROM payroll_runs pr
        LEFT JOIN payroll_entries pe ON pe.payroll_run_id = pr.id
        GROUP BY pr.id
        ORDER BY pr.year DESC, pr.month DESC
        "#,
    )
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn create_payroll_run(
    db: &Db,
    req: CreatePayrollRunRequest,
    created_by: Uuid,
) -> Result<PayrollRunSummary> {
    if req.month < 1 || req.month > 12 {
        return Err(anyhow!("Month must be between 1 and 12"));
    }
    if req.year < 2000 {
        return Err(anyhow!("Invalid year"));
    }
    let working_days = req.working_days.unwrap_or(22);
    let id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO payroll_runs (month, year, working_days, notes, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
    )
    .bind(req.month)
    .bind(req.year)
    .bind(working_days)
    .bind(&req.notes)
    .bind(created_by)
    .fetch_one(db)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") {
            anyhow!("A payroll run for {}/{} already exists", req.month, req.year)
        } else {
            anyhow!(e)
        }
    })?;

    get_payroll_run_summary(db, id).await
}

pub async fn get_payroll_run_summary(db: &Db, id: Uuid) -> Result<PayrollRunSummary> {
    sqlx::query_as::<_, PayrollRunSummary>(
        r#"
        SELECT
            pr.id, pr.month, pr.year, pr.status, pr.working_days, pr.notes,
            COUNT(pe.id) AS entry_count,
            COALESCE(SUM(pe.net_pay), 0) AS total_net_pay,
            pr.created_at, pr.processed_at, pr.paid_at
        FROM payroll_runs pr
        LEFT JOIN payroll_entries pe ON pe.payroll_run_id = pr.id
        WHERE pr.id = $1
        GROUP BY pr.id
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Payroll run not found"))
}

pub async fn get_payroll_run_detail(db: &Db, id: Uuid) -> Result<PayrollRunDetail> {
    let run = get_payroll_run_summary(db, id).await?;
    let entries = list_entries_for_run(db, id).await?;
    Ok(PayrollRunDetail { run, entries })
}

pub async fn list_entries_for_run(db: &Db, run_id: Uuid) -> Result<Vec<PayrollEntryResponse>> {
    let rows = sqlx::query_as::<_, PayrollEntryResponse>(
        r#"
        SELECT
            pe.id, pe.payroll_run_id, pe.employee_id,
            CONCAT(p.first_name, ' ', p.last_name) AS employee_name,
            e.employee_id AS employee_code,
            d.name AS department,
            pe.basic_salary, pe.housing_allowance, pe.transport_allowance,
            pe.meal_allowance, pe.other_allowance,
            pe.bonus, pe.overtime_pay,
            pe.income_tax, pe.social_security, pe.other_deduction,
            pe.working_days, pe.days_present,
            pe.gross_pay, pe.total_deductions, pe.net_pay,
            pe.notes
        FROM payroll_entries pe
        JOIN employees e ON e.id = pe.employee_id
        JOIN persons p ON p.id = e.person_id
        LEFT JOIN departments d ON d.id = e.department_id
        WHERE pe.payroll_run_id = $1
        ORDER BY p.first_name, p.last_name
        "#,
    )
    .bind(run_id)
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn process_payroll_run(db: &Db, run_id: Uuid) -> Result<PayrollRunDetail> {
    // Fetch the run
    let run: (i16, i16, i16, String) = sqlx::query_as(
        "SELECT month, year, working_days, status FROM payroll_runs WHERE id = $1",
    )
    .bind(run_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Payroll run not found"))?;

    let (month, year, working_days, status) = run;
    if status == "paid" {
        return Err(anyhow!("Cannot reprocess a paid payroll run"));
    }

    // Build date range for the month
    let first_day = NaiveDate::from_ymd_opt(year as i32, month as u32, 1)
        .ok_or_else(|| anyhow!("Invalid month/year"))?;
    let last_day = {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };
        NaiveDate::from_ymd_opt(next_year as i32, next_month as u32, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };

    // All active employees with their latest salary structure effective <= last_day
    let employees = sqlx::query_as::<_, EmployeeSalaryRow>(
        r#"
        SELECT
            e.id AS employee_id,
            CONCAT(p.first_name, ' ', p.last_name) AS employee_name,
            e.employee_id AS employee_code,
            d.name AS department,
            ss.basic_salary,
            ss.housing_allowance,
            ss.transport_allowance,
            ss.meal_allowance,
            ss.other_allowance,
            ss.income_tax_pct,
            ss.social_security_pct,
            ss.other_deduction_fixed
        FROM employees e
        JOIN persons p ON p.id = e.person_id
        LEFT JOIN departments d ON d.id = e.department_id
        LEFT JOIN LATERAL (
            SELECT *
            FROM salary_structures s
            WHERE s.employee_id = e.id AND s.effective_date <= $1
            ORDER BY s.effective_date DESC
            LIMIT 1
        ) ss ON true
        WHERE e.status = 'active'
        ORDER BY p.first_name, p.last_name
        "#,
    )
    .bind(last_day)
    .fetch_all(db)
    .await?;

    // Attendance summary for the month (present + late = 1 day, half_day = 0.5)
    let attendance_rows = sqlx::query_as::<_, AttendanceSummaryRow>(
        r#"
        SELECT
            employee_id,
            SUM(
                CASE status
                    WHEN 'half_day' THEN 0.5
                    ELSE 1.0
                END
            ) AS days_present
        FROM attendance_records
        WHERE date >= $1 AND date <= $2
          AND status IN ('present', 'late', 'half_day')
        GROUP BY employee_id
        "#,
    )
    .bind(first_day)
    .bind(last_day)
    .fetch_all(db)
    .await?;

    let attendance_map: HashMap<Uuid, f64> = attendance_rows
        .into_iter()
        .map(|r| (r.employee_id, f(&r.days_present.unwrap_or_default())))
        .collect();

    let wd = working_days as f64;

    // Delete existing entries for this run (re-process)
    sqlx::query("DELETE FROM payroll_entries WHERE payroll_run_id = $1")
        .bind(run_id)
        .execute(db)
        .await?;

    // Insert one entry per employee
    for emp in &employees {
        let basic = f(&emp.basic_salary.clone().unwrap_or_default());
        let housing = f(&emp.housing_allowance.clone().unwrap_or_default());
        let transport = f(&emp.transport_allowance.clone().unwrap_or_default());
        let meal = f(&emp.meal_allowance.clone().unwrap_or_default());
        let other_allow = f(&emp.other_allowance.clone().unwrap_or_default());
        let tax_pct = f(&emp.income_tax_pct.clone().unwrap_or_default());
        let ss_pct = f(&emp.social_security_pct.clone().unwrap_or_default());
        let other_ded = f(&emp.other_deduction_fixed.clone().unwrap_or_default());

        let days_present = *attendance_map.get(&emp.employee_id).unwrap_or(&wd);
        let proration = if wd > 0.0 {
            (days_present / wd).min(1.0)
        } else {
            1.0
        };

        let monthly_salary = basic + housing + transport + meal + other_allow;
        let gross = monthly_salary * proration;
        let income_tax = (gross * tax_pct / 100.0 * 100.0).round() / 100.0;
        let social_security = (basic * ss_pct / 100.0 * 100.0).round() / 100.0;
        let total_deductions = income_tax + social_security + other_ded;
        let net = (gross - total_deductions).max(0.0);

        sqlx::query(
            r#"
            INSERT INTO payroll_entries (
                payroll_run_id, employee_id,
                basic_salary, housing_allowance, transport_allowance,
                meal_allowance, other_allowance,
                bonus, overtime_pay,
                income_tax, social_security, other_deduction,
                working_days, days_present,
                gross_pay, total_deductions, net_pay
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7,
                0, 0,
                $8, $9, $10,
                $11, $12,
                $13, $14, $15
            )
            "#,
        )
        .bind(run_id)
        .bind(emp.employee_id)
        .bind(bd(basic))
        .bind(bd(housing))
        .bind(bd(transport))
        .bind(bd(meal))
        .bind(bd(other_allow))
        .bind(bd(income_tax))
        .bind(bd(social_security))
        .bind(bd(other_ded))
        .bind(working_days)
        .bind(bd(days_present))
        .bind(bd(gross))
        .bind(bd(total_deductions))
        .bind(bd(net))
        .execute(db)
        .await?;
    }

    // Mark run as processed
    sqlx::query(
        "UPDATE payroll_runs SET status = 'processed', processed_at = NOW(), updated_at = NOW() WHERE id = $1",
    )
    .bind(run_id)
    .execute(db)
    .await?;

    get_payroll_run_detail(db, run_id).await
}

pub async fn mark_run_paid(db: &Db, run_id: Uuid) -> Result<PayrollRunSummary> {
    let rows = sqlx::query(
        r#"
        UPDATE payroll_runs
        SET status = 'paid', paid_at = NOW(), updated_at = NOW()
        WHERE id = $1 AND status = 'processed'
        "#,
    )
    .bind(run_id)
    .execute(db)
    .await?;
    if rows.rows_affected() == 0 {
        return Err(anyhow!("Run not found or not in 'processed' status"));
    }
    get_payroll_run_summary(db, run_id).await
}

pub async fn delete_payroll_run(db: &Db, run_id: Uuid) -> Result<()> {
    let rows = sqlx::query(
        "DELETE FROM payroll_runs WHERE id = $1 AND status = 'draft'",
    )
    .bind(run_id)
    .execute(db)
    .await?;
    if rows.rows_affected() == 0 {
        return Err(anyhow!("Run not found or cannot delete a processed/paid run"));
    }
    Ok(())
}

// ── Entry Adjustments ─────────────────────────────────────────────────────────

pub async fn update_payroll_entry(
    db: &Db,
    entry_id: Uuid,
    req: UpdatePayrollEntryRequest,
) -> Result<PayrollEntryResponse> {
    // Fetch current entry data
    let current = sqlx::query_as::<_, PayrollEntryResponse>(
        r#"
        SELECT
            pe.id, pe.payroll_run_id, pe.employee_id,
            CONCAT(p.first_name, ' ', p.last_name) AS employee_name,
            e.employee_id AS employee_code,
            d.name AS department,
            pe.basic_salary, pe.housing_allowance, pe.transport_allowance,
            pe.meal_allowance, pe.other_allowance,
            pe.bonus, pe.overtime_pay,
            pe.income_tax, pe.social_security, pe.other_deduction,
            pe.working_days, pe.days_present,
            pe.gross_pay, pe.total_deductions, pe.net_pay,
            pe.notes
        FROM payroll_entries pe
        JOIN employees e ON e.id = pe.employee_id
        JOIN persons p ON p.id = e.person_id
        LEFT JOIN departments d ON d.id = e.department_id
        WHERE pe.id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_optional(db)
    .await?
    .ok_or_else(|| anyhow!("Entry not found"))?;

    // Check run is not paid
    let run_status: String =
        sqlx::query_scalar("SELECT status FROM payroll_runs WHERE id = $1")
            .bind(current.payroll_run_id)
            .fetch_one(db)
            .await?;
    if run_status == "paid" {
        return Err(anyhow!("Cannot edit entries in a paid payroll run"));
    }

    let wd = current.working_days as f64;
    let days_present = f(&req.days_present.clone().unwrap_or_else(|| current.days_present.clone()));
    let bonus = f(&req.bonus.clone().unwrap_or_else(|| current.bonus.clone()));
    let overtime = f(&req.overtime_pay.clone().unwrap_or_else(|| current.overtime_pay.clone()));
    let other_ded = f(&req.other_deduction.clone().unwrap_or_else(|| current.other_deduction.clone()));

    let basic = f(&current.basic_salary);
    let housing = f(&current.housing_allowance);
    let transport = f(&current.transport_allowance);
    let meal = f(&current.meal_allowance);
    let other_allow = f(&current.other_allowance);

    let proration = if wd > 0.0 { (days_present / wd).min(1.0) } else { 1.0 };
    let monthly_salary = basic + housing + transport + meal + other_allow;
    let gross = monthly_salary * proration + bonus + overtime;
    let income_tax = (gross * f(&current.income_tax) / f(&current.gross_pay).max(1.0) * 100.0).round() / 100.0;
    // recalculate from stored % — fetch from salary_structures
    // simpler: keep same income_tax proportion (tax / old_gross * new_gross)
    let old_gross = f(&current.gross_pay);
    let new_income_tax = if old_gross > 0.0 {
        (f(&current.income_tax) / old_gross * gross * 100.0).round() / 100.0
    } else {
        0.0
    };
    let new_ss = f(&current.social_security); // unchanged (based on basic)
    let total_deductions = new_income_tax + new_ss + other_ded;
    let net = (gross - total_deductions).max(0.0);

    sqlx::query(
        r#"
        UPDATE payroll_entries SET
            bonus = $1, overtime_pay = $2, other_deduction = $3,
            days_present = $4,
            income_tax = $5,
            gross_pay = $6, total_deductions = $7, net_pay = $8,
            notes = $9,
            updated_at = NOW()
        WHERE id = $10
        "#,
    )
    .bind(bd(bonus))
    .bind(bd(overtime))
    .bind(bd(other_ded))
    .bind(bd(days_present))
    .bind(bd(new_income_tax))
    .bind(bd(gross))
    .bind(bd(total_deductions))
    .bind(bd(net))
    .bind(&req.notes)
    .bind(entry_id)
    .execute(db)
    .await?;

    // Re-fetch with updated values
    sqlx::query_as::<_, PayrollEntryResponse>(
        r#"
        SELECT
            pe.id, pe.payroll_run_id, pe.employee_id,
            CONCAT(p.first_name, ' ', p.last_name) AS employee_name,
            e.employee_id AS employee_code,
            d.name AS department,
            pe.basic_salary, pe.housing_allowance, pe.transport_allowance,
            pe.meal_allowance, pe.other_allowance,
            pe.bonus, pe.overtime_pay,
            pe.income_tax, pe.social_security, pe.other_deduction,
            pe.working_days, pe.days_present,
            pe.gross_pay, pe.total_deductions, pe.net_pay,
            pe.notes
        FROM payroll_entries pe
        JOIN employees e ON e.id = pe.employee_id
        JOIN persons p ON p.id = e.person_id
        LEFT JOIN departments d ON d.id = e.department_id
        WHERE pe.id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_one(db)
    .await
    .map_err(|e| anyhow!(e))
}

// ── My Payslips ───────────────────────────────────────────────────────────────

pub async fn get_my_payslips(db: &Db, user_id: Uuid) -> Result<Vec<MyPayslip>> {
    let rows = sqlx::query_as::<_, MyPayslip>(
        r#"
        SELECT
            pe.id, pr.month, pr.year, pr.status,
            pe.basic_salary, pe.housing_allowance, pe.transport_allowance,
            pe.meal_allowance, pe.other_allowance,
            pe.bonus, pe.overtime_pay,
            pe.income_tax, pe.social_security, pe.other_deduction,
            pe.working_days, pe.days_present,
            pe.gross_pay, pe.total_deductions, pe.net_pay,
            pr.paid_at
        FROM payroll_entries pe
        JOIN payroll_runs pr ON pr.id = pe.payroll_run_id
        JOIN employees e ON e.id = pe.employee_id
        JOIN users u ON u.person_id = e.person_id
        WHERE u.id = $1
        ORDER BY pr.year DESC, pr.month DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;
    Ok(rows)
}
