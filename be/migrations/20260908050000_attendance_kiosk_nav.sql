-- Surface the standalone face-recognition kiosk (fe/src/routes/attendance/kiosk,
-- outside /admin) in the sidebar next to the existing Attendance/Register Face
-- items, so it's easy to find/bookmark on the shared kiosk device. The kiosk
-- page itself needs no nav-path permission check (it isn't under /admin), but
-- its check-in call still requires the browser session to hold
-- /admin/hr/attendance permission - same audience as those sibling items.
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Attendance Kiosk',
    '/attendance/kiosk',
    'Clock',
    (SELECT id FROM navigation_items WHERE path = '/admin/hr'),
    6
)
ON CONFLICT (path) DO NOTHING;

INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/attendance/kiosk'
WHERE d.name = 'Administration'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
