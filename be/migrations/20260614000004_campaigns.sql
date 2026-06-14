CREATE TABLE campaigns (
    id              UUID          PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(255)  NOT NULL,
    description     TEXT,
    platform        VARCHAR(100)  NOT NULL,
    status          VARCHAR(50)   NOT NULL DEFAULT 'draft',
    budget          NUMERIC(14,2) NOT NULL DEFAULT 0,
    spent           NUMERIC(14,2) NOT NULL DEFAULT 0,
    revenue         NUMERIC(14,2) NOT NULL DEFAULT 0,
    start_date      DATE,
    end_date        DATE,
    target_audience TEXT,
    goals           TEXT,
    created_by      UUID REFERENCES users(id) ON DELETE SET NULL,
    assigned_to     UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at      TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_campaigns_status      ON campaigns(status);
CREATE INDEX idx_campaigns_platform    ON campaigns(platform);
CREATE INDEX idx_campaigns_created_by  ON campaigns(created_by);
CREATE INDEX idx_campaigns_assigned_to ON campaigns(assigned_to);

-- Navigation
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Campaigns', '/admin/digital-marketing/campaigns', 'BarChart2',
        (SELECT id FROM navigation_items WHERE path = '/admin/digital-marketing'), 3)
ON CONFLICT (path) DO NOTHING;

-- DM Manager: full CRUD
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p CROSS JOIN navigation_items n
WHERE p.name = 'Digital Marketing Manager'
  AND n.path = '/admin/digital-marketing/campaigns';

-- Marketing Analyst, PPC Specialist: create/read/update
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, false
FROM positions p CROSS JOIN navigation_items n
WHERE p.name IN ('Marketing Analyst', 'PPC Specialist')
  AND n.path = '/admin/digital-marketing/campaigns';

-- Administration: full CRUD
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d CROSS JOIN navigation_items n
WHERE d.name = 'Administration'
  AND n.path = '/admin/digital-marketing/campaigns';
