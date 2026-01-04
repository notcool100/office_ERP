-- RBAC Navigation System Migration
-- Creates role-based access control with departments, positions, navigation items, and permissions

-- Step 1: Create Departments reference table
CREATE TABLE departments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Step 2: Create Positions reference table
CREATE TABLE positions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) UNIQUE NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Step 3: Create Navigation Items table
CREATE TABLE navigation_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    path VARCHAR(255) NOT NULL,
    icon VARCHAR(100),
    parent_id UUID REFERENCES navigation_items(id) ON DELETE CASCADE,
    display_order INT DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(path)
);

-- Step 4: Create Role Permissions junction table
CREATE TABLE role_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    department_id UUID REFERENCES departments(id) ON DELETE CASCADE,
    position_id UUID REFERENCES positions(id) ON DELETE CASCADE,
    navigation_item_id UUID REFERENCES navigation_items(id) ON DELETE CASCADE NOT NULL,
    can_create BOOLEAN DEFAULT false,
    can_read BOOLEAN DEFAULT false,
    can_update BOOLEAN DEFAULT false,
    can_delete BOOLEAN DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT at_least_one_role CHECK (department_id IS NOT NULL OR position_id IS NOT NULL),
    UNIQUE(department_id, position_id, navigation_item_id)
);

-- Step 5: Migrate existing data - Extract unique departments from employees
INSERT INTO departments (name, description, is_active)
SELECT DISTINCT 
    department,
    'Auto-migrated from employees table',
    true
FROM employees
WHERE department IS NOT NULL AND department != ''
ON CONFLICT (name) DO NOTHING;

-- Step 6: Migrate existing data - Extract unique departments from interns
INSERT INTO departments (name, description, is_active)
SELECT DISTINCT 
    department,
    'Auto-migrated from interns table',
    true
FROM interns
WHERE department IS NOT NULL AND department != ''
ON CONFLICT (name) DO NOTHING;

-- Step 7: Migrate existing data - Extract unique positions from employees
INSERT INTO positions (name, description, is_active)
SELECT DISTINCT 
    position,
    'Auto-migrated from employees table',
    true
FROM employees
WHERE position IS NOT NULL AND position != ''
ON CONFLICT (name) DO NOTHING;

-- Step 8: Add new columns to employees table
ALTER TABLE employees ADD COLUMN department_id UUID REFERENCES departments(id);
ALTER TABLE employees ADD COLUMN position_id UUID REFERENCES positions(id);

-- Step 9: Update employees table with foreign key references
UPDATE employees e
SET department_id = d.id
FROM departments d
WHERE e.department = d.name;

UPDATE employees e
SET position_id = p.id
FROM positions p
WHERE e.position = p.name;

-- Step 10: Add new columns to interns table
ALTER TABLE interns ADD COLUMN department_id UUID REFERENCES departments(id);
ALTER TABLE interns ADD COLUMN position_id UUID REFERENCES positions(id);

-- Step 11: Update interns table with foreign key references
UPDATE interns i
SET department_id = d.id
FROM departments d
WHERE i.department = d.name;

-- Step 12: Drop old VARCHAR columns from employees
ALTER TABLE employees DROP COLUMN department;
ALTER TABLE employees DROP COLUMN position;

-- Step 13: Drop old VARCHAR column from interns (keep position_id as nullable)
ALTER TABLE interns DROP COLUMN department;

-- Step 14: Add is_admin flag to users table
ALTER TABLE users ADD COLUMN is_admin BOOLEAN DEFAULT false;

-- Step 15: Create indexes for performance
CREATE INDEX idx_departments_active ON departments(is_active);
CREATE INDEX idx_positions_active ON positions(is_active);
CREATE INDEX idx_navigation_items_active ON navigation_items(is_active);
CREATE INDEX idx_navigation_items_parent ON navigation_items(parent_id);
CREATE INDEX idx_navigation_items_order ON navigation_items(display_order);
CREATE INDEX idx_role_permissions_dept ON role_permissions(department_id);
CREATE INDEX idx_role_permissions_position ON role_permissions(position_id);
CREATE INDEX idx_role_permissions_nav ON role_permissions(navigation_item_id);
CREATE INDEX idx_employees_department ON employees(department_id);
CREATE INDEX idx_employees_position ON employees(position_id);
CREATE INDEX idx_interns_department ON interns(department_id);
CREATE INDEX idx_interns_position ON interns(position_id);

-- Step 16: Insert ALL navigation items matching frontend menu.ts structure
-- Main level items
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Dashboard', '/admin/dashboard', 'LayoutDashboard', NULL, 1),
('Profile', '/admin/profile', 'UserCog', NULL, 2),
('Settings', '/admin/settings', 'Settings', NULL, 3),
('Purchase', '/admin/purchase', 'ShoppingCart', NULL, 4),
('Sales', '/admin/sales', 'ChartColumn', NULL, 5),
('HR', '/admin/hr', 'Users', NULL, 6),
('Report', '/admin/report', 'ChartPie', NULL, 7);

