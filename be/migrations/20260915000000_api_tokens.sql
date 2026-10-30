-- Personal API tokens.
--
-- A token lets a script or an AI agent act as its owner against the
-- project API without going through the interactive login. Only the
-- SHA-256 of the token is stored: the plaintext is shown once at creation
-- and cannot be recovered afterwards. A token carries exactly its owner's
-- RBAC permissions, so revoking a user's access revokes their tokens' too.
CREATE TABLE api_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    token_hash VARCHAR(64) NOT NULL UNIQUE,
    -- First characters of the token, so a user can tell their tokens apart.
    token_prefix VARCHAR(16) NOT NULL,
    expires_at TIMESTAMPTZ,
    last_used_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_api_tokens_user ON api_tokens (user_id);

-- Settings > API Tokens. Every user manages their own tokens (the API has no
-- RBAC layer), so this grant only controls who sees the menu entry: the
-- Administration department, like the other Settings pages.
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'API Tokens',
    '/admin/settings/api-tokens',
    'KeyRound',
    (SELECT id FROM navigation_items WHERE path = '/admin/settings'),
    10
)
ON CONFLICT (path) DO NOTHING;

INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
JOIN navigation_items n ON n.path = '/admin/settings/api-tokens'
WHERE d.name = 'Administration'
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
