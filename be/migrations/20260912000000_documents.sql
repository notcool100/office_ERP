-- Drive-style document storage shared by three surfaces: company-wide
-- documents (PAN, registration, ...), per-client documents (TOR, contracts,
-- ...), and per-employee documents (contracts, ...). One schema backs all
-- three; which surface a row belongs to is `category`, and the matching
-- owner column is the only one populated (company docs have no owner).
-- Folder-name uniqueness within a parent is enforced at the application
-- layer, not here (same choice made for meeting_attachments).

CREATE TABLE document_folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category VARCHAR(20) NOT NULL CHECK (category IN ('company', 'client', 'employee')),
    client_id UUID REFERENCES clients(id) ON DELETE CASCADE,
    employee_id UUID REFERENCES employees(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES document_folders(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT document_folders_owner_matches_category CHECK (
        (category = 'company' AND client_id IS NULL AND employee_id IS NULL) OR
        (category = 'client' AND client_id IS NOT NULL AND employee_id IS NULL) OR
        (category = 'employee' AND employee_id IS NOT NULL AND client_id IS NULL)
    )
);

CREATE INDEX idx_document_folders_scope ON document_folders(category, client_id, employee_id, parent_id);

CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category VARCHAR(20) NOT NULL CHECK (category IN ('company', 'client', 'employee')),
    client_id UUID REFERENCES clients(id) ON DELETE CASCADE,
    employee_id UUID REFERENCES employees(id) ON DELETE CASCADE,
    folder_id UUID REFERENCES document_folders(id) ON DELETE CASCADE,
    doc_type VARCHAR(50),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_name VARCHAR(255) NOT NULL,
    content_type VARCHAR(120) NOT NULL,
    file_size BIGINT NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    expiry_date DATE,
    uploaded_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT documents_owner_matches_category CHECK (
        (category = 'company' AND client_id IS NULL AND employee_id IS NULL) OR
        (category = 'client' AND client_id IS NOT NULL AND employee_id IS NULL) OR
        (category = 'employee' AND employee_id IS NOT NULL AND client_id IS NULL)
    )
);

CREATE INDEX idx_documents_scope ON documents(category, client_id, employee_id, folder_id);
CREATE INDEX idx_documents_uploaded_by ON documents(uploaded_by);

-- Navigation: a new top-level "Documents" parent with Company/Client
-- children. Employee documents reuse the existing /admin/hr/employee
-- permission (accessed from the employee view page, not a nav item).
INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES ('Documents', '/admin/documents', 'FileText', NULL, 9)
ON CONFLICT (path) DO NOTHING;

INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Company Documents', '/admin/documents/company', 'Building2',
    (SELECT id FROM navigation_items WHERE path = '/admin/documents'), 1
)
ON CONFLICT (path) DO NOTHING;

INSERT INTO navigation_items (name, path, icon, parent_id, display_order)
VALUES (
    'Client Documents', '/admin/documents/client', 'Briefcase',
    (SELECT id FROM navigation_items WHERE path = '/admin/documents'), 2
)
ON CONFLICT (path) DO NOTHING;

-- Default grants: Administration department + System Administrator position
-- get full CRUD, same as every other new module. Anyone else is assigned
-- from Settings > Permissions.
INSERT INTO role_permissions (department_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT d.id, n.id, true, true, true, true
FROM departments d
CROSS JOIN navigation_items n
WHERE d.name = 'Administration'
AND n.path IN ('/admin/documents', '/admin/documents/company', '/admin/documents/client')
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;

INSERT INTO role_permissions (position_id, navigation_item_id, can_create, can_read, can_update, can_delete)
SELECT p.id, n.id, true, true, true, true
FROM positions p
CROSS JOIN navigation_items n
WHERE p.name = 'System Administrator'
AND n.path IN ('/admin/documents', '/admin/documents/company', '/admin/documents/client')
ON CONFLICT (department_id, position_id, navigation_item_id) DO NOTHING;
