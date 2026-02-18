-- Calendar events (personal, department, company)

CREATE TABLE calendar_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(200) NOT NULL,
    description TEXT,
    location VARCHAR(200),
    start_at TIMESTAMP NOT NULL,
    end_at TIMESTAMP NOT NULL,
    all_day BOOLEAN DEFAULT false,
    scope VARCHAR(20) NOT NULL DEFAULT 'personal', -- personal, department, company
    department_id UUID REFERENCES departments(id) ON DELETE SET NULL,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT calendar_events_time_check CHECK (end_at >= start_at),
    CONSTRAINT calendar_events_scope_check CHECK (
        (scope = 'personal' AND department_id IS NULL)
        OR (scope = 'company' AND department_id IS NULL)
        OR (scope = 'department' AND department_id IS NOT NULL)
    )
);

CREATE INDEX idx_calendar_events_start ON calendar_events(start_at);
CREATE INDEX idx_calendar_events_end ON calendar_events(end_at);
CREATE INDEX idx_calendar_events_scope ON calendar_events(scope);
CREATE INDEX idx_calendar_events_department ON calendar_events(department_id);
CREATE INDEX idx_calendar_events_creator ON calendar_events(created_by);
