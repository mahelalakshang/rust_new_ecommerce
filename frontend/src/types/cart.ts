/** Wire shape from the backend — `price`/`subtotal`/`total` are Decimals serialized as JSON strings. */
export interface CartItemResponseRaw {
  id: string;
  product_id: string;
  product_name: string;
  price: string;
  quantity: number;
  subtotal: string;
}

export interface CartResponseRaw {
  items: CartItemResponseRaw[];
  total: string;
}

/** UI-facing shapes — decimal strings parsed to numbers. Produced in features/cart/hooks.ts. */
export interface CartItemView {
  id: string;
  product_id: string;
  product_name: string;
  price: number;
  quantity: number;
  subtotal: number;
}

export interface CartView {
  items: CartItemView[];
  total: number;
}

export interface AddToCartRequest {
  product_id: string;
  quantity: number;
}

export interface UpdateCartItemRequest {
  quantity: number;
}
