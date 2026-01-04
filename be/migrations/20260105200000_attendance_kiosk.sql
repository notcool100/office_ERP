-- Add face_descriptor to employees table
ALTER TABLE employees 
ADD COLUMN IF NOT EXISTS face_descriptor TEXT; -- Storing JSON string of the descriptor

-- Add check-in metadata to attendance table
ALTER TABLE attendance_records 
ADD COLUMN IF NOT EXISTS check_in_image TEXT,
ADD COLUMN IF NOT EXISTS check_in_method TEXT DEFAULT 'MANUAL',
ADD COLUMN IF NOT EXISTS check_in_lat DECIMAL(10, 8),
ADD COLUMN IF NOT EXISTS check_in_long DECIMAL(11, 8),
ADD COLUMN IF NOT EXISTS check_out_lat DECIMAL(10, 8),
ADD COLUMN IF NOT EXISTS check_out_long DECIMAL(11, 8);
