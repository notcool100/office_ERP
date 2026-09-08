-- Track the method used for check-out (mirrors check_in_method), so a
-- face-recognition check-out can be told apart from a manual one and its
-- location can be audited the same way check-in already is.
ALTER TABLE attendance_records
ADD COLUMN IF NOT EXISTS check_out_method TEXT DEFAULT 'MANUAL';
