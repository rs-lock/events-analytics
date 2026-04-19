use std::collections::HashMap;

use actix_web::{
    HttpResponse,
    web::{self, Data, Path},
};
use chrono::DateTime;
use clickhouse::Client;
use uuid::Uuid;

use crate::{
    errors::AnalyticsError,
    queries::{TopProductsQuery, UserActivityQuery},
    response::{EventCounts, ProductActivity, ProductsResponse, UserActivityResponse},
    sql::{select_top_products, select_top_products_by_period, select_user_event_count},
    validation::{validate_metric, validate_period},
};

pub async fn handle_top_products(
    query: web::Query<TopProductsQuery>,
    client: Data<Client>,
) -> Result<HttpResponse, AnalyticsError> {
    let query = query.into_inner();
    let metric = validate_metric(&query.metric.ok_or(AnalyticsError::MissingMetric)?)
        .ok_or(AnalyticsError::InvalidMetric)?;

    let period = validate_period(&query.period.ok_or(AnalyticsError::MissingPeriod)?)
        .ok_or(AnalyticsError::InvalidPeriod)?;

    let lim = query.limit.unwrap_or(10) as u32;

    let products = select_top_products(client.get_ref(), lim, &metric, period).await?;

    Ok(HttpResponse::Ok().json(ProductsResponse {
        period: period.as_param().to_string(),
        metric: metric.table().to_string(),
        items: products.iter().map(|p| p.into()).collect(),
    }))
}

pub async fn handle_user_activity(
    path: Path<String>,
    query: web::Query<UserActivityQuery>,
    client: Data<Client>,
) -> Result<HttpResponse, AnalyticsError> {
    let Ok(user_id) = Uuid::parse_str(&path.into_inner()) else {
        return Err(AnalyticsError::InvalidUserID);
    };

    let q = query.into_inner();

    let Some(from) = q.from else {
        return Err(AnalyticsError::MissingFrom);
    };

    let Some(to) = q.to else {
        return Err(AnalyticsError::MissingTo);
    };

    let Ok(from) = DateTime::parse_from_rfc3339(&from).map(|d| d.to_utc()) else {
        return Err(AnalyticsError::InvalidFrom);
    };

    let Ok(to) = DateTime::parse_from_rfc3339(&to).map(|d| d.to_utc()) else {
        return Err(AnalyticsError::InvalidTo);
    };

    let (all_clicks, all_views, all_purchases, product_clicks, product_views, product_purchases) = tokio::join!(
        select_user_event_count(client.get_ref(), "clicks", user_id, from, to),
        select_user_event_count(client.get_ref(), "views", user_id, from, to),
        select_user_event_count(client.get_ref(), "purchases", user_id, from, to),
        select_top_products_by_period(client.get_ref(), "clicks", user_id, from, to),
        select_top_products_by_period(client.get_ref(), "views", user_id, from, to),
        select_top_products_by_period(client.get_ref(), "purchases", user_id, from, to),
    );

    let event_clicks = all_clicks?;
    let event_views = all_views?;
    let event_purchases = all_purchases?;

    let product_activity_clicks = product_clicks?;
    let product_activity_views = product_views?;
    let product_activity_purchases = product_purchases?;

    let mut map: HashMap<Uuid, EventCounts> = HashMap::new();

    for v in &product_activity_clicks {
        map.entry(v.product_id).or_default().clicks = v.count;
    }
    for v in &product_activity_views {
        map.entry(v.product_id).or_default().views = v.count;
    }
    for v in &product_activity_purchases {
        map.entry(v.product_id).or_default().purchases = v.count;
    }

    let resp = UserActivityResponse {
        user_id,
        from: from.to_string(),
        to: to.to_string(),
        events: EventCounts {
            clicks: event_clicks,
            views: event_views,
            purchases: event_purchases,
        },
        top_products: map
            .into_iter()
            .map(|(product_id, event_counts)| ProductActivity {
                product_id,
                event_counts,
            })
            .collect(),
    };
    Ok(HttpResponse::Ok().json(resp))
}
