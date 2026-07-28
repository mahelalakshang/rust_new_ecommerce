import { useMutation, useQuery, useQueryClient, keepPreviousData } from "@tanstack/react-query";

import * as productsApi from "@/features/products/api";
import { parseDecimal } from "@/lib/decimal";
import type { PaginatedResponse, PaginationParams } from "@/types/common";
import type { CreateProductRequest, Product, ProductResponseRaw } from "@/types/product";

function toProduct(raw: ProductResponseRaw): Product {
  return { ...raw, price: parseDecimal(raw.price) };
}

export function useProducts(params: PaginationParams) {
  return useQuery({
    queryKey: ["products", params.limit, params.offset],
    queryFn: () => productsApi.getProducts(params),
    placeholderData: keepPreviousData,
    select: (data: PaginatedResponse<ProductResponseRaw>): PaginatedResponse<Product> => ({
      items: data.items.map(toProduct),
      total: data.total,
    }),
  });
}

export function useProduct(id: string | undefined) {
  return useQuery({
    queryKey: ["products", id],
    queryFn: () => productsApi.getProduct(id!),
    enabled: !!id,
    select: toProduct,
  });
}

export function useCreateProduct() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateProductRequest) => productsApi.createProduct(data),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["products"] });
    },
  });
}
