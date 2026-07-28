export interface CategoryResponse {
  id: string;
  name: string;
  created_at: string;
}

export interface CreateCategoryRequest {
  name: string;
}
