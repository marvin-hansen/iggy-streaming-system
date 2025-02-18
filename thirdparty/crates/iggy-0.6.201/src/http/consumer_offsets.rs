use crate::client::ConsumerOffsetClient;
use crate::consumer::Consumer;
use crate::consumer_offsets::get_consumer_offset::GetConsumerOffset;
use crate::consumer_offsets::store_consumer_offset::StoreConsumerOffset;
use crate::error::IggyError;
use crate::http::client::HttpClient;
use crate::http::HttpTransport;
use crate::identifier::Identifier;
use crate::models::consumer_offset_info::ConsumerOffsetInfo;
use async_trait::async_trait;

#[async_trait]
impl ConsumerOffsetClient for HttpClient {
    async fn store_consumer_offset(
        &self,
        consumer: &Consumer,
        stream_id: &Identifier,
        topic_id: &Identifier,
        partition_id: Option<u32>,
        offset: u64,
    ) -> Result<(), IggyError> {
        self.put(
            &get_path(&stream_id.as_cow_str(), &topic_id.as_cow_str()),
            &StoreConsumerOffset {
                consumer: consumer.clone(),
                stream_id: stream_id.clone(),
                topic_id: topic_id.clone(),
                partition_id,
                offset,
            },
        )
        .await?;
        Ok(())
    }

    async fn get_consumer_offset(
        &self,
        consumer: &Consumer,
        stream_id: &Identifier,
        topic_id: &Identifier,
        partition_id: Option<u32>,
    ) -> Result<Option<ConsumerOffsetInfo>, IggyError> {
        let response = self
            .get_with_query(
                &get_path(&stream_id.as_cow_str(), &topic_id.as_cow_str()),
                &GetConsumerOffset {
                    consumer: consumer.clone(),
                    stream_id: stream_id.clone(),
                    topic_id: topic_id.clone(),
                    partition_id,
                },
            )
            .await;
        if let Err(error) = response {
            if matches!(error, IggyError::ResourceNotFound(_)) {
                return Ok(None);
            }

            return Err(error);
        }

        let offset = response?
            .json()
            .await
            .map_err(|_| IggyError::InvalidJsonResponse)?;
        Ok(Some(offset))
    }

    async fn delete_consumer_offset(
        &self,
        consumer: &Consumer,
        stream_id: &Identifier,
        topic_id: &Identifier,
        partition_id: Option<u32>,
    ) -> Result<(), IggyError> {
        let partition_id = partition_id
            .map(|id| format!("?partition_id={id}"))
            .unwrap_or_default();
        let path = format!(
            "{}/{}{partition_id}",
            get_path(&stream_id.as_cow_str(), &topic_id.as_cow_str()),
            consumer.id
        );
        self.delete(&path).await?;
        Ok(())
    }
}

fn get_path(stream_id: &str, topic_id: &str) -> String {
    format!("streams/{stream_id}/topics/{topic_id}/consumer-offsets")
}
