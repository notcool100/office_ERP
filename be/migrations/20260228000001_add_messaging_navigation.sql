-- Add Messaging to Navigation System

-- 1. Add Parent Item: Messaging
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Messaging', '/messaging', 'MessageSquare', NULL, 8)
ON CONFLICT (path) DO NOTHING;

-- 2. Add Child Item: Channels
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Channels', 
    '/messaging/channels', 
    'Hash', 
    (SELECT id FROM navigation_items WHERE path = '/messaging'), 
    1
)
ON CONFLICT (path) DO NOTHING;

-- 3. Add Child Item: Direct Messages
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Direct Messages', 
    '/messaging/dm', 
    'MessageCircle', 
    (SELECT id FROM navigation_items WHERE path = '/messaging'), 
    2
)
ON CONFLICT (path) DO NOTHING;

-- 4. Grant Permissions to Administration Department
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
CROSS JOIN navigation_items n
WHERE d.name = 'Administration' 
AND n.path LIKE '/messaging%'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;

-- 5. Grant Permissions to System Administrator Position
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p
CROSS JOIN navigation_items n
WHERE p.name = 'System Administrator'
AND n.path LIKE '/messaging%'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
