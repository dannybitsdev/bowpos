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
    pub primary_color: String,
    pub secondary_color: String,
    pub background_color: String,
    pub font_family: String,
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
        primary_color: "#DEFF9A".to_string(),
        secondary_color: "#141414".to_string(),
        background_color: "#0D0D0D".to_string(),
        font_family: "Inter, sans-serif".to_string(),
        logo_url: "".to_string(),
    }))
}
