use std::str::FromStr;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use tonic::transport::Channel;
use tonic::Status;
use uuid::Uuid;

use crate::dtos::{OrderItemResponse, OrderResponse};

// Include the generated protobuf code
pub mod order {
    tonic::include_proto!("order");
}

use order::{
    order_service_client::OrderServiceClient, CreateOrderRequest, GetOrderRequest,
    ListOrdersRequest, Order, OrderItemInput,
};

/// Converts the gRPC wire type (money/timestamps as strings) into the HTTP DTO.
pub fn to_order_response(order: Order) -> Result<OrderResponse, Status> {
    let parse_decimal = |s: &str| {
        Decimal::from_str(s)
            .map_err(|e| Status::internal(format!("order service returned invalid decimal: {e}")))
    };
    let parse_timestamp = |s: &str| {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| Status::internal(format!("order service returned invalid timestamp: {e}")))
    };

    let items = order
        .items
        .into_iter()
        .map(|item| {
            Ok(OrderItemResponse {
                product_id: Uuid::parse_str(&item.product_id)
                    .map_err(|e| Status::internal(format!("invalid product_id: {e}")))?,
                product_name: item.product_name,
                unit_price: parse_decimal(&item.unit_price)?,
                quantity: item.quantity,
            })
        })
        .collect::<Result<Vec<_>, Status>>()?;

    Ok(OrderResponse {
        id: Uuid::parse_str(&order.id).map_err(|e| Status::internal(format!("invalid order id: {e}")))?,
        user_id: Uuid::parse_str(&order.user_id)
            .map_err(|e| Status::internal(format!("invalid user_id: {e}")))?,
        status: order.status,
        total_amount: parse_decimal(&order.total_amount)?,
        items,
        created_at: parse_timestamp(&order.created_at)?,
        updated_at: parse_timestamp(&order.updated_at)?,
    })
}

/// Builds a lazily-connecting channel once at startup (stored on `Config`), unlike
/// `grpc_client.rs` which opens a fresh TCP connection on every call. `connect_lazy`
/// doesn't dial immediately, so API startup isn't coupled to the order service
/// already being up; the connection is established on first RPC use and reused
/// (cheap to clone) after that.
pub fn connect() -> OrderServiceClient<Channel> {
    let channel = Channel::from_static("http://localhost:50052").connect_lazy();
    OrderServiceClient::new(channel)
}

pub async fn create_order(
    client: &OrderServiceClient<Channel>,
    user_id: Uuid,
    items: Vec<(Uuid, i32)>,
) -> Result<Order, Status> {
    let mut client = client.clone();
    let request = tonic::Request::new(CreateOrderRequest {
        user_id: user_id.to_string(),
        items: items
            .into_iter()
            .map(|(product_id, quantity)| OrderItemInput {
                product_id: product_id.to_string(),
                quantity,
            })
            .collect(),
    });

    let response = client.create_order(request).await?;
    response
        .into_inner()
        .order
        .ok_or_else(|| Status::internal("order service returned an empty order"))
}

pub async fn get_order(
    client: &OrderServiceClient<Channel>,
    order_id: Uuid,
) -> Result<Order, Status> {
    let mut client = client.clone();
    let request = tonic::Request::new(GetOrderRequest {
        order_id: order_id.to_string(),
    });

    let response = client.get_order(request).await?;
    response
        .into_inner()
        .order
        .ok_or_else(|| Status::internal("order service returned an empty order"))
}

pub async fn list_orders(
    client: &OrderServiceClient<Channel>,
    user_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Order>, i64), Status> {
    let mut client = client.clone();
    let request = tonic::Request::new(ListOrdersRequest {
        user_id: user_id.to_string(),
        limit: limit as i32,
        offset: offset as i32,
    });

    let response = client.list_orders(request).await?.into_inner();
    Ok((response.orders, response.total))
}
