use crate::{api::user::vmail::MailcowConfig, db::Db};
use axum::Extension;
use tower::ServiceBuilder;

pub fn add_extensions(
    db: Db,
    mailcow: Option<MailcowConfig>,
) -> ServiceBuilder<
    tower::layer::util::Stack<
        Extension<Option<MailcowConfig>>,
        tower::layer::util::Stack<Extension<Db>, tower::layer::util::Identity>,
    >,
> {
    ServiceBuilder::new()
        .layer(Extension(db))
        .layer(Extension(mailcow))
}
