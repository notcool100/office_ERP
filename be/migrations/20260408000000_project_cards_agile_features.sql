-- Agile card enhancements: human readable keys, sprint labels, comments, attachments, and activity history.

ALTER TABLE cards
    ADD COLUMN IF NOT EXISTS sequence_no INT,
    ADD COLUMN IF NOT EXISTS card_key VARCHAR(80),
    ADD COLUMN IF NOT EXISTS sprint_name VARCHAR(100);

WITH card_sequences AS (
    SELECT
        c.id,
        c.project_id,
        ROW_NUMBER() OVER (
            PARTITION BY c.project_id
            ORDER BY c.created_at, c.id
        )::INT AS next_seq
    FROM cards c
)
UPDATE cards c
SET sequence_no = cs.next_seq
FROM card_sequences cs
WHERE c.id = cs.id
  AND c.sequence_no IS NULL;

WITH project_slugs AS (
    SELECT
        p.id AS project_id,
        COALESCE(
            NULLIF(
                TRIM(BOTH '-' FROM REGEXP_REPLACE(LOWER(p.name), '[^a-z0-9]+', '-', 'g')),
                ''
            ),
            'project'
        ) AS slug
    FROM projects p
)
UPDATE cards c
SET card_key = CONCAT(
    'pro-',
    LEFT(ps.slug, 12),
    '-',
    LPAD(c.sequence_no::TEXT, 2, '0')
)
FROM project_slugs ps
WHERE c.project_id = ps.project_id
  AND c.sequence_no IS NOT NULL
  AND c.card_key IS NULL;

ALTER TABLE cards
    ALTER COLUMN sequence_no SET NOT NULL,
    ALTER COLUMN card_key SET NOT NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_cards_project_sequence
    ON cards(project_id, sequence_no);

CREATE UNIQUE INDEX IF NOT EXISTS idx_cards_project_card_key
    ON cards(project_id, card_key);

CREATE INDEX IF NOT EXISTS idx_cards_project_sprint
    ON cards(project_id, sprint_name);

CREATE TABLE IF NOT EXISTS card_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    comment TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_card_comments_card
    ON card_comments(card_id, created_at);

CREATE TABLE IF NOT EXISTS card_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    file_name VARCHAR(255) NOT NULL,
    content_type VARCHAR(120) NOT NULL,
    file_size BIGINT NOT NULL,
    file_data BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_card_attachments_card
    ON card_attachments(card_id, created_at);

CREATE TABLE IF NOT EXISTS card_activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    actor_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action_type VARCHAR(50) NOT NULL,
    description TEXT NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_card_activities_card
    ON card_activities(card_id, created_at);
