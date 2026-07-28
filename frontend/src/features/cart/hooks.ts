import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import * as cartApi from "@/features/cart/api";
import { useAuth } from "@/features/auth/AuthContext";
import { parseDecimal } from "@/lib/decimal";
import type {
  AddToCartRequest,
  CartResponseRaw,
  CartView,
  UpdateCartItemRequest,
} from "@/types/cart";

const CART_KEY = ["cart"];

function toCartView(raw: CartResponseRaw): CartView {
  return {
    total: parseDecimal(raw.total),
    items: raw.items.map((item) => ({
      ...item,
      price: parseDecimal(item.price),
      subtotal: parseDecimal(item.subtotal),
    })),
  };
}

export function useCart() {
  const { status } = useAuth();
  return useQuery({
    queryKey: CART_KEY,
    queryFn: cartApi.getCart,
    enabled: status === "authenticated",
    select: toCartView,
  });
}

export function useAddToCart() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (data: AddToCartRequest) => cartApi.addToCart(data),
    onSuccess: (data) => {
      queryClient.setQueryData(CART_KEY, data);
    },
  });
}

export function useUpdateCartItem() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ productId, quantity }: { productId: string; quantity: number }) =>
      cartApi.updateCartItem(productId, { quantity } satisfies UpdateCartItemRequest),
    onSuccess: (data) => {
      queryClient.setQueryData(CART_KEY, data);
    },
  });
}

export function useRemoveCartItem() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (productId: string) => cartApi.removeCartItem(productId),
    onSuccess: (data) => {
      queryClient.setQueryData(CART_KEY, data);
    },
  });
}
