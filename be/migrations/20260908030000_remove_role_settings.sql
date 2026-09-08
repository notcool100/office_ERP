-- The Settings > Role section was never wired up (no backend, no persisted
-- data — the frontend page rendered a hardcoded list and its "create role"
-- form just logged to the console). RBAC is actually driven by Department
-- and Position (role_permissions.department_id/position_id), so this was
-- vestigial and confusing next to the real Permissions page. Removing the
-- nav item cascades to its "Create Role" child and any role_permissions
-- rows granting access to either.
DELETE FROM navigation_items WHERE path = '/admin/settings/role';
