use std::pin::Pin;
use std::time::Duration;

use rust_decimal::Decimal;
use sqlx::PgPool;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::Stream;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::model::{LockedProductRow, OrderItemRow, OrderRow};
use crate::proto::order::{
    order_service_server::OrderService, CreateOrderRequest, CreateOrderResponse, GetOrderRequest,
    GetOrderResponse, ListOrdersRequest, ListOrdersResponse, Order, OrderItem, OrderStatusUpdate,
    WatchOrderStatusRequest,
};

#[derive(Clone)]
pub struct OrderServiceImpl {
    pub db_pool: PgPool,
}

fn parse_uuid(raw: &str, field: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(raw).map_err(|_| Status::invalid_argument(format!("invalid {field}: {raw}")))
}

fn to_proto_order(order: OrderRow, items: Vec<OrderItemRow>) -> Order {
    Order {
        id: order.id.to_string(),
        user_id: order.user_id.to_string(),
        status: order.status,
        total_amount: order.total_amount.to_string(),
        items: items
            .into_iter()
            .map(|i| OrderItem {
                product_id: i.product_id.to_string(),
                product_name: i.product_name,
                unit_price: i.unit_price.to_string(),
                quantity: i.quantity,
            })
            .collect(),
        created_at: order.created_at.to_rfc3339(),
        updated_at: order.updated_at.to_rfc3339(),
    }
}

impl OrderServiceImpl {
    async fn fetch_order_items(&self, order_id: Uuid) -> Result<Vec<OrderItemRow>, Status> {
        sqlx::query_as::<_, OrderItemRow>(
            "SELECT product_id, product_name, unit_price, quantity FROM order_items WHERE order_id = $1 ORDER BY created_at",
        )
        .bind(order_id)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Status::internal(format!("failed to load order items: {e}")))
    }

    async fn load_order(&self, order_id: Uuid) -> Result<Option<Order>, Status> {
        let order_row = sqlx::query_as::<_, OrderRow>("SELECT * FROM orders WHERE id = $1")
            .bind(order_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| Status::internal(format!("failed to load order: {e}")))?;

        let Some(order_row) = order_row else {
            return Ok(None);
        };
        let items = self.fetch_order_items(order_id).await?;
        Ok(Some(to_proto_order(order_row, items)))
    }
}

