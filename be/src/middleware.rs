use axum::Extension;

use crate::db::Db;

pub fn add_extensions(db: Db) -> Extension<Db> {
    Extension(db)
}
