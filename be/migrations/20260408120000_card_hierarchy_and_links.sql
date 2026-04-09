-- Add is_done to board_columns
ALTER TABLE board_columns ADD COLUMN IF NOT EXISTS is_done BOOLEAN NOT NULL DEFAULT false;

-- Update existing "Done" columns
UPDATE board_columns SET is_done = true WHERE LOWER(name) IN ('done', 'completed', 'closed');

-- Add card_type, parent_id, is_migrated to cards
ALTER TABLE cards 
    ADD COLUMN IF NOT EXISTS card_type VARCHAR(20) DEFAULT 'task',
    ADD COLUMN IF NOT EXISTS parent_id UUID REFERENCES cards(id) ON DELETE CASCADE,
    ADD COLUMN IF NOT EXISTS is_migrated BOOLEAN NOT NULL DEFAULT false;

-- Step 1: Insert Legacy Epic per project
INSERT INTO cards (project_id, column_id, title, description, card_type, is_migrated, sequence_no, card_key)
SELECT 
    p.id, 
    (SELECT id FROM board_columns WHERE board_id = (SELECT id FROM boards WHERE project_id = p.id LIMIT 1) ORDER BY display_order ASC LIMIT 1),
    'Legacy Epic',
    'Auto-generated epic for organizing legacy cards without a parent.',
    'epic',
    true,
    COALESCE((SELECT MAX(sequence_no) FROM cards WHERE project_id = p.id), 0) + 1,
    'LEGACY-EPIC-' || p.id -- We will use a safe fallback format that can be fixed later or they can update manually
FROM projects p
WHERE EXISTS (SELECT 1 FROM cards c WHERE c.project_id = p.id AND c.card_type = 'task');

-- Step 2: Ensure Legacy Epics have a correct sequence_no if the DB constraint requires it to be unique (already handled roughly inline above but it's risky).
-- Actually sequence_no is strictly sequential. If we just insert, we might violate uniqueness on card_key. So let's build card keys correctly instead using the existing logic from a trigger/app layer, or just use UUID-based safe keys.
-- A better approach for migration is to use PL/pgSQL block to safely generate the sequence and keys.

DO $$ 
DECLARE 
    proj RECORD;
    epic_id UUID;
    story_id UUID;
    first_col UUID;
    seq INT;
    slug VARCHAR;
BEGIN
    FOR proj IN SELECT id, name FROM projects LOOP
        -- Check if project has orphaned cards
        IF EXISTS (
            SELECT 1 FROM cards 
            WHERE project_id = proj.id 
              AND parent_id IS NULL
        ) THEN
            -- Get first column
            SELECT id INTO first_col FROM board_columns 
            WHERE board_id = (SELECT id FROM boards WHERE project_id = proj.id LIMIT 1) 
            ORDER BY display_order ASC LIMIT 1;
            
            -- Prepare slug
            slug := COALESCE(NULLIF(TRIM(BOTH '-' FROM REGEXP_REPLACE(LOWER(proj.name), '[^a-z0-9]+', '-', 'g')), ''), 'project');
            slug := LEFT(slug, 12);
            
            -- Epic
            SELECT COALESCE(MAX(sequence_no), 0) + 1 INTO seq FROM cards WHERE project_id = proj.id;
            INSERT INTO cards (project_id, column_id, title, description, card_type, is_migrated, sequence_no, card_key)
            VALUES (proj.id, first_col, 'Legacy Epic', '[MIGRATED] Container for legacy items', 'epic', true, seq, 'pro-' || slug || '-' || LPAD(seq::text, 2, '0'))
            RETURNING id INTO epic_id;
            
            -- Story
            SELECT COALESCE(MAX(sequence_no), 0) + 1 INTO seq FROM cards WHERE project_id = proj.id;
            INSERT INTO cards (project_id, column_id, title, description, card_type, parent_id, is_migrated, sequence_no, card_key)
            VALUES (proj.id, first_col, 'Legacy Story', '[MIGRATED] Container for legacy tasks', 'story', epic_id, true, seq, 'pro-' || slug || '-' || LPAD(seq::text, 2, '0'))
            RETURNING id INTO story_id;
            
            -- Update existing cards to be child of Legacy Story
            UPDATE cards 
            SET parent_id = story_id, card_type = 'task', is_migrated = true
            WHERE project_id = proj.id 
              AND id != epic_id 
              AND id != story_id
              AND parent_id IS NULL;
        END IF;
    END LOOP;
END $$;

ALTER TABLE cards ALTER COLUMN card_type SET NOT NULL;

-- Create card links table
CREATE TABLE IF NOT EXISTS card_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    target_card_id UUID NOT NULL REFERENCES cards(id) ON DELETE CASCADE,
    link_type VARCHAR(20) NOT NULL, -- 'depends_on', 'relates_to'
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(source_card_id, target_card_id, link_type)
);

CREATE INDEX idx_card_links_source ON card_links(source_card_id);
CREATE INDEX idx_card_links_target ON card_links(target_card_id);
