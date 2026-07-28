import { request } from "@/lib/api-client";
import type {
  AddToCartRequest,
  CartResponseRaw,
  UpdateCartItemRequest,
} from "@/types/cart";

export function getCart(): Promise<CartResponseRaw> {
  return request<CartResponseRaw>("/cart");
}

export function addToCart(data: AddToCartRequest): Promise<CartResponseRaw> {
  return request<CartResponseRaw>("/cart", { method: "POST", body: data });
}

export function updateCartItem(
  productId: string,
  data: UpdateCartItemRequest,
): Promise<CartResponseRaw> {
  return request<CartResponseRaw>(`/cart/${productId}`, { method: "PATCH", body: data });
}

export function removeCartItem(productId: string): Promise<CartResponseRaw> {
  return request<CartResponseRaw>(`/cart/${productId}`, { method: "DELETE" });
}
