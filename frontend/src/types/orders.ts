/** Wire shape from the backend — money fields are Decimals serialized as JSON strings. */
export interface OrderItemResponseRaw {
  product_id: string;
  product_name: string;
  unit_price: string;
  quantity: number;
}

export interface OrderResponseRaw {
  id: string;
  user_id: string;
  status: string;
  total_amount: string;
  items: OrderItemResponseRaw[];
  created_at: string;
  updated_at: string;
}

/** UI-facing shapes — decimal strings parsed to numbers. Produced in features/orders/hooks.ts. */
export interface OrderItemView {
  product_id: string;
  product_name: string;
  unit_price: number;
  quantity: number;
}

export interface OrderView {
  id: string;
  user_id: string;
  status: string;
  total_amount: number;
  items: OrderItemView[];
  created_at: string;
  updated_at: string;
}
