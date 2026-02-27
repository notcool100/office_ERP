use crate::db::{Db, VmailDb};
use tower::ServiceBuilder;
use axum::Extension;

pub fn add_extensions(db: Db, vmail_db: VmailDb) -> ServiceBuilder<tower::layer::util::Stack<Extension<VmailDb>, tower::layer::util::Stack<Extension<Db>, tower::layer::util::Identity>>> {
    ServiceBuilder::new()
        .layer(Extension(db))
        .layer(Extension(vmail_db))
}
