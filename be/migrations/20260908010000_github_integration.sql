-- GitHub connector: one global PAT-based connection, per-project repo links,
-- and GitHub-issue provenance columns on cards.

CREATE TABLE github_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    access_token_encrypted TEXT NOT NULL,
    github_username VARCHAR(255) NOT NULL,
    connected_by UUID REFERENCES users(id),
    connected_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE project_github_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID UNIQUE NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    repo_owner VARCHAR(255) NOT NULL,
    repo_name VARCHAR(255) NOT NULL,
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    last_synced_at TIMESTAMP,
    last_sync_status VARCHAR(20), -- success, error
    last_sync_error TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

ALTER TABLE cards
    ADD COLUMN github_issue_number INT,
    ADD COLUMN github_issue_url TEXT,
    ADD COLUMN github_state VARCHAR(10), -- open, closed
    ADD COLUMN github_synced_at TIMESTAMP;

CREATE UNIQUE INDEX idx_cards_project_github_issue
    ON cards(project_id, github_issue_number) WHERE github_issue_number IS NOT NULL;

-- Navigation item for Settings > Connectors
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Connectors', '/admin/settings/connectors', 'Plug', NULL, 9)
ON CONFLICT (path) DO NOTHING;

INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/settings/connectors'
WHERE d.name = 'Administration'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
