import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import * as categoriesApi from "@/features/categories/api";
import type { PaginationParams } from "@/types/common";
import type { CreateCategoryRequest } from "@/types/category";

export function useCategories(params: PaginationParams) {
  return useQuery({
    queryKey: ["categories", params.limit, params.offset],
    queryFn: () => categoriesApi.getCategories(params),
  });
}

export function useCreateCategory() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateCategoryRequest) => categoriesApi.createCategory(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["categories"] });
    },
  });
}
