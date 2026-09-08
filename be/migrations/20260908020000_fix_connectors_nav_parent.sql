-- The Connectors nav item was inserted with parent_id = NULL, making it a
-- detached root-level item instead of a child of the Settings folder (every
-- other Settings sub-page links parent_id to the '/admin/settings' item).
UPDATE navigation_items
SET parent_id = (SELECT id FROM navigation_items WHERE path = '/admin/settings')
WHERE path = '/admin/settings/connectors';
