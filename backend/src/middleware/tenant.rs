use axum::{
    extract::Request,
    http::StatusCode,
    response::Response,
};

#[derive(Clone, Default)]
#[allow(dead_code)]
pub struct TenantContext {
    pub tenant_id: Option<String>,
    pub slug: Option<String>,
}

pub async fn tenant_middleware(
    mut request: Request,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let tenant_id = request.headers().get("x-tenant-id").and_then(|value| value.to_str().ok()).map(str::to_string);
    let slug = request.headers().get("x-tenant-slug").and_then(|value| value.to_str().ok()).map(str::to_string);

    request.extensions_mut().insert(TenantContext { tenant_id, slug });
    Ok(next.run(request).await)
}
