import { Trash2 } from "lucide-react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { QuantityStepper } from "@/features/cart/components/QuantityStepper";
import { useRemoveCartItem, useUpdateCartItem } from "@/features/cart/hooks";
import { ApiError } from "@/lib/api-client";
import { formatCurrency } from "@/lib/decimal";
import type { CartItemView } from "@/types/cart";

export function CartItemRow({ item }: { item: CartItemView }) {
  const updateMutation = useUpdateCartItem();
  const removeMutation = useRemoveCartItem();

  const isRowPending =
    (updateMutation.isPending && updateMutation.variables?.productId === item.product_id) ||
    (removeMutation.isPending && removeMutation.variables === item.product_id);

  const handleQuantityChange = (quantity: number) => {
    updateMutation.mutate(
      { productId: item.product_id, quantity },
      {
        onError: (err) => {
          toast.error(err instanceof ApiError ? err.body?.error ?? err.message : "Failed to update quantity");
        },
      },
    );
  };

  const handleRemove = () => {
    removeMutation.mutate(item.product_id, {
      onError: (err) => {
        toast.error(err instanceof ApiError ? err.body?.error ?? err.message : "Failed to remove item");
      },
    });
  };

  return (
    <div className="flex items-center justify-between gap-4 border-b py-4 last:border-0">
      {/* min-w-0 + truncate: a long product name must not widen the row on mobile */}
      <div className="flex min-w-0 flex-col gap-1">
        <span className="truncate font-medium">{item.product_name}</span>
        <span className="text-sm text-muted-foreground">{formatCurrency(item.price)} each</span>
      </div>
      <div className="flex shrink-0 items-center gap-2 sm:gap-4">
        <QuantityStepper value={item.quantity} disabled={isRowPending} onChange={handleQuantityChange} />
        <span className="w-20 text-right font-medium">{formatCurrency(item.subtotal)}</span>
        <Button
          type="button"
          variant="ghost"
          size="icon"
          disabled={isRowPending}
          onClick={handleRemove}
        >
          <Trash2 className="size-4 text-destructive" />
        </Button>
      </div>
    </div>
  );
}
