use axum::{http::StatusCode, Json};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct DashboardMetrics {
    pub ventas_totales: f64,
    pub costo: f64,
    pub utilidad_bruta: f64,
    pub ordenes: u64,
    pub ticket_promedio: f64,
    pub plato_mas_vendido: String,
    pub ventas_por_categoria: HashMap<String, f64>,
}

#[derive(Serialize)]
pub struct UiConfigResponse {
    pub color_primario: String,
    pub color_secundario: String,
    pub color_fondo: String,
    pub tipografia: String,
    pub logo_url: String,
}

pub async fn get_dashboard_metrics() -> Result<Json<DashboardMetrics>, StatusCode> {
    let mut ventas_por_categoria = HashMap::new();
    ventas_por_categoria.insert("Platos principales".to_string(), 142000.0);
    ventas_por_categoria.insert("Bebidas".to_string(), 38000.0);
    ventas_por_categoria.insert("Postres".to_string(), 18500.0);

    let metrics = DashboardMetrics {
        ventas_totales: 248500.0,
        costo: 102400.0,
        utilidad_bruta: 146100.0,
        ordenes: 184,
        ticket_promedio: 1350.0,
        plato_mas_vendido: "Bandeja Paisa".to_string(),
        ventas_por_categoria,
    };

    Ok(Json(metrics))
}

pub async fn get_ui_config() -> Result<Json<UiConfigResponse>, StatusCode> {
    Ok(Json(UiConfigResponse {
        color_primario: "#d97706".to_string(),
        color_secundario: "#1f2937".to_string(),
        color_fondo: "#fef3c7".to_string(),
        tipografia: "Inter".to_string(),
        logo_url: "https://example.com/logo.png".to_string(),
    }))
}
