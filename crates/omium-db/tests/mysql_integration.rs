use omium_db::{apply_migrations, connect, ping, run_in_transaction};
use omium_db_test_migrations::Migrator;
use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};

fn mysql_url() -> Option<String> {
    std::env::var("RUSTBRIDGE_TEST_MYSQL_URL").ok()
}

#[tokio::test]
#[ignore = "requires RUSTBRIDGE_TEST_MYSQL_URL and a reachable MySQL server"]
async fn mysql_connect_ping_and_migrate_fresh() {
    let Some(url) = mysql_url() else {
        panic!("set RUSTBRIDGE_TEST_MYSQL_URL to run this test");
    };

    let settings = test_settings_from_url(&url);
    let connection = connect(&settings).await.expect("connect");
    ping(&connection).await.expect("ping");

    apply_migrations::<Migrator>(&connection)
        .await
        .expect("migrate fresh");

    let row = connection
        .query_one(Statement::from_string(
            DatabaseBackend::MySql,
            String::from("SHOW COLUMNS FROM examples LIKE 'description'"),
        ))
        .await
        .expect("describe")
        .expect("description column");
    let field: String = row.try_get("", "Field").expect("field");
    assert_eq!(field, "description");
}

#[tokio::test]
#[ignore = "requires RUSTBRIDGE_TEST_MYSQL_URL and a reachable MySQL server"]
async fn mysql_transaction_commit_and_rollback() {
    let Some(url) = mysql_url() else {
        panic!("set RUSTBRIDGE_TEST_MYSQL_URL to run this test");
    };

    let settings = test_settings_from_url(&url);
    let connection = connect(&settings).await.expect("connect");
    apply_migrations::<Migrator>(&connection)
        .await
        .expect("migrate");

    let marker = format!("tx-test-{}", uuid::Uuid::new_v4());
    run_in_transaction(&connection, |transaction| {
        let marker = marker.clone();
        async move {
            transaction
                .execute(Statement::from_string(
                    DatabaseBackend::MySql,
                    format!("INSERT INTO examples (name) VALUES ('{marker}')"),
                ))
                .await?;
            transaction.commit().await?;
            Ok::<(), sea_orm::DbErr>(())
        }
    })
    .await
    .expect("commit transaction");

    let count_after_commit: i64 = connection
        .query_one(Statement::from_string(
            DatabaseBackend::MySql,
            format!("SELECT COUNT(*) AS count FROM examples WHERE name = '{marker}'"),
        ))
        .await
        .expect("count")
        .expect("row")
        .try_get("", "count")
        .expect("count value");
    assert_eq!(count_after_commit, 1);

    let rollback_marker = format!("rollback-{}", uuid::Uuid::new_v4());
    let result: Result<(), sea_orm::DbErr> = run_in_transaction(&connection, |transaction| {
        let rollback_marker = rollback_marker.clone();
        async move {
            transaction
                .execute(Statement::from_string(
                    DatabaseBackend::MySql,
                    format!("INSERT INTO examples (name) VALUES ('{rollback_marker}')"),
                ))
                .await?;
            Err(sea_orm::DbErr::Custom(String::from("force rollback")))
        }
    })
    .await;
    assert!(result.is_err());

    let count_after_rollback: i64 = connection
        .query_one(Statement::from_string(
            DatabaseBackend::MySql,
            format!("SELECT COUNT(*) AS count FROM examples WHERE name = '{rollback_marker}'"),
        ))
        .await
        .expect("count")
        .expect("row")
        .try_get("", "count")
        .expect("count value");
    assert_eq!(count_after_rollback, 0);
}

fn test_settings_from_url(url: &str) -> omium_core::config::DatabaseSettings {
    let parsed = url::Url::parse(url).expect("mysql url");
    let password = parsed.password().unwrap_or("").to_string();
    omium_core::config::DatabaseSettings {
        enabled: true,
        host: parsed.host_str().unwrap_or("127.0.0.1").to_string(),
        port: parsed.port().unwrap_or(3306),
        name: parsed.path().trim_start_matches('/').to_string(),
        username: parsed.username().to_string(),
        password: Some(password),
    }
}
