-- Admin-editable company branding (logo, favicon, name) and office pinpoint
-- location. Single-row table, same shape as github_connections. The
-- location is captured here for future use (e.g. geofencing the
-- face-recognition attendance kiosk) - no enforcement is added yet.
CREATE TABLE company_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL DEFAULT 'Company',
    logo_path VARCHAR(500),
    logo_content_type VARCHAR(100),
    favicon_path VARCHAR(500),
    favicon_content_type VARCHAR(100),
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    location_name TEXT,
    updated_by UUID REFERENCES users(id),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO company_settings (name) VALUES ('Adya Technologies');

INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Company',
    '/admin/settings/company',
    'Building2',
    (SELECT id FROM navigation_items WHERE path = '/admin/settings'),
    11
)
ON CONFLICT (path) DO NOTHING;

INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/settings/company'
WHERE d.name = 'Administration'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
