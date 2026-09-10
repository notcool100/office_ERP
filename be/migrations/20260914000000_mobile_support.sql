-- Support tables for the Flutter companion app.
--
-- The web ERP delivers notifications over a live WebSocket only, which is
-- fine for a dashboard that is open all day but useless for a phone: the
-- socket is gone the moment the app is backgrounded. These three tables
-- give the mobile client a durable inbox, somewhere to register for push,
-- and a weekly schedule to show on the profile screen.

-- A delivered notification, kept per recipient so read state is per-user
-- rather than per-event. `kind` drives the icon and the tap target in the
-- app; `entity_type`/`entity_id` say what to open.
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    kind VARCHAR(40) NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT,
    entity_type VARCHAR(40),
    entity_id UUID,
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_notifications_user_created ON notifications (user_id, created_at DESC);
CREATE INDEX idx_notifications_user_unread ON notifications (user_id) WHERE read_at IS NULL;

-- One row per installed app instance. The FCM registration token is the
-- natural key: a token can migrate between users when a phone is handed
-- over or a second account signs in, so an upsert on `token` reassigns
-- `user_id` rather than leaving a stale row that would push someone
-- else's notifications to this device.
CREATE TABLE device_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT NOT NULL UNIQUE,
    platform VARCHAR(20) NOT NULL DEFAULT 'android',
    device_name VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_device_tokens_user ON device_tokens (user_id);

-- Weekly working hours per employee. Rows are optional: an employee with
-- no rows falls back to the company default in the service layer, so this
-- only needs filling in for people who work a non-standard week.
-- day_of_week follows ISO-8601 (1 = Monday .. 7 = Sunday) to match
-- chrono's Weekday::number_from_monday().
CREATE TABLE employee_schedules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id UUID NOT NULL REFERENCES employees(id) ON DELETE CASCADE,
    day_of_week SMALLINT NOT NULL CHECK (day_of_week BETWEEN 1 AND 7),
    start_time TIME,
    end_time TIME,
    is_working BOOLEAN NOT NULL DEFAULT true,
    UNIQUE (employee_id, day_of_week)
);
