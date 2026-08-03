import { request } from "@/lib/api-client";
import type { PaginatedResponse, PaginationParams } from "@/types/common";
import type { OrderResponseRaw } from "@/types/orders";

export function checkout(): Promise<OrderResponseRaw> {
  return request<OrderResponseRaw>("/checkout", { method: "POST" });
}

export function getOrders(
  params: PaginationParams,
): Promise<PaginatedResponse<OrderResponseRaw>> {
  return request<PaginatedResponse<OrderResponseRaw>>(
    `/orders?limit=${params.limit}&offset=${params.offset}`,
  );
}

export function getOrder(id: string): Promise<OrderResponseRaw> {
  return request<OrderResponseRaw>(`/orders/${id}`);
}
