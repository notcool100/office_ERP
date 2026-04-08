-- Sprint management: proper sprint entities for projects.

CREATE TABLE IF NOT EXISTS sprints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    goal TEXT,
    start_date DATE,
    end_date DATE,
    status VARCHAR(20) NOT NULL DEFAULT 'planning',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_sprints_project ON sprints(project_id);
CREATE INDEX IF NOT EXISTS idx_sprints_status ON sprints(project_id, status);

ALTER TABLE cards ADD COLUMN IF NOT EXISTS sprint_id UUID REFERENCES sprints(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_cards_sprint ON cards(sprint_id);

-- Backfill: create sprints from existing sprint_name values on cards
INSERT INTO sprints (project_id, name, status)
SELECT DISTINCT c.project_id, c.sprint_name, 'active'
FROM cards c
WHERE c.sprint_name IS NOT NULL AND c.sprint_name != ''
ON CONFLICT DO NOTHING;

-- Link existing cards to their backfilled sprints
UPDATE cards c
SET sprint_id = s.id
FROM sprints s
WHERE s.project_id = c.project_id
  AND s.name = c.sprint_name
  AND c.sprint_id IS NULL
  AND c.sprint_name IS NOT NULL
  AND c.sprint_name != '';
