use event_analytics::response::UserActivityResponse;
use std::time::Duration;
use tokio::task::JoinSet;
use uuid::Uuid;

#[tokio::test]
async fn test_ingestion_to_analytics_flow() {
    let client = reqwest::Client::new();
    let user_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let before_2h = now - chrono::Duration::hours(2);

    let mut set = JoinSet::new();
    for time in [now, before_2h] {
        let client = client.clone();
        set.spawn(async move {
            client
                .post("http://localhost:8080/api/v1/events/view")
                .json(&serde_json::json!({
                    "user_id": user_id,
                    "product_id": "660e8400-e29b-41d4-a716-446655440001",
                    "session_id": "770e8400-e29b-41d4-a716-446655440002",
                    "timestamp": time.to_rfc3339(),
                    "metadata": {"source": "test", "position": 1, "category": "test"}
                }))
                .send()
                .await
        });
    }

    while let Some(res) = set.join_next().await {
        assert!(res.unwrap().unwrap().status().is_success());
    }

    tokio::time::sleep(Duration::from_secs(6)).await;

    let now = chrono::Utc::now();
    let ago_6h = now - chrono::Duration::hours(6);

    let from = ago_6h.to_rfc3339().replace("+", "%2B");
    let to = now.to_rfc3339().replace("+", "%2B");

    eprintln!("{}", ago_6h.to_rfc3339());
    let res = client
        .get(format!(
            "http://localhost:8081/api/v1/analytics/user-activity/{}?from={}&to={}",
            user_id, from, to
        ))
        .send()
        .await
        .unwrap();

    let body: UserActivityResponse = res.json().await.unwrap();
    eprintln!("{:?}", body);
    assert_eq!(body.events.views, 2);
}
