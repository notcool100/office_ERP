CREATE TABLE content_calendar_items (
    id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    title         VARCHAR(255) NOT NULL,
    description   TEXT,
    content_type  VARCHAR(50)  NOT NULL DEFAULT 'post',
    channel       VARCHAR(50)  NOT NULL DEFAULT 'instagram',
    scheduled_date DATE        NOT NULL,
    scheduled_time TIME,
    status        VARCHAR(20)  NOT NULL DEFAULT 'draft',
    campaign_name VARCHAR(200),
    assigned_to   UUID         REFERENCES users(id) ON DELETE SET NULL,
    created_by    UUID         REFERENCES users(id) NOT NULL,
    notes         TEXT,
    created_at    TIMESTAMP    NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cc_date       ON content_calendar_items(scheduled_date);
CREATE INDEX idx_cc_status     ON content_calendar_items(status);
CREATE INDEX idx_cc_channel    ON content_calendar_items(channel);
CREATE INDEX idx_cc_created_by ON content_calendar_items(created_by);
CREATE INDEX idx_cc_assigned   ON content_calendar_items(assigned_to);

-- Navigation items
INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Digital Marketing', '/admin/digital-marketing', 'Megaphone', NULL, 8)
ON CONFLICT (path) DO NOTHING;

INSERT INTO navigation_items (name, path, icon, parent_id, display_order) VALUES
('Content Calendar', '/admin/digital-marketing/content-calendar', 'CalendarDays',
 (SELECT id FROM navigation_items WHERE path = '/admin/digital-marketing'), 1)
ON CONFLICT (path) DO NOTHING;

-- Digital Marketing Manager: full CRUD
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p CROSS JOIN navigation_items n
WHERE p.name = 'Digital Marketing Manager'
  AND n.path IN ('/admin/digital-marketing', '/admin/digital-marketing/content-calendar');

-- DM staff: create/read/update, no delete
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, false
FROM positions p CROSS JOIN navigation_items n
WHERE p.name IN ('SEO Specialist','Content Writer','Social Media Manager','Graphic Designer','Marketing Analyst','PPC Specialist')
  AND n.path IN ('/admin/digital-marketing', '/admin/digital-marketing/content-calendar');

-- Administration full access (already covered by dept permissions, but add position-based too for nav items)
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d CROSS JOIN navigation_items n
WHERE d.name = 'Administration'
  AND n.path IN ('/admin/digital-marketing', '/admin/digital-marketing/content-calendar');
