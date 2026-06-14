-- Salary structure per employee (versioned by effective_date)
CREATE TABLE salary_structures (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    effective_date DATE NOT NULL DEFAULT CURRENT_DATE,
    basic_salary NUMERIC(12,2) NOT NULL DEFAULT 0,
    housing_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    transport_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    meal_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    other_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    income_tax_pct NUMERIC(5,2) NOT NULL DEFAULT 0,
    social_security_pct NUMERIC(5,2) NOT NULL DEFAULT 0,
    other_deduction_fixed NUMERIC(12,2) NOT NULL DEFAULT 0,
    notes TEXT,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (employee_id, effective_date)
);

-- One payroll run per calendar month
CREATE TABLE payroll_runs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    month SMALLINT NOT NULL CHECK (month BETWEEN 1 AND 12),
    year SMALLINT NOT NULL CHECK (year >= 2000),
    status VARCHAR(20) NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'processed', 'paid')),
    working_days SMALLINT NOT NULL DEFAULT 22,
    notes TEXT,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    processed_at TIMESTAMPTZ,
    paid_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (month, year)
);

-- One entry per employee per run (snapshot at processing time)
CREATE TABLE payroll_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    payroll_run_id UUID NOT NULL REFERENCES payroll_runs(id) ON DELETE CASCADE,
    employee_id UUID NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    -- Salary snapshot
    basic_salary NUMERIC(12,2) NOT NULL DEFAULT 0,
    housing_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    transport_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    meal_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    other_allowance NUMERIC(12,2) NOT NULL DEFAULT 0,
    -- One-off additions
    bonus NUMERIC(12,2) NOT NULL DEFAULT 0,
    overtime_pay NUMERIC(12,2) NOT NULL DEFAULT 0,
    -- Computed deductions
    income_tax NUMERIC(12,2) NOT NULL DEFAULT 0,
    social_security NUMERIC(12,2) NOT NULL DEFAULT 0,
    other_deduction NUMERIC(12,2) NOT NULL DEFAULT 0,
    -- Attendance
    working_days SMALLINT NOT NULL DEFAULT 22,
    days_present NUMERIC(5,2) NOT NULL DEFAULT 0,
    -- Computed totals
    gross_pay NUMERIC(12,2) NOT NULL DEFAULT 0,
    total_deductions NUMERIC(12,2) NOT NULL DEFAULT 0,
    net_pay NUMERIC(12,2) NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (payroll_run_id, employee_id)
);

CREATE INDEX idx_payroll_entries_run ON payroll_entries(payroll_run_id);
CREATE INDEX idx_payroll_entries_employee ON payroll_entries(employee_id);
CREATE INDEX idx_salary_structures_employee ON salary_structures(employee_id, effective_date DESC);

-- Navigation item
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Payroll',
    '/admin/hr/payroll',
    'Banknote',
    (SELECT id FROM navigation_items WHERE path = '/admin/hr'),
    10
) ON CONFLICT (path) DO NOTHING;

-- Human Resources: full CRUD
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d, navigation_items n
WHERE d.name = 'Human Resources' AND n.path = '/admin/hr/payroll'
ON CONFLICT DO NOTHING;

-- Finance: read only
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, false, true, false, false
FROM departments d, navigation_items n
WHERE d.name = 'Finance' AND n.path = '/admin/hr/payroll'
ON CONFLICT DO NOTHING;

-- Administration: full CRUD
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d, navigation_items n
WHERE d.name = 'Administration' AND n.path = '/admin/hr/payroll'
ON CONFLICT DO NOTHING;
