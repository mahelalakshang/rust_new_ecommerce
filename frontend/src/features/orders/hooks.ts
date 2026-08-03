import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import * as ordersApi from "@/features/orders/api";
import { parseDecimal } from "@/lib/decimal";
import type { PaginatedResponse, PaginationParams } from "@/types/common";
import type { OrderResponseRaw, OrderView } from "@/types/orders";

function toOrderView(raw: OrderResponseRaw): OrderView {
  return {
    ...raw,
    total_amount: parseDecimal(raw.total_amount),
    items: raw.items.map((item) => ({
      ...item,
      unit_price: parseDecimal(item.unit_price),
    })),
  };
}

export function useCheckout() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ordersApi.checkout,
    onSuccess: () => {
      // Checkout empties the cart server-side; drop the stale cached cart.
      queryClient.invalidateQueries({ queryKey: ["cart"] });
      queryClient.invalidateQueries({ queryKey: ["orders"] });
    },
  });
}

export function useOrders(params: PaginationParams) {
  return useQuery({
    queryKey: ["orders", params.limit, params.offset],
    queryFn: () => ordersApi.getOrders(params),
    select: (data: PaginatedResponse<OrderResponseRaw>): PaginatedResponse<OrderView> => ({
      items: data.items.map(toOrderView),
      total: data.total,
    }),
  });
}

export function useOrder(id: string | undefined) {
  return useQuery({
    queryKey: ["orders", id],
    queryFn: () => ordersApi.getOrder(id!),
    enabled: !!id,
    select: toOrderView,
  });
}
