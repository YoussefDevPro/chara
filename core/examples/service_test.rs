use charac::db::DB;
use charac::init;
use charac::models::*;
use charac::service::user::{AuthMethod, Session as UserSession, UserService};
use std::time::{Duration, Instant};
use surrealdb::types::RecordId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup Environment
    dotenvy::dotenv().ok();

    println!("==================================================");
    println!("🧪  STATUS: RUNNING IN TEST / BENCHMARK MODE");
    println!("📂  CRATE: CORE (@core)");
    println!("🔍  DEBUG: DBG! ENABLED FOR ALL DB OPS");
    println!("==================================================\n");

    println!("[1/7] 🔧 Initializing Database...");
    init().await;
    println!("✅ Database connection established.\n");

    // 2. Prepare Mock Data
    println!("[2/7] 🛠️  Preparing Mock Data (User & Session)...");
    let setup_start = Instant::now();
    let user_id = UserId(RecordId::parse_simple("user:bench_user").unwrap());

    // Clean start
    let cleanup_res: Vec<surrealdb::types::Value> = DB
        .query("DELETE $user; DELETE base WHERE owner = $user;")
        .bind(("user", user_id.clone()))
        .await?
        .take(0)?;
    dbg!(&cleanup_res);

    // Create Test User
    let user_res: Option<User> = DB.query("CREATE $id SET first_name = 'Bench', last_name = 'Tester', email = 'bench@chara.local', role = 'admin', is_deleted = false;")
        .bind(("id", user_id.clone()))
        .await?
        .take(0)?;
    dbg!(&user_res);

    // Create Session
    let raw_token = "bench_secret_token";
    let ip = "127.0.0.1";
    let agent = "Chara-Bench-Tool/1.0";
    let session_res: Option<Session> = DB.query("CREATE session SET user = $user, token = crypto::sha512($tokenn), ip = $ip, user_agent = $agent, expires_at = time::now() + 1h;")
        .bind(("user", user_id.clone()))
        .bind(("tokenn", raw_token))
        .bind(("ip", ip))
        .bind(("agent", agent))
        .await?
        .take(0)?;
    dbg!(&session_res);

    let setup_dur = setup_start.elapsed();

    // 3. Benchmark UserService
    println!("[3/7] 👤 Benchmarking UserService...");
    let login_start = Instant::now();
    let mut user_service = UserService::login(AuthMethod::Session(UserSession {
        token: raw_token.to_string(),
        ip: ip.to_string(),
        agent: agent.to_string(),
    }))
    .await?;
    let login_dur = login_start.elapsed();
    dbg!(&user_service.user);

    let create_base_start = Instant::now();
    let base = user_service.create_base("Bench Base".into()).await?;
    let create_base_dur = create_base_start.elapsed();
    let base_id = base.id.clone().unwrap();
    dbg!(&base);

    // 4. Benchmark BaseService
    println!("[4/7] 📦 Benchmarking BaseService...");
    let open_base_start = Instant::now();
    user_service.open_base(base_id.clone()).await?;
    let open_base_dur = open_base_start.elapsed();

    let base_service = user_service.current_base.as_ref().unwrap();
    dbg!(&base_service.base);

    let create_table_start = Instant::now();
    let table = base_service.create_table("BenchTable".into()).await?;
    let create_table_dur = create_table_start.elapsed();
    let table_id = table.id.clone().unwrap();
    dbg!(&table);

    // 5. Benchmark TableService
    println!("[5/7] 📊 Benchmarking TableService...");
    let open_table_start = Instant::now();
    let table_service = base_service.open_table(table_id.clone()).await?;
    let open_table_dur = open_table_start.elapsed();
    dbg!(&table_service.table);

    let create_field_start = Instant::now();
    let field = table_service
        .create_field(InsertField {
            name: "BenchmarkField".into(),
            description: Some("Speed test".into()),
            is_primary: false,
            is_nullable: true,
            is_unique: false,
            order: 1,
            config: FieldConfig::Text(TextConfig::SingleLine {
                default: None,
                max_length: 100,
            }),
        })
        .await?;
    let create_field_dur = create_field_start.elapsed();
    dbg!(&field);

    // 6. Visualization Table
    println!("\n[6/7] 🏁 Results Summary:");
    println!(
        "+--------------------------------+----------------------------+--------------------+"
    );
    println!(
        "| Service Operation              | Result Detail              | Execution Time     |"
    );
    println!(
        "+--------------------------------+----------------------------+--------------------+"
    );
    print_row("Mock Setup (DB Init)", "User & Session Created", setup_dur);
    print_row(
        "UserService::login",
        &format!("Auth as {}", user_service.user.first_name),
        login_dur,
    );
    print_row("UserService::create_base", &base.name, create_base_dur);
    print_row(
        "UserService::open_base",
        "BaseService Initialized",
        open_base_dur,
    );
    print_row("BaseService::create_table", &table.name, create_table_dur);
    print_row(
        "BaseService::open_table",
        "TableService Initialized",
        open_table_dur,
    );
    print_row("TableService::create_field", &field.name, create_field_dur);
    println!(
        "+--------------------------------+----------------------------+--------------------+"
    );

    // 7. Cleanup
    println!("\n[7/7] 🧹 Cleaning up benchmark data...");
    let final_cleanup: Vec<surrealdb::types::Value> = DB
        .query("DELETE $user; DELETE $base; DELETE $table; DELETE field WHERE table = $table;")
        .bind(("user", user_id))
        .bind(("base", base_id))
        .bind(("table", table_id))
        .await?
        .take(0)?;
    dbg!(&final_cleanup);

    println!("✅ Benchmarks completed successfully.");

    Ok(())
}

fn print_row(op: &str, detail: &str, dur: Duration) {
    println!("| {:<30} | {:<26} | {:>18?} |", op, detail, dur);
}
