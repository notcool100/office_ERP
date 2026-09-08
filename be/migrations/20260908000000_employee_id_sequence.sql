-- Auto-increment employee_id (format EMP-XXX) instead of requiring manual entry.
CREATE SEQUENCE IF NOT EXISTS employee_id_seq;

-- Seed the sequence past the highest existing EMP-### number so new ids never collide.
SELECT setval(
    'employee_id_seq',
    COALESCE(
        (SELECT MAX(substring(employee_id from 'EMP-(\d+)')::int)
         FROM employees WHERE employee_id ~ '^EMP-\d+$'),
        0
    )
);
