use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use uuid::Uuid;

// ── Salary Structure ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalaryStructureRequest {
    pub employee_id: Uuid,
    pub effective_date: NaiveDate,
    pub basic_salary: BigDecimal,
    pub housing_allowance: BigDecimal,
    pub transport_allowance: BigDecimal,
    pub meal_allowance: BigDecimal,
    pub other_allowance: BigDecimal,
    pub income_tax_pct: BigDecimal,
    pub social_security_pct: BigDecimal,
    pub other_deduction_fixed: BigDecimal,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SalaryStructureResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub employee_code: String,
    pub effective_date: NaiveDate,
    pub basic_salary: BigDecimal,
    pub housing_allowance: BigDecimal,
    pub transport_allowance: BigDecimal,
    pub meal_allowance: BigDecimal,
    pub other_allowance: BigDecimal,
    pub income_tax_pct: BigDecimal,
    pub social_security_pct: BigDecimal,
    pub other_deduction_fixed: BigDecimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ── Payroll Run ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePayrollRunRequest {
    pub month: i16,
    pub year: i16,
    pub working_days: Option<i16>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PayrollRunSummary {
    pub id: Uuid,
    pub month: i16,
    pub year: i16,
    pub status: String,
    pub working_days: i16,
    pub notes: Option<String>,
    pub entry_count: i64,
    pub total_net_pay: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
}

// ── Payroll Entry ─────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PayrollEntryResponse {
    pub id: Uuid,
    pub payroll_run_id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub employee_code: String,
    pub department: Option<String>,
    pub basic_salary: BigDecimal,
    pub housing_allowance: BigDecimal,
    pub transport_allowance: BigDecimal,
    pub meal_allowance: BigDecimal,
    pub other_allowance: BigDecimal,
    pub bonus: BigDecimal,
    pub overtime_pay: BigDecimal,
    pub income_tax: BigDecimal,
    pub social_security: BigDecimal,
    pub other_deduction: BigDecimal,
    pub working_days: i16,
    pub days_present: BigDecimal,
    pub gross_pay: BigDecimal,
    pub total_deductions: BigDecimal,
    pub net_pay: BigDecimal,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePayrollEntryRequest {
    pub bonus: Option<BigDecimal>,
    pub overtime_pay: Option<BigDecimal>,
    pub other_deduction: Option<BigDecimal>,
    pub days_present: Option<BigDecimal>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayrollRunDetail {
    pub run: PayrollRunSummary,
    pub entries: Vec<PayrollEntryResponse>,
}

// Internal row used during payroll processing
#[derive(Debug, sqlx::FromRow)]
pub struct EmployeeSalaryRow {
    pub employee_id: Uuid,
    pub employee_name: String,
    pub employee_code: String,
    pub department: Option<String>,
    pub basic_salary: Option<BigDecimal>,
    pub housing_allowance: Option<BigDecimal>,
    pub transport_allowance: Option<BigDecimal>,
    pub meal_allowance: Option<BigDecimal>,
    pub other_allowance: Option<BigDecimal>,
    pub income_tax_pct: Option<BigDecimal>,
    pub social_security_pct: Option<BigDecimal>,
    pub other_deduction_fixed: Option<BigDecimal>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AttendanceSummaryRow {
    pub employee_id: Uuid,
    pub days_present: Option<BigDecimal>,
}

// ── My Payslips ───────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MyPayslip {
    pub id: Uuid,
    pub month: i16,
    pub year: i16,
    pub status: String,
    pub basic_salary: BigDecimal,
    pub housing_allowance: BigDecimal,
    pub transport_allowance: BigDecimal,
    pub meal_allowance: BigDecimal,
    pub other_allowance: BigDecimal,
    pub bonus: BigDecimal,
    pub overtime_pay: BigDecimal,
    pub income_tax: BigDecimal,
    pub social_security: BigDecimal,
    pub other_deduction: BigDecimal,
    pub working_days: i16,
    pub days_present: BigDecimal,
    pub gross_pay: BigDecimal,
    pub total_deductions: BigDecimal,
    pub net_pay: BigDecimal,
    pub paid_at: Option<DateTime<Utc>>,
}
