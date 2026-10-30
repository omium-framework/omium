use rustbridge_core::config::load_config;
use rustbridge_db::{apply_migrations, connect};
use sample_app::services::ExampleService;
use sample_app_migration::Migrator;
use std::path::PathBuf;

#[tokio::test]
#[ignore = "requires MySQL; see docs/LOCAL_MYSQL.md"]
async fn example_crud_round_trip() {
    let config_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config");
    let config = load_config(&config_dir).expect("config");
    if !config.database.enabled {
        eprintln!("Set APP_DATABASE__ENABLED=true and APP_DATABASE__PASSWORD");
        return;
    }

    let connection = connect(&config.database).await.expect("connect");
    apply_migrations::<Migrator>(&connection)
        .await
        .expect("migrate");

    let created = ExampleService::create(&connection, "Omium Phase 3", None)
        .await
        .expect("create");
    let found = ExampleService::find_by_name(&connection, "Omium Phase 3")
        .await
        .expect("find")
        .expect("row");
    assert_eq!(created.id, found.id);
}
