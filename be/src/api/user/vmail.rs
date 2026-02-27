use crate::db::VmailDb;
use sha2::{Digest, Sha512};
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
use anyhow::{Result, anyhow};
use chrono::Utc;

pub struct VmailService;

impl VmailService {
    pub async fn create_mailbox(
        db: &VmailDb,
        email: &str,
        password: &str,
        name: &str,
    ) -> Result<()> {
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid email address"));
        }
        let username = parts[0];
        let domain = parts[1];
        
        // Check if user already exists
        let exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM mailbox WHERE username = ?")
            .bind(email)
            .fetch_one(db)
            .await?;
            
        if exists > 0 {
            return Ok(());
        }

        let hashed_password = Self::ssha512_hash(password);
        let timestamp = Utc::now().format("%Y.%m.%d.%H.%M.%S").to_string();
        
        // Maildir format: domain/first_char/second_char/third_char/username-timestamp/
        let mut chars = username.chars();
        let c1 = chars.next().unwrap_or('?').to_string();
        let c2 = chars.next().unwrap_or('?').to_string();
        let c3 = chars.next().unwrap_or('?').to_string();
        let maildir = format!("{}/{}/{}/{}/{}-{}/", domain, c1, c2, c3, username, timestamp);

        // Insert into mailbox
        sqlx::query(
            r#"
            INSERT INTO mailbox (
                username, password, name, domain, maildir,
                storagebasedirectory, storagenode, mailboxformat, mailboxfolder,
                isadmin, isglobaladmin, enablesmtp, enablesmtpsecured,
                enablepop3, enablepop3secured, enablepop3tls,
                enableimap, enableimapsecured, enableimaptls,
                enabledeliver, enablelda, enablemanagesieve,
                enablemanagesievesecured, enablesieve, enablesievesecured,
                enablesievetls, enableinternal, enabledoveadm,
                `enablelib-storage`, `enablequota-status`, `enableindexer-worker`,
                enablelmtp, enabledsync, enablesogo,
                enablesogowebmail, enablesogocalendar, enablesogoactivesync,
                active, created, modified, expired, birthday
            ) VALUES (
                ?, ?, ?, ?, ?,
                '/var/vmail', 'vmail1', 'maildir', 'Maildir',
                0, 0, 1, 1,
                1, 1, 1,
                1, 1, 1,
                1, 1, 1,
                1, 1, 1,
                1, 1, 1,
                1, 1, 1,
                1, 1, 1,
                'y', 'y', 'y',
                1, NOW(), NOW(), '9999-12-31 01:01:01', '0001-01-01'
            )
            "#,
        )
        .bind(email)
        .bind(hashed_password)
        .bind(name)
        .bind(domain)
        .bind(maildir)
        .execute(db)
        .await?;

        // Insert into forwardings (self-referential)
        sqlx::query(
            r#"
            INSERT INTO forwardings (
                address, forwarding, domain, dest_domain,
                is_maillist, is_list, is_forwarding, is_alias, active
            ) VALUES (?, ?, ?, ?, 0, 0, 1, 0, 1)
            "#,
        )
        .bind(email)
        .bind(email)
        .bind(domain)
        .bind(domain)
        .execute(db)
        .await?;

        Ok(())
    }

    fn ssha512_hash(password: &str) -> String {
        let mut salt = [0u8; 8];
        rand::thread_rng().fill_bytes(&mut salt);
        
        let mut hasher = Sha512::new();
        hasher.update(password.as_bytes());
        hasher.update(&salt);
        let hash = hasher.finalize();
        
        let mut combined = hash.to_vec();
        combined.extend_from_slice(&salt);
        
        format!("{{SSHA512}}{}", general_purpose::STANDARD.encode(combined))
    }
}
