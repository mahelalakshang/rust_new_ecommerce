use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use tonic::{transport::Server, Request, Response, Status};

// Include the generated protobuf code
pub mod notification {
    tonic::include_proto!("notification");
}

use notification::{
    notification_service_server::{NotificationService, NotificationServiceServer},
    OrderConfirmationRequest, OrderConfirmationResponse, ProductNotificationRequest,
    ProductNotificationResponse,
};

#[derive(Clone)]
pub struct NotificationServiceImpl {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
}

#[tonic::async_trait]
impl NotificationService for NotificationServiceImpl {
    async fn send_product_notification(
        &self,
        request: Request<ProductNotificationRequest>,
    ) -> Result<Response<ProductNotificationResponse>, Status> {
        let req = request.into_inner();

        // Print the received notification data
        println!("📢 NOTIFICATION RECEIVED:");
        println!("   User ID: {}", req.user_id);
        println!("   Product Name: {}", req.name);
        println!("   Username: {}", req.username);
        println!("   Timestamp: {}", chrono::Utc::now());
        println!("---");

        let email = Message::builder()
            .from(self.from_email.parse().map_err(|e| {
                Status::internal(format!("invalid from address: {e}"))
            })?)
            .to(req.username.parse().map_err(|e| {
                Status::internal(format!("invalid recipient address: {e}"))
            })?)
            .subject(format!("New Product Created: {}", req.name))
            .body(format!(
                "Your company has created a new product.\n\nProduct Name: {}\nCreated At: {}\n\nThis is an automated notification.",
                req.name,
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            ))
            .map_err(|e| Status::internal(format!("failed to build email: {e}")))?;

        match self.mailer.send(email).await {
            Ok(_) => tracing::info!("Notification email sent to {}", req.username),
            Err(e) => tracing::error!(
                "Failed to send notification email to {}: {}",
                req.username,
                e
            ),
        }

        // Return success response
        let response = ProductNotificationResponse {
            success: true,
            message: format!("Notification received for product: {}", req.name),
        };

        Ok(Response::new(response))
    }

    async fn send_order_confirmation(
        &self,
        request: Request<OrderConfirmationRequest>,
    ) -> Result<Response<OrderConfirmationResponse>, Status> {
        let req = request.into_inner();

        println!("📦 ORDER CONFIRMATION RECEIVED:");
        println!("   User ID: {}", req.user_id);
        println!("   Order ID: {}", req.order_id);
        println!("   Username: {}", req.username);
        println!("   Total: {}", req.total_amount);
        println!("   Timestamp: {}", chrono::Utc::now());
        println!("---");

        let email = Message::builder()
            .from(
                self.from_email
                    .parse()
                    .map_err(|e| Status::internal(format!("invalid from address: {e}")))?,
            )
            .to(req
                .username
                .parse()
                .map_err(|e| Status::internal(format!("invalid recipient address: {e}")))?)
            .subject(format!("Order Confirmation: #{}", req.order_id))
            .body(format!(
                "Thanks for your order!\n\nOrder ID: {}\nTotal: {}\nPlaced At: {}\n\nThis is an automated notification.",
                req.order_id,
                req.total_amount,
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            ))
            .map_err(|e| Status::internal(format!("failed to build email: {e}")))?;

        match self.mailer.send(email).await {
            Ok(_) => tracing::info!("Order confirmation email sent to {}", req.username),
            Err(e) => tracing::error!(
                "Failed to send order confirmation email to {}: {}",
                req.username,
                e
            ),
        }

        let response = OrderConfirmationResponse {
            success: true,
            message: format!("Order confirmation received for order: {}", req.order_id),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let smtp_host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let smtp_user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
    let smtp_pass = std::env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let smtp_from = std::env::var("SMTP_FROM").expect("SMTP_FROM must be set");

    let creds = Credentials::new(smtp_user, smtp_pass);
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
        .credentials(creds)
        .build();

    let addr = "0.0.0.0:50051".parse()?;
    let notification_service = NotificationServiceImpl {
        mailer,
        from_email: smtp_from,
    };

    println!("🚀 Notification Service starting on {}", addr);

    Server::builder()
        .add_service(NotificationServiceServer::new(notification_service))
        .serve(addr)
        .await?;

    Ok(())
}
