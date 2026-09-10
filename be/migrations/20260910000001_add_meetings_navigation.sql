-- Add Meetings to Navigation System (child of Messaging)

-- 1. Add Child Item: Meetings
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Meetings',
    '/messaging/meetings',
    'Video',
    (SELECT id FROM navigation_items WHERE path = '/messaging'),
    3
)
ON CONFLICT (path) DO NOTHING;

-- 2. Grant Permissions to Administration Department
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
CROSS JOIN navigation_items n
WHERE d.name = 'Administration'
AND n.path = '/messaging/meetings'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;

-- 3. Grant Permissions to System Administrator Position
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p
CROSS JOIN navigation_items n
WHERE p.name = 'System Administrator'
AND n.path = '/messaging/meetings'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
