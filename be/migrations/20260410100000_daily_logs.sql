-- Daily Logs Migration

CREATE TABLE daily_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE NOT NULL,
    log_date DATE NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE daily_log_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    daily_log_id UUID REFERENCES daily_logs(id) ON DELETE CASCADE NOT NULL,
    card_id UUID REFERENCES cards(id) ON DELETE CASCADE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Index for performance
CREATE INDEX idx_daily_logs_user ON daily_logs(user_id);
CREATE INDEX idx_daily_logs_date ON daily_logs(log_date);
CREATE INDEX idx_daily_log_links_log ON daily_log_links(daily_log_id);
CREATE INDEX idx_daily_log_links_card ON daily_log_links(card_id);

-- Navigation item for Daily Log
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Daily Log', '/admin/daily-log', 'ClipboardList', NULL, 1) -- Position 1 for high visibility
ON CONFLICT (path) DO NOTHING;

-- Permissions for Administration Department
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/daily-log'
WHERE d.name = 'Administration'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;

-- Permissions for Human Resources Department (Admins/HR can view all)
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/daily-log'
WHERE d.name = 'Human Resources'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;

-- Permissions for Finance (Example of another department that can read/create their own)
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/daily-log'
WHERE d.name = 'Finance'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;

-- Permissions for IT (Example of another department that can read/create their own)
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/daily-log'
WHERE d.name = 'Information Technology'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
