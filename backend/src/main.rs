mod handlers;
mod middleware;

use axum::{
    extract::State,
    http::{Method, StatusCode},
    middleware as axum_middleware,
    routing::get,
    Json, Router,
};
use handlers::{dashboard::{get_dashboard_metrics, get_ui_config}, sedes::sedes_router};
use middleware::tenant::{tenant_middleware, TenantContext};
use serde::Serialize;
use sqlx::PgPool;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://appuser:changeme@db:5432/appdb".to_string());
    let pool = PgPool::connect(&database_url).await.expect("database connection");

    let migrations_dir = env::var("MIGRATIONS_DIR")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            if std::path::Path::new("migrations").exists() {
                "migrations".to_string()
            } else {
                "/app/migrations".to_string()
            }
        });

    let migrations = std::fs::read_dir(&migrations_dir).expect("migrations dir");
    let mut paths: Vec<_> = migrations
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension().and_then(|ext| ext.to_str()) == Some("sql")).then_some(path)
        })
        .collect();
    paths.sort();

    for path in paths {
        let sql = std::fs::read_to_string(&path).expect("migration sql");
        for statement in sql.split(";") {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                sqlx::query(stmt).execute(&pool).await.expect("apply migration");
            }
        }
    }

    seed_initial_data(&pool).await.expect("seed data");

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/dashboard", get(get_dashboard_metrics))
        .route("/api/config/ui", get(get_ui_config))
        .nest("/api", sedes_router())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods([Method::GET, Method::POST]))
        .layer(axum_middleware::from_fn(tenant_middleware))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("bind tcp listener");

    println!("Backend listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.expect("axum server");
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok", service: "backend" })
}

async fn seed_initial_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM usuarios")
        .fetch_one(pool)
        .await?;

    if count == 0 {
        let tenant_id = Uuid::new_v4();
        let sede_id = Uuid::new_v4();
        let password_hash = bcrypt::hash("admin123", 10).expect("hash password");

        sqlx::query("INSERT INTO tenants (id, name, slug, created_at) VALUES ($1, $2, $3, NOW())")
            .bind(tenant_id)
            .bind("Sabor & Raíz")
            .bind("sabor-y-raiz")
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO sedes (id, tenant_id, nombre, direccion, ciudad, configuracion_impresora) VALUES ($1, $2, $3, $4, $5, '{}'::jsonb)")
            .bind(sede_id)
            .bind(tenant_id)
            .bind("Sede Principal")
            .bind("Calle 10 # 20-30")
            .bind("Bogotá")
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO config_ui (tenant_id, color_primario, color_secundario, color_fondo, tipografia, logo_url) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(tenant_id)
            .bind("#d97706")
            .bind("#1f2937")
            .bind("#fef3c7")
            .bind("Inter")
            .bind("https://example.com/logo.png")
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO usuarios (id, tenant_id, sede_id, nombre, email, password_hash, rol) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(Uuid::new_v4())
            .bind(tenant_id)
            .bind(sede_id)
            .bind("Administrador")
            .bind("admin@pos.com")
            .bind(password_hash)
            .bind("SUPER_ADMIN")
            .execute(pool)
            .await?;

        sqlx::query("INSERT INTO productos (id, tenant_id, nombre, precio, stock, imagen_url) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(Uuid::new_v4())
            .bind(tenant_id)
            .bind("Bandeja Paisa")
            .bind(28000.0)
            .bind(45)
            .bind("https://example.com/bandeja-paisa.png")
            .execute(pool)
            .await?;
    }

    Ok(())
}
