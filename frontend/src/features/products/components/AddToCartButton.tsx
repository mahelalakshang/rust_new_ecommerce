import { useState } from "react";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { QuantityStepper } from "@/features/cart/components/QuantityStepper";
import { useAddToCart } from "@/features/cart/hooks";
import { ApiError } from "@/lib/api-client";

interface AddToCartButtonProps {
  productId: string;
  stockQuantity: number;
  disabled?: boolean;
}

export function AddToCartButton({ productId, stockQuantity, disabled }: AddToCartButtonProps) {
  const [quantity, setQuantity] = useState(1);
  const addToCart = useAddToCart();

  const handleAdd = () => {
    addToCart.mutate(
      { product_id: productId, quantity },
      {
        onSuccess: () => {
          toast.success(`Added ${quantity} to cart`);
          setQuantity(1);
        },
        onError: (err) => {
          toast.error(
            err instanceof ApiError ? err.body?.error ?? err.message : "Failed to add to cart",
          );
        },
      },
    );
  };

  if (disabled) {
    return (
      <Button disabled className="mt-2 w-fit">
        Out of stock
      </Button>
    );
  }

  return (
    <div className="mt-2 flex items-center gap-4">
      <QuantityStepper value={quantity} max={stockQuantity} onChange={setQuantity} />
      <Button onClick={handleAdd} disabled={addToCart.isPending}>
        {addToCart.isPending ? "Adding…" : "Add to cart"}
      </Button>
    </div>
  );
}
