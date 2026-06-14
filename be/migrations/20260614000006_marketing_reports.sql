-- Navigation
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Marketing Reports', '/admin/digital-marketing/reports', 'FileBarChart2',
        (SELECT id FROM navigation_items WHERE path = '/admin/digital-marketing'), 5)
ON CONFLICT (path) DO NOTHING;

-- Marketing Analyst: read-only
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, false, true, false, false
FROM positions p CROSS JOIN navigation_items n
WHERE p.name = 'Marketing Analyst'
  AND n.path = '/admin/digital-marketing/reports';

-- DM Manager: read-only
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, false, true, false, false
FROM positions p CROSS JOIN navigation_items n
WHERE p.name = 'Digital Marketing Manager'
  AND n.path = '/admin/digital-marketing/reports';

-- Administration: full
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d CROSS JOIN navigation_items n
WHERE d.name = 'Administration'
  AND n.path = '/admin/digital-marketing/reports';