/// Stands in for a real payment-webhook / shipping-carrier-webhook integration,
/// so `WatchOrderStatus` subscribers have something to observe. Clearly a demo
/// simulator, not a real fulfillment pipeline.
fn spawn_fulfillment_simulator(pool: PgPool, order_id: Uuid) {
    tokio::spawn(async move {
        for status in ["paid", "shipped", "delivered"] {
            tokio::time::sleep(Duration::from_secs(5)).await;
            if let Err(e) = sqlx::query(
                "UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(status)
            .bind(order_id)
            .execute(&pool)
            .await
            {
                tracing::warn!("fulfillment simulator failed to advance order {order_id}: {e}");
                return;
            }
            tracing::info!("order {order_id} advanced to '{status}'");
        }
    });
}

#[tonic::async_trait]
impl OrderService for OrderServiceImpl {
    async fn create_order(
        &self,
        request: Request<CreateOrderRequest>,
    ) -> Result<Response<CreateOrderResponse>, Status> {
        let req = request.into_inner();
        let user_id = parse_uuid(&req.user_id, "user_id")?;

        if req.items.is_empty() {
            return Err(Status::invalid_argument("order must contain at least one item"));
        }
        let mut requested: Vec<(Uuid, i32)> = Vec::with_capacity(req.items.len());
        for item in &req.items {
            if item.quantity <= 0 {
                return Err(Status::invalid_argument("item quantity must be greater than 0"));
            }
            requested.push((parse_uuid(&item.product_id, "product_id")?, item.quantity));
        }
        // Sorting by product_id gives every concurrent CreateOrder call the same
        // lock-acquisition order, avoiding deadlocks between transactions that
        // touch the same products in different orders.
        requested.sort_by_key(|(id, _)| *id);

        let mut tx = self
            .db_pool
            .begin()
            .await
            .map_err(|e| Status::internal(format!("failed to start transaction: {e}")))?;

        let mut locked_items: Vec<OrderItemRow> = Vec::with_capacity(requested.len());
        for (product_id, quantity) in &requested {
            let product = sqlx::query_as::<_, LockedProductRow>(
                "SELECT id, name, price, stock_quantity FROM products WHERE id = $1 FOR UPDATE",
            )
            .bind(product_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("failed to lock product: {e}")))?
            .ok_or_else(|| Status::not_found(format!("product {product_id} not found")))?;

            if product.stock_quantity < *quantity {
                return Err(Status::failed_precondition(format!(
                    "insufficient stock for product '{}': requested {}, available {}",
                    product.name, quantity, product.stock_quantity
                )));
            }

            locked_items.push(OrderItemRow {
                product_id: product.id,
                product_name: product.name,
                unit_price: product.price,
                quantity: *quantity,
            });
        }

        for item in &locked_items {
            sqlx::query(
                "UPDATE products SET stock_quantity = stock_quantity - $1, updated_at = NOW() WHERE id = $2",
            )
            .bind(item.quantity)
            .bind(item.product_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("failed to decrement stock: {e}")))?;
        }

        let total_amount: Decimal = locked_items
            .iter()
            .map(|i| i.unit_price * Decimal::from(i.quantity))
            .sum();

        let order_row = sqlx::query_as::<_, OrderRow>(
            "INSERT INTO orders (user_id, status, total_amount) VALUES ($1, 'created', $2) RETURNING *",
        )
        .bind(user_id)
        .bind(total_amount)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("failed to create order: {e}")))?;

        for item in &locked_items {
            sqlx::query(
                "INSERT INTO order_items (order_id, product_id, product_name, unit_price, quantity) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(order_row.id)
            .bind(item.product_id)
            .bind(&item.product_name)
            .bind(item.unit_price)
            .bind(item.quantity)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("failed to create order item: {e}")))?;
        }

        tx.commit()
            .await
            .map_err(|e| Status::internal(format!("failed to commit order: {e}")))?;

        spawn_fulfillment_simulator(self.db_pool.clone(), order_row.id);

        let order = to_proto_order(order_row, locked_items);
        Ok(Response::new(CreateOrderResponse { order: Some(order) }))
    }

    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> Result<Response<GetOrderResponse>, Status> {
        let req = request.into_inner();
        let order_id = parse_uuid(&req.order_id, "order_id")?;

        let order = self
            .load_order(order_id)
            .await?
            .ok_or_else(|| Status::not_found(format!("order {order_id} not found")))?;

        Ok(Response::new(GetOrderResponse { order: Some(order) }))
    }

    async fn list_orders(
        &self,
        request: Request<ListOrdersRequest>,
    ) -> Result<Response<ListOrdersResponse>, Status> {
        let req = request.into_inner();
        let user_id = parse_uuid(&req.user_id, "user_id")?;
        let limit = if req.limit > 0 { req.limit as i64 } else { 10 };
        let offset = if req.offset > 0 { req.offset as i64 } else { 0 };

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM orders WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&self.db_pool)
            .await
            .map_err(|e| Status::internal(format!("failed to count orders: {e}")))?;

        let order_rows = sqlx::query_as::<_, OrderRow>(
            "SELECT * FROM orders WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| Status::internal(format!("failed to list orders: {e}")))?;

        let mut orders = Vec::with_capacity(order_rows.len());
        for row in order_rows {
            let items = self.fetch_order_items(row.id).await?;
            orders.push(to_proto_order(row, items));
        }

        Ok(Response::new(ListOrdersResponse { orders, total }))
    }

    type WatchOrderStatusStream =
        Pin<Box<dyn Stream<Item = Result<OrderStatusUpdate, Status>> + Send + 'static>>;

    async fn watch_order_status(
        &self,
        request: Request<WatchOrderStatusRequest>,
    ) -> Result<Response<Self::WatchOrderStatusStream>, Status> {
        let req = request.into_inner();
        let order_id = parse_uuid(&req.order_id, "order_id")?;

        // Confirm the order exists before opening the stream.
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM orders WHERE id = $1")
            .bind(order_id)
            .fetch_optional(&self.db_pool)
            .await
            .map_err(|e| Status::internal(format!("failed to check order: {e}")))?
            .ok_or_else(|| Status::not_found(format!("order {order_id} not found")))?;

        let (tx, rx) = mpsc::channel(16);
        let pool = self.db_pool.clone();

        // DB polling, not an in-process broadcast channel: polling survives
        // restarts and works correctly across multiple service replicas,
        // whereas an in-memory channel would only ever see updates made by
        // this one process.
        tokio::spawn(async move {
            let mut last_status: Option<String> = None;
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            // Safety cutoff so a stuck/never-terminal order can't leak this task forever.
            for _ in 0..300 {
                interval.tick().await;

                let row = sqlx::query_as::<_, (String, chrono::DateTime<chrono::Utc>)>(
                    "SELECT status, updated_at FROM orders WHERE id = $1",
                )
                .bind(order_id)
                .fetch_optional(&pool)
                .await;

                let Ok(Some((status, updated_at))) = row else {
                    break;
                };

                if last_status.as_deref() != Some(status.as_str()) {
                    let update = OrderStatusUpdate {
                        order_id: order_id.to_string(),
                        status: status.clone(),
                        updated_at: updated_at.to_rfc3339(),
                    };
                    if tx.send(Ok(update)).await.is_err() {
                        // Receiver dropped: client disconnected.
                        break;
                    }
                    let is_terminal = matches!(status.as_str(), "delivered" | "cancelled");
                    last_status = Some(status);
                    if is_terminal {
                        break;
                    }
                }
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream) as Self::WatchOrderStatusStream))
    }
}
