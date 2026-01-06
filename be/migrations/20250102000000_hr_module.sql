-- HR Module Migration
-- Creates tables for employees, interns, leave management, and attendance tracking

-- Employees table
CREATE TABLE employees (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id VARCHAR(50) UNIQUE NOT NULL,
    person_id UUID REFERENCES persons(id) ON DELETE CASCADE,
    department VARCHAR(100),
    position VARCHAR(100),
    hire_date DATE NOT NULL,
    employment_type VARCHAR(50), -- Full-time, Part-time, Contract
    salary NUMERIC(12, 2),
    manager_id UUID REFERENCES employees(id),
    status VARCHAR(20) DEFAULT 'active', -- active, inactive, terminated
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Interns table
CREATE TABLE interns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    intern_id VARCHAR(50) UNIQUE NOT NULL,
    person_id UUID REFERENCES persons(id) ON DELETE CASCADE,
    department VARCHAR(100),
    supervisor_id UUID REFERENCES employees(id),
    start_date DATE NOT NULL,
    end_date DATE,
    stipend NUMERIC(10, 2),
    university VARCHAR(200),
    status VARCHAR(20) DEFAULT 'active', -- active, completed, terminated
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Leave types table
CREATE TABLE leave_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    max_days_per_year INT,
    requires_approval BOOLEAN DEFAULT true,
    carry_forward BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Leave requests table
CREATE TABLE leave_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID REFERENCES employees(id) ON DELETE CASCADE,
    leave_type_id UUID REFERENCES leave_types(id),
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    total_days NUMERIC(4, 1) NOT NULL,
    reason TEXT,
    status VARCHAR(20) DEFAULT 'pending', -- pending, approved, rejected, cancelled
    approved_by UUID REFERENCES users(id),
    approved_at TIMESTAMP,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Attendance records table
CREATE TABLE attendance_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID REFERENCES employees(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    check_in TIMESTAMP,
    check_out TIMESTAMP,
    total_hours NUMERIC(5, 2),
    status VARCHAR(20) DEFAULT 'present', -- present, late, half_day, absent
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(employee_id, date)
);

-- Indexes for performance
CREATE INDEX idx_employees_department ON employees(department);
CREATE INDEX idx_employees_status ON employees(status);
CREATE INDEX idx_employees_person ON employees(person_id);
CREATE INDEX idx_interns_status ON interns(status);
CREATE INDEX idx_interns_person ON interns(person_id);
CREATE INDEX idx_leave_requests_employee ON leave_requests(employee_id);
CREATE INDEX idx_leave_requests_status ON leave_requests(status);
CREATE INDEX idx_leave_requests_dates ON leave_requests(start_date, end_date);
CREATE INDEX idx_attendance_employee_date ON attendance_records(employee_id, date);
CREATE INDEX idx_attendance_date ON attendance_records(date);

-- Insert default leave types
INSERT INTO leave_types (name, description, max_days_per_year, requires_approval, carry_forward) VALUES
('Annual Leave', 'Paid annual leave', 20, true, true),
('Sick Leave', 'Medical leave', 10, false, false),
('Casual Leave', 'Short-term casual leave', 15, true, false),
('Maternity Leave', 'Maternity leave', 90, true, false),
('Paternity Leave', 'Paternity leave', 7, true, false);
