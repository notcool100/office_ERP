CREATE TABLE notification_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key VARCHAR(100) NOT NULL UNIQUE,
    label VARCHAR(255) NOT NULL,
    description TEXT,
    emails TEXT NOT NULL DEFAULT '[]',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO notification_settings (key, label, description, emails) VALUES
(
    'leave_notifications',
    'Leave Request Notifications',
    'These email addresses receive a notification whenever an employee submits a leave request.',
    '[]'
);

INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Notifications',
    '/admin/settings/notifications',
    'Bell',
    (SELECT id FROM navigation_items WHERE path = '/admin/settings'),
    10
) ON CONFLICT (path) DO NOTHING;
