fn main() {}

#[cfg(test)]
mod test {
    use std::env;

    use bytes::Bytes;
    use object_store::gcp::GoogleCloudStorageBuilder;
    use object_store::path::Path;
    use object_store::ObjectStore;

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

        let from = Path::from("a");
        let to = Path::from("b");

        let bytes = Bytes::from("test-data");

        store.put(&from, bytes).await.unwrap();

        store.rename_if_not_exists(&from, &to).await.unwrap();
    }
}
