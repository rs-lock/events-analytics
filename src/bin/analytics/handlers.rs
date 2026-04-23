use std::collections::HashMap;

use actix_web::{
    HttpResponse, get,
    web::{self, Data, Path},
};
use chrono::DateTime;
use clickhouse::Client;
use uuid::Uuid;

use crate::{
    errors::AnalyticsError,
    queries::{ConversionRateQuery, TopProductsQuery, UserActivityQuery},
    response::{
        ConversionRateResponse, EventCounts, ProductActivity, ProductsResponse,
        RealtimeStatsResponse, UserActivityResponse,
    },
    sql::{
        select_global_event_count, select_products_by_period, select_top_products,
        select_user_event_count,
    },
    utils::conversion_rate,
    validation::{validate_metric, validate_period},
};

#[get("/analytics/top-products")]
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

#[get("/analytics/user-activity/{user_id}")]
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

    let limit = q.limit.unwrap_or(50).min(500);
    let offset = q.offset.unwrap_or_default();

    let (all_clicks, all_views, all_purchases, product_clicks, product_views, product_purchases) = tokio::join!(
        select_user_event_count(client.get_ref(), "clicks", user_id, from, to),
        select_user_event_count(client.get_ref(), "views", user_id, from, to),
        select_user_event_count(client.get_ref(), "purchases", user_id, from, to),
        select_products_by_period(client.get_ref(), "clicks", user_id, from, to),
        select_products_by_period(client.get_ref(), "views", user_id, from, to),
        select_products_by_period(client.get_ref(), "purchases", user_id, from, to),
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
            .skip(offset as usize)
            .take(limit as usize)
            .collect(),
    };
    Ok(HttpResponse::Ok().json(resp))
}

#[get("/analytics/conversion-rate")]
pub async fn handle_conversion_rate(
    query: web::Query<ConversionRateQuery>,
    client: Data<Client>,
) -> Result<HttpResponse, AnalyticsError> {
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

    let (c, v, p) = tokio::join!(
        select_global_event_count(client.get_ref(), "clicks", from, to),
        select_global_event_count(client.get_ref(), "views", from, to),
        select_global_event_count(client.get_ref(), "purchases", from, to),
    );

    let clicks = c?;
    let views = v?;
    let purchases = p?;

    let click_to_purchase = conversion_rate(purchases, clicks);
    let view_to_purchase = conversion_rate(purchases, views);

    let r = ConversionRateResponse {
        from: from.to_rfc3339(),
        to: to.to_rfc3339(),
        views,
        clicks,
        purchases,
        view_to_purchase,
        click_to_purchase,
    };

    Ok(HttpResponse::Ok().json(r))
}

#[get("/analytics/realtime-stats")]
pub async fn handle_realtime_stats(client: Data<Client>) -> Result<HttpResponse, AnalyticsError> {
    let now = chrono::Utc::now();
    let last_15m = now - chrono::Duration::minutes(15);
    let last_hour = now - chrono::Duration::hours(1);
    let last_24h = now - chrono::Duration::hours(24);
    let (c15, v15, p15, c1h, v1h, p1h, c24h, v24h, p24h) = tokio::join!(
        select_global_event_count(client.get_ref(), "clicks", last_15m, now),
        select_global_event_count(client.get_ref(), "views", last_15m, now),
        select_global_event_count(client.get_ref(), "purchases", last_15m, now),
        //
        select_global_event_count(client.get_ref(), "clicks", last_hour, now),
        select_global_event_count(client.get_ref(), "views", last_hour, now),
        select_global_event_count(client.get_ref(), "purchases", last_hour, now),
        //
        select_global_event_count(client.get_ref(), "clicks", last_24h, now),
        select_global_event_count(client.get_ref(), "views", last_24h, now),
        select_global_event_count(client.get_ref(), "purchases", last_24h, now),
    );

    let (c15, v15, p15, c1h, v1h, p1h, c24h, v24h, p24h) =
        (c15?, v15?, p15?, c1h?, v1h?, p1h?, c24h?, v24h?, p24h?);

    Ok(HttpResponse::Ok().json(RealtimeStatsResponse {
        last_15m: EventCounts {
            clicks: c15,
            views: v15,
            purchases: p15,
        },
        last_hour: EventCounts {
            clicks: c1h,
            views: v1h,
            purchases: p1h,
        },
        last_24h: EventCounts {
            clicks: c24h,
            views: v24h,
            purchases: p24h,
        },
        timestamp: now.to_rfc3339(),
    }))
}
