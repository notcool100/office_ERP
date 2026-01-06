
-- Add User to Settings Menu
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('User', '/admin/settings/user', 'User', (SELECT id FROM navigation_items WHERE path = '/admin/settings'), 9)
ON CONFLICT (path) DO NOTHING;

-- Grant permissions to Administration/System Admin
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
CROSS JOIN navigation_items n
WHERE d.name = 'Administration' AND n.path = '/admin/settings/user'
ON CONFLICT DO NOTHING;

INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p, navigation_items n
WHERE p.name = 'System Administrator' AND n.path = '/admin/settings/user'
ON CONFLICT DO NOTHING;
