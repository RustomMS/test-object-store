fn main() {}

#[cfg(test)]
mod test {
    use std::env;
    use std::sync::Arc;

    use bytes::Bytes;
    use object_store::gcp::GoogleCloudStorageBuilder;
    use object_store::path::Path;
    use object_store::ObjectStore;
    use uuid::Uuid;

    #[tokio::test]
    async fn content_length_test() {
        let store = GoogleCloudStorageBuilder::new()
            .with_bucket_name(
                env::var("OBJECT_STORE_BUCKET").expect("already checked OBJECT_STORE_BUCKET"),
            )
            .with_service_account_path(
                env::var("GOOGLE_SERVICE_ACCOUNT").expect("already checked GOOGLE_SERVICE_ACCOUNT"),
            )
            .build()
            .unwrap();

        let store: Arc<dyn ObjectStore> = Arc::new(store);

        let uuid = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();

        let other = tmp_path("a", &uuid, &uuid2);
        let from = tmp_path("b", &uuid, &uuid2);
        let to = visible_path("c", &uuid);

        let bytes = Bytes::from("test-data-test-data");

        store.put(&other, bytes.clone()).await.unwrap();
        store.put(&from, bytes).await.unwrap();

        store.rename_if_not_exists(&from, &to).await.unwrap();
    }

    fn visible_path(location: &str, db_id: &Uuid) -> Path {
        Path::from(format!("databases/{}/visible/{}", db_id, location,))
    }

    fn tmp_path(location: &str, db_id: &Uuid, process_id: &Uuid) -> Path {
        Path::from(format!(
            "databases/{}/tmp/{}/{}",
            db_id, process_id, location,
        ))
    }
}
