use tonic::transport::Channel;

// Include the generated protobuf code
pub mod notification {
    tonic::include_proto!("notification");
}

use notification::{
    notification_service_client::NotificationServiceClient, ProductNotificationRequest,
};

/// Send a product notification to the notification service
pub async fn send_product_notification(
    user_id: &str,
    product_name: &str,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the notification service
    let channel = Channel::from_static("http://localhost:50051")
        .connect()
        .await?;

    let mut client = NotificationServiceClient::new(channel);

    let request = tonic::Request::new(ProductNotificationRequest {
        user_id: user_id.to_string(),
        name: product_name.to_string(),
        username: username.to_string(),
    });

    let response = client.send_product_notification(request).await?;

    tracing::info!(
        "Notification sent successfully: {}",
        response.into_inner().message
    );

    Ok(())
}
