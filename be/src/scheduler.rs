use chrono::{Duration, Utc};
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
struct DueSoonCard {
    title: String,
    project_name: String,
    assignee_email: String,
    assignee_name: String,
    due_date: chrono::NaiveDate,
}

#[derive(sqlx::FromRow)]
struct DueScheduleNote {
    title: String,
    user_email: String,
    user_name: String,
    event_date: chrono::NaiveDate,
}

pub fn start(pool: PgPool) {
    tokio::spawn(async move {
        loop {
            // Sleep until next midnight UTC (run once per day)
            let now = Utc::now();
            let tomorrow = (now + Duration::days(1))
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .unwrap();
            let sleep_secs = (tomorrow - now.naive_utc()).num_seconds().max(0) as u64;
            tokio::time::sleep(tokio::time::Duration::from_secs(sleep_secs)).await;

            send_due_date_reminders(&pool).await;
            send_schedule_reminders(&pool).await;
        }
    });
}

async fn send_due_date_reminders(pool: &PgPool) {
    let tomorrow = (Utc::now() + Duration::days(1)).date_naive();

    let cards = sqlx::query_as::<_, DueSoonCard>(
        r#"
        SELECT c.title,
               p.name as project_name,
               u.email as assignee_email,
               u.user_name as assignee_name,
               c.due_date
        FROM cards c
        JOIN projects p ON p.id = c.project_id
        JOIN users u ON u.id = c.assignee_id
        LEFT JOIN board_columns bc ON bc.id = c.column_id
        WHERE c.due_date = $1
          AND u.email IS NOT NULL
          AND (bc.is_done = false OR c.column_id IS NULL)
        "#,
    )
    .bind(tomorrow)
    .fetch_all(pool)
    .await;

    let cards = match cards {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("[SCHEDULER] Failed to fetch due-tomorrow cards: {}", e);
            return;
        }
    };

    tracing::info!(
        "[SCHEDULER] Sending due-date reminders for {} tasks (due {})",
        cards.len(),
        tomorrow
    );

    for card in cards {
        let to = card.assignee_email.clone();
        let name = card.assignee_name.clone();
        let title = card.title.clone();
        let project = card.project_name.clone();
        let due = card.due_date.to_string();
        let result = tokio::task::spawn_blocking(move || {
            crate::api::user::mailer::Mailer::new().send_due_date_reminder_email(
                &to, &name, &title, &project, &due,
            )
        })
        .await;
        if let Ok(Err(e)) = result {
            tracing::error!("[SCHEDULER] Due-date reminder failed for {}: {}", card.assignee_email, e);
        }
    }
}

async fn send_schedule_reminders(pool: &PgPool) {
    let tomorrow = (Utc::now() + Duration::days(1)).date_naive();

    let notes = sqlx::query_as::<_, DueScheduleNote>(
        r#"
        SELECT
            ce.title,
            u.email  AS user_email,
            u.user_name AS user_name,
            DATE(ce.start_at) AS event_date
        FROM calendar_events ce
        JOIN users u ON u.id = ce.created_by
        WHERE ce.scope = 'personal'
          AND DATE(ce.start_at) = $1
          AND u.email IS NOT NULL
        "#,
    )
    .bind(tomorrow)
    .fetch_all(pool)
    .await;

    let notes = match notes {
        Ok(n) => n,
        Err(e) => {
            tracing::error!("[SCHEDULER] Failed to fetch tomorrow's schedule notes: {}", e);
            return;
        }
    };

    tracing::info!(
        "[SCHEDULER] Sending schedule reminders for {} notes (due {})",
        notes.len(),
        tomorrow
    );

    for note in notes {
        let to = note.user_email.clone();
        let name = note.user_name.clone();
        let title = note.title.clone();
        let date = note.event_date.to_string();
        let result = tokio::task::spawn_blocking(move || {
            crate::api::user::mailer::Mailer::new()
                .send_schedule_reminder_email(&to, &name, &title, &date)
        })
        .await;
        if let Ok(Err(e)) = result {
            tracing::error!("[SCHEDULER] Schedule reminder failed for {}: {}", note.user_email, e);
        }
    }
}
