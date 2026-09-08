use be::api::auth::password::hash_password;
use chrono::Utc;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(about = "Seed the database with an initial admin user, roles, and reference data")]
struct Args {
    /// Password for the seeded admin user
    #[arg(long)]
    password: String,

    /// Username for the seeded admin user
    #[arg(long, default_value = "admin")]
    username: String,

    /// Email for the seeded admin user
    #[arg(long, default_value = "admin@example.com")]
    email: String,

    /// Phone for the seeded admin user
    #[arg(long, default_value = "9800000000")]
    phone: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let args = Args::parse();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    if sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = $1")
        .bind(&args.email)
        .fetch_one(&pool)
        .await?
        > 0
    {
        println!(
            "User with email {} already exists. Nothing to do.",
            args.email
        );
        return Ok(());
    }

    // Departments and positions are already seeded by the RBAC migration
    // (20250103000000_rbac_system.sql), which also grants them full access
    // to every navigation item. We just need to look their ids up.
    let department_id: Uuid =
        sqlx::query_scalar("SELECT id FROM departments WHERE name = 'Administration'")
            .fetch_one(&pool)
            .await?;
    let position_id: Uuid =
        sqlx::query_scalar("SELECT id FROM positions WHERE name = 'System Administrator'")
            .fetch_one(&pool)
            .await?;

    let hashed = hash_password(&args.password)?;
    let person_id = Uuid::new_v4();
    let user_id = Uuid::new_v4();
    let employee_id = Uuid::new_v4();

    let mut tx = pool.begin().await?;

    sqlx::query("INSERT INTO persons (id, first_name, middle_name, last_name) VALUES ($1, 'Admin', '', 'User')")
        .bind(person_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "INSERT INTO person_contacts (id, person_id, email, phone) VALUES ($1, $2, $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(person_id)
    .bind(&args.email)
    .bind(&args.phone)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO users (id, user_name, email, phone, password_hash, person_id, is_admin) \
         VALUES ($1, $2, $3, $4, $5, $6, true)",
    )
    .bind(user_id)
    .bind(&args.username)
    .bind(&args.email)
    .bind(&args.phone)
    .bind(&hashed)
    .bind(person_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO employees (id, employee_id, person_id, department_id, position_id, hire_date, employment_type, status) \
         VALUES ($1, 'ADMIN-0001', $2, $3, $4, $5, 'Full-time', 'active')",
    )
    .bind(employee_id)
    .bind(person_id)
    .bind(department_id)
    .bind(position_id)
    .bind(Utc::now().date_naive())
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    println!("Seeded admin user:");
    println!("  username: {}", args.username);
    println!("  email:    {}", args.email);
    println!("  password: (as provided)");
    println!("Linked to an Admin Employee in the Administration department / System Administrator position.");

    Ok(())
}
