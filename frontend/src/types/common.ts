export interface PaginatedResponse<T> {
  items: T[];
  total: number;
}

export interface ApiErrorBody {
  error: string;
}

export interface PaginationParams {
  limit: number;
  offset: number;
}
