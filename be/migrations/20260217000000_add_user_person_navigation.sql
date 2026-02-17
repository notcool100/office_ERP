-- Add missing navigation item for Person management

INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES
    (
        'Person',
        '/admin/hr/person',
        'UserPlus',
        (SELECT id FROM navigation_items WHERE path = '/admin/hr'),
        6
    )
ON CONFLICT (path) DO NOTHING;

-- Grant admin department full access to the new item
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/hr/person'
WHERE d.name = 'Administration'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
