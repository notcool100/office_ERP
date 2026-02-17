-- Link positions to departments

ALTER TABLE positions
    ADD COLUMN IF NOT EXISTS department_id UUID REFERENCES departments(id);

CREATE INDEX IF NOT EXISTS idx_positions_department ON positions(department_id);
