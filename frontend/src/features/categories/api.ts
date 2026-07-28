import { request } from "@/lib/api-client";
import type { CategoryResponse, CreateCategoryRequest } from "@/types/category";
import type { PaginatedResponse, PaginationParams } from "@/types/common";

export function getCategories(
  params: PaginationParams,
): Promise<PaginatedResponse<CategoryResponse>> {
  return request<PaginatedResponse<CategoryResponse>>(
    `/categories?limit=${params.limit}&offset=${params.offset}`,
  );
}

export function getCategory(id: string): Promise<CategoryResponse> {
  return request<CategoryResponse>(`/categories/${id}`);
}

export function createCategory(data: CreateCategoryRequest): Promise<CategoryResponse> {
  return request<CategoryResponse>("/categories", { method: "POST", body: data });
}
