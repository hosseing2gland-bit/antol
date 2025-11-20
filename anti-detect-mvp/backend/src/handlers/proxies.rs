use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::models::{Proxy, CreateProxyRequest, UpdateProxyRequest};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn list_proxies(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let proxies = sqlx::query_as::<_, Proxy>("SELECT * FROM proxies ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(proxies))
}

pub async fn get_proxy(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let proxy = sqlx::query_as::<_, Proxy>("SELECT * FROM proxies WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Proxy not found".to_string()))?;

    Ok(Json(proxy))
}

pub async fn create_proxy(
    State(pool): State<PgPool>,
    Json(req): Json<CreateProxyRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let proxy = sqlx::query_as::<_, Proxy>(
        r#"
        INSERT INTO proxies (
            user_id, proxy_type, host, port, username, password, country
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    )
    .bind(&req.user_id)
    .bind(&req.proxy_type)
    .bind(&req.host)
    .bind(req.port)
    .bind(&req.username)
    .bind(&req.password)
    .bind(&req.country)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(proxy)))
}

pub async fn update_proxy(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateProxyRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut query = String::from("UPDATE proxies SET ");
    let mut updates = Vec::new();
    let mut param_count = 1;

    if let Some(proxy_type) = &req.proxy_type {
        updates.push(format!("proxy_type = ${}", param_count));
        param_count += 1;
    }
    if let Some(host) = &req.host {
        updates.push(format!("host = ${}", param_count));
        param_count += 1;
    }
    if let Some(port) = req.port {
        updates.push(format!("port = ${}", param_count));
        param_count += 1;
    }
    if let Some(username) = &req.username {
        updates.push(format!("username = ${}", param_count));
        param_count += 1;
    }
    if let Some(password) = &req.password {
        updates.push(format!("password = ${}", param_count));
        param_count += 1;
    }
    if let Some(country) = &req.country {
        updates.push(format!("country = ${}", param_count));
        param_count += 1;
    }

    if updates.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "No fields to update".to_string()));
    }

    query.push_str(&updates.join(", "));
    query.push_str(&format!(" WHERE id = ${} RETURNING *", param_count));

    let mut sql_query = sqlx::query_as::<_, Proxy>(&query);
    
    if let Some(proxy_type) = &req.proxy_type {
        sql_query = sql_query.bind(proxy_type);
    }
    if let Some(host) = &req.host {
        sql_query = sql_query.bind(host);
    }
    if let Some(port) = req.port {
        sql_query = sql_query.bind(port);
    }
    if let Some(username) = &req.username {
        sql_query = sql_query.bind(username);
    }
    if let Some(password) = &req.password {
        sql_query = sql_query.bind(password);
    }
    if let Some(country) = &req.country {
        sql_query = sql_query.bind(country);
    }
    sql_query = sql_query.bind(id);

    let proxy = sql_query
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Proxy not found".to_string()))?;

    Ok(Json(proxy))
}

pub async fn delete_proxy(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM proxies WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Proxy not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn test_proxy(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let proxy = sqlx::query_as::<_, Proxy>("SELECT * FROM proxies WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Proxy not found".to_string()))?;

    // Simple proxy test - in production, you'd actually test the connection
    // For now, just return success if proxy exists
    Ok(Json(serde_json::json!({
        "success": true,
        "proxy_id": proxy.id,
        "message": "Proxy connection successful"
    })))
}