-- Profile sub-items
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Information', '/admin/profile/information', 'CircleUser', (SELECT id FROM navigation_items WHERE path = '/admin/profile'), 1),
('Schedule', '/admin/profile/schedule', 'CalendarHeart', (SELECT id FROM navigation_items WHERE path = '/admin/profile'), 2);

-- Settings sub-items (Role folder)
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Role', '/admin/settings/role', 'Shield', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 1),
('Role List', '/admin/settings/role', 'List', (SELECT id FROM navigation_items WHERE path = '/admin/settings/role' AND icon = 'Shield'), 1),
('Create Role', '/admin/settings/role/create', 'Plus', (SELECT id FROM navigation_items WHERE path = '/admin/settings/role' AND icon = 'Shield'), 2);

-- Settings sub-items (Contact folder)
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Contact', '/admin/settings/contact', 'UserPlus', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 2),
('Contact List', '/admin/settings/contact', 'List', (SELECT id FROM navigation_items WHERE path = '/admin/settings/contact'), 1),
('Create Contact', '/admin/settings/contact/create', 'UserPlus', (SELECT id FROM navigation_items WHERE path = '/admin/settings/contact'), 2);

-- Settings sub-items (other)
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Product', '/admin/settings/product', 'Package', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 3),
('Calendar', '/admin/settings/calendar', 'Calendar', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 4),
('Department', '/admin/settings/department', 'mdi-office-building', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 5),
('Position', '/admin/settings/position', 'mdi-briefcase', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 6),
('Navigation', '/admin/settings/navigation', 'mdi-menu', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 7),
('Permissions', '/admin/settings/permissions', 'mdi-shield-account', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 8);

-- Purchase sub-items
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Purchase Order', '/admin/purchase/order', 'ClipboardList', (SELECT id FROM navigation_items WHERE path = '/admin/purchase'), 1),
('Bill', '/admin/purchase/bill', 'FileText', (SELECT id FROM navigation_items WHERE path = '/admin/purchase'), 2),
('Receipt Voucher', '/admin/purchase/receipt', 'Receipt', (SELECT id FROM navigation_items WHERE path = '/admin/purchase'), 3);

-- Sales sub-items
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Quotation', '/admin/sales/quotation', 'File', (SELECT id FROM navigation_items WHERE path = '/admin/sales'), 1),
('Invoice', '/admin/sales/invoice', 'FileX', (SELECT id FROM navigation_items WHERE path = '/admin/sales'), 2),
('Payment Voucher', '/admin/sales/payment', 'FileText', (SELECT id FROM navigation_items WHERE path = '/admin/sales'), 3);

-- HR sub-items
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Employee', '/admin/hr/employee', 'UserCheck', (SELECT id FROM navigation_items WHERE path = '/admin/hr'), 1),
('Intern', '/admin/hr/intern', 'SquareUser', (SELECT id FROM navigation_items WHERE path = '/admin/hr'), 2),
('Leave', '/admin/hr/leave', 'CalendarMinus', (SELECT id FROM navigation_items WHERE path = '/admin/hr'), 3),
('Attendance', '/admin/hr/attendance', 'Clock', (SELECT id FROM navigation_items WHERE path = '/admin/hr'), 4);

-- Report sub-items
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Income', '/admin/report/income', 'ChartColumnIncreasing', (SELECT id FROM navigation_items WHERE path = '/admin/report'), 1),
('Expenditure', '/admin/report/expenditure', 'ChartColumnDecreasing', (SELECT id FROM navigation_items WHERE path = '/admin/report'), 2),
('Sales Report', '/admin/report/sales', 'ChartBarStacked', (SELECT id FROM navigation_items WHERE path = '/admin/report'), 3);

-- Step 17: Insert some default departments if none exist
INSERT INTO departments (name, description) VALUES
('Administration', 'Administrative department'),
('Human Resources', 'HR department'),
('Information Technology', 'IT department'),
('Finance', 'Finance and accounting')
ON CONFLICT (name) DO NOTHING;

-- Step 18: Insert some default positions if none exist
INSERT INTO positions (name, description) VALUES
('System Administrator', 'Full system access'),
('Manager', 'Department manager'),
('Employee', 'Regular employee'),
('Intern', 'Intern position')
ON CONFLICT (name) DO NOTHING;

-- Step 19: Grant admin full access to all navigation items (department-based)
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT 
    d.id,
    n.id,
    true,
    true,
    true,
    true
FROM departments d
CROSS JOIN navigation_items n
WHERE d.name = 'Administration';

-- Step 20: Grant admin full access to all navigation items (position-based)
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT 
    p.id,
    n.id,
    true,
    true,
    true,
    true
FROM positions p
CROSS JOIN navigation_items n
WHERE p.name = 'System Administrator';
