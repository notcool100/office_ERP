# HR Management — Module Roadmap

## Already Built

| Module | Path | What it does |
|--------|------|-------------|
| Employee | `/admin/hr/employee` | Employee profiles, department, position |
| Intern | `/admin/hr/intern` | Intern records separate from employees |
| Leave | `/admin/hr/leave` | Apply, approve/reject, balance tracking |
| Attendance | `/admin/hr/attendance` | Kiosk check-in/out, face registration |
| Person | `/admin/hr/person` | Underlying person records linked to users |

---

## What's Missing

### 1. Payroll
**Priority: High**
**Who uses it:** HR Manager, Finance

- Define salary structure per employee (base, allowances, deductions)
- Monthly payroll run — calculate net pay from attendance/leave data
- Payslip generation (viewable by employee, downloadable PDF)
- Bonus and deduction entries (one-off adjustments)
- Tax calculation (flat % or bracket-based)
- Payroll history — month-by-month records
- Mark payroll as paid / pending

**Tables needed:**
- `salary_structures` — base, housing, transport, other allowances, deductions
- `payroll_runs` — month, year, status (draft/processed/paid)
- `payroll_entries` — one row per employee per run (gross, deductions, net, tax)

---

### 2. Recruitment / Applicant Tracking (ATS)
**Priority: High**
**Who uses it:** HR Manager, Department Heads

- Create job openings (title, department, position, description, requirements)
- Track applicants per job (name, contact, CV link/upload, applied date)
- Pipeline stages: Applied → Screened → Interview → Offer → Hired / Rejected
- Move applicants between stages (Kanban-style or status dropdown)
- Interview scheduling — link to calendar, add notes
- Offer letter status (sent, accepted, declined)
- Convert hired applicant directly into Employee record

**Tables needed:**
- `job_openings` — title, department_id, position_id, description, status (open/closed/on-hold)
- `applicants` — job_opening_id, name, email, phone, cv_path, stage, notes, created_at
- `interview_notes` — applicant_id, interviewer_id, date, rating, notes

---

### 3. Onboarding Checklist
**Priority: Medium**
**Who uses it:** HR Manager, IT, new employee

- Define reusable checklist templates (e.g. "New Full-time Hire")
- Assign a checklist to a new employee on joining
- Tasks with owner (HR / IT / employee themselves) and due date
- Track completion per task — checkbox with who completed it and when
- Examples: Sign contract, Set up email, Issue laptop, Complete tax form, Badge photo

**Tables needed:**
- `onboarding_templates` — name, description
- `onboarding_template_tasks` — template_id, task, owner_role, due_day_offset
- `onboarding_assignments` — employee_id, template_id, started_at
- `onboarding_task_completions` — assignment_id, task_id, completed_by, completed_at

---

### 4. Performance Reviews
**Priority: Medium**
**Who uses it:** HR Manager, Managers, Employees

- Review cycles — define period (Q1 2026, Annual 2025, etc.)
- Each employee gets a review in a cycle
- Self-assessment section (employee fills in)
- Manager assessment section (manager fills in)
- Rating scale (e.g. 1–5) per category: work quality, communication, punctuality, initiative
- Final rating + overall comments
- Status: Draft → Submitted → Reviewed → Acknowledged

**Tables needed:**
- `review_cycles` — name, period_start, period_end, status
- `performance_reviews` — cycle_id, employee_id, reviewer_id, status
- `review_ratings` — review_id, category, self_score, manager_score, comments

---

### 5. HR Document Vault
**Priority: Medium**
**Who uses it:** HR Manager, Employee (read-only own docs)

- Upload documents per employee (contract, NDA, ID copy, offer letter, etc.)
- Document categories (Identity, Contract, Certificate, Other)
- Expiry date field — alert before expiry (e.g. visa, work permit)
- Employee can view their own documents
- HR can upload/delete any employee's documents

**Tables needed:**
- `employee_documents` — employee_id, category, title, file_path, expiry_date, uploaded_by, uploaded_at

---

### 6. Offboarding Checklist
**Priority: Low**
**Who uses it:** HR Manager, IT, departing employee

- Same checklist mechanic as Onboarding but for exits
- Tasks: Return laptop, Revoke system access, Final payslip, Exit interview, Handover document
- Link to employee — set separation date, type (resigned / terminated / contract end)

**Tables needed:**
- Reuse `onboarding_templates` pattern with a `type` column (onboarding/offboarding)
- `employee_separations` — employee_id, separation_date, type, reason, exit_interview_notes

---

### 7. Company Announcements
**Priority: Low**
**Who uses it:** HR Manager, All staff

- HR posts company-wide notices (policy changes, holidays, events)
- Target audience: all / department / position
- Pinned announcements stay at top
- Employees see their relevant announcements on dashboard
- Read receipts optional

**Tables needed:**
- `announcements` — title, body, target_type (all/department/position), target_id, pinned, published_at, created_by

---

## Build Order Recommendation

```
Phase 1 (Core ops)
├── Payroll
└── Recruitment / ATS

Phase 2 (Employee lifecycle)
├── Onboarding Checklist
├── HR Document Vault
└── Offboarding Checklist

Phase 3 (Growth)
├── Performance Reviews
└── Company Announcements
```
