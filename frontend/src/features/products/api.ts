import { request } from "@/lib/api-client";
import type { PaginatedResponse, PaginationParams } from "@/types/common";
import type { CreateProductRequest, ProductResponseRaw } from "@/types/product";

export function getProducts(
  params: PaginationParams,
): Promise<PaginatedResponse<ProductResponseRaw>> {
  return request<PaginatedResponse<ProductResponseRaw>>(
    `/products?limit=${params.limit}&offset=${params.offset}`,
  );
}

export function getProduct(id: string): Promise<ProductResponseRaw> {
  return request<ProductResponseRaw>(`/products/${id}`);
}

export function createProduct(data: CreateProductRequest): Promise<ProductResponseRaw> {
  return request<ProductResponseRaw>("/products", { method: "POST", body: data });
}
