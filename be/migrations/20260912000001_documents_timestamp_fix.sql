-- The documents/document_folders columns were created as TIMESTAMPTZ, but
-- their Rust structs (DocumentMeta/FolderMeta) decode into NaiveDateTime —
-- matching the TIMESTAMP (no tz) convention used by media_assets elsewhere
-- in this codebase. Align the column type instead of the struct type, since
-- every other document/timestamp column in this app follows that pattern.

ALTER TABLE document_folders
    ALTER COLUMN created_at TYPE TIMESTAMP,
    ALTER COLUMN updated_at TYPE TIMESTAMP;

ALTER TABLE documents
    ALTER COLUMN created_at TYPE TIMESTAMP,
    ALTER COLUMN updated_at TYPE TIMESTAMP;
