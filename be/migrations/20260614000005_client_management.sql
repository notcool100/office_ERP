CREATE TABLE clients (
    id             UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    name           VARCHAR(255) NOT NULL,
    contact_person VARCHAR(255),
    email          VARCHAR(255),
    phone          VARCHAR(50),
    industry       VARCHAR(100),
    website        VARCHAR(255),
    address        TEXT,
    notes          TEXT,
    status         VARCHAR(50)  NOT NULL DEFAULT 'active',
    created_by     UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at     TIMESTAMP    NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE TABLE client_deliverables (
    id               UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    client_id        UUID         NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    campaign_id      UUID         REFERENCES campaigns(id) ON DELETE SET NULL,
    title            VARCHAR(255) NOT NULL,
    description      TEXT,
    deliverable_type VARCHAR(100) NOT NULL DEFAULT 'other',
    due_date         DATE,
    status           VARCHAR(50)  NOT NULL DEFAULT 'pending',
    assigned_to      UUID         REFERENCES users(id) ON DELETE SET NULL,
    created_by       UUID         REFERENCES users(id) ON DELETE SET NULL,
    created_at       TIMESTAMP    NOT NULL DEFAULT NOW(),
    updated_at       TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clients_status        ON clients(status);
CREATE INDEX idx_clients_created_by    ON clients(created_by);
CREATE INDEX idx_deliverables_client   ON client_deliverables(client_id);
CREATE INDEX idx_deliverables_campaign ON client_deliverables(campaign_id);
CREATE INDEX idx_deliverables_status   ON client_deliverables(status);
CREATE INDEX idx_deliverables_due_date ON client_deliverables(due_date);

-- Navigation
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Clients', '/admin/digital-marketing/clients', 'Briefcase',
        (SELECT id FROM navigation_items WHERE path = '/admin/digital-marketing'), 4)
ON CONFLICT (path) DO NOTHING;

-- DM Manager: full CRUD
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p CROSS JOIN navigation_items n
WHERE p.name = 'Digital Marketing Manager'
  AND n.path = '/admin/digital-marketing/clients';

-- Administration: full CRUD
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d CROSS JOIN navigation_items n
WHERE d.name = 'Administration'
  AND n.path = '/admin/digital-marketing/clients';
