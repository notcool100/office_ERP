CREATE TABLE media_assets (
    id           UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    name         VARCHAR(255) NOT NULL,
    description  TEXT,
    category     VARCHAR(50)  NOT NULL DEFAULT 'other',
    tags         TEXT[]       NOT NULL DEFAULT '{}',
    file_name    VARCHAR(255) NOT NULL,
    content_type VARCHAR(120) NOT NULL,
    file_size    BIGINT       NOT NULL,
    file_data    BYTEA        NOT NULL,
    uploaded_by  UUID         REFERENCES users(id) ON DELETE SET NULL,
    created_at   TIMESTAMP    NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMP    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ma_category    ON media_assets(category);
CREATE INDEX idx_ma_uploaded_by ON media_assets(uploaded_by);
CREATE INDEX idx_ma_created_at  ON media_assets(created_at DESC);

-- Navigation
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Media Library', '/admin/digital-marketing/media-library', 'FolderImage',
        (SELECT id FROM navigation_items WHERE path = '/admin/digital-marketing'), 2)
ON CONFLICT (path) DO NOTHING;

-- Digital Marketing Manager: full CRUD
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p CROSS JOIN navigation_items n
WHERE p.name = 'Digital Marketing Manager'
  AND n.path = '/admin/digital-marketing/media-library';

-- DM staff: create/read/update — no delete (except creator, handled in service)
INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, false
FROM positions p CROSS JOIN navigation_items n
WHERE p.name IN ('SEO Specialist','Content Writer','Social Media Manager','Graphic Designer','Marketing Analyst','PPC Specialist')
  AND n.path = '/admin/digital-marketing/media-library';

-- Administration full access
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d CROSS JOIN navigation_items n
WHERE d.name = 'Administration'
  AND n.path = '/admin/digital-marketing/media-library';
