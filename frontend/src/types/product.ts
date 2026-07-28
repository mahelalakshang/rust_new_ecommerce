/** Wire shape from the backend — `price` is a Decimal serialized as a JSON string. */
export interface ProductResponseRaw {
  id: string;
  name: string;
  description: string;
  category_id: string;
  price: string;
  stock_quantity: number;
}

/** UI-facing shape — `price` parsed to a number. Produced in features/products/hooks.ts. */
export interface Product {
  id: string;
  name: string;
  description: string;
  category_id: string;
  price: number;
  stock_quantity: number;
}

/**
 * NOTE: `price` here is a plain number, unlike `ProductResponseRaw.price` (a string).
 * This asymmetry mirrors the backend: `CreateProductRequest.price` is an `f64` on the way in,
 * but `Product.price` is a `Decimal` (serialized as a string) on the way out.
 */
export interface CreateProductRequest {
  name: string;
  description: string;
  category_id: string;
  price: number;
  stock_quantity: number;
}
