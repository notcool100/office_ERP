-- A second "Create Role" navigation_items row existed independently of the
-- migration-seeded Role/Create Role pair (parent_id = NULL, so it wasn't a
-- child of the "Role" item removed in 20260908030000 and didn't cascade
-- with it). Clean up any remaining row under this path regardless of how
-- it was created.
DELETE FROM navigation_items WHERE path LIKE '/admin/settings/role%';
