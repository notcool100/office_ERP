-- Meeting attachments (documents/recordings) and meeting minutes.
-- Both are restricted to actual meeting participants at the application layer
-- (join against meeting_participants), not via a DB-level policy here.

-- Files attached to a meeting: pre-meeting documents, or the meeting
-- recording uploaded after the call. category is 'document' | 'recording'.
CREATE TABLE meeting_attachments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    meeting_id UUID NOT NULL REFERENCES meetings(id) ON DELETE CASCADE,
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    category VARCHAR(20) NOT NULL DEFAULT 'document',
    file_name VARCHAR(255) NOT NULL,
    content_type VARCHAR(255) NOT NULL,
    file_size BIGINT NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_meeting_attachments_meeting_id ON meeting_attachments(meeting_id);
CREATE INDEX idx_meeting_attachments_uploaded_by ON meeting_attachments(uploaded_by);

-- One editable "minutes" doc per meeting (summary/notes/action items as
-- free text), upserted in place rather than versioned.
CREATE TABLE meeting_minutes (
    meeting_id UUID PRIMARY KEY REFERENCES meetings(id) ON DELETE CASCADE,
    content TEXT NOT NULL DEFAULT '',
    updated_by UUID REFERENCES users(id) ON DELETE SET NULL,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Promote "Meetings" from a nested Messaging child to its own top-level nav
-- item. This updates the row inserted by 20260910000001_add_meetings_navigation.sql
-- in place; role_permissions rows already reference it by navigation_item_id
-- (a stable UUID), so permissions carry over automatically.
UPDATE navigation_items
SET parent_id = NULL,
    path = '/meetings',
    display_order = 10
WHERE path = '/messaging/meetings';
