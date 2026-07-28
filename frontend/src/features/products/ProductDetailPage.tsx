import { useParams } from "react-router-dom";

import { ErrorState } from "@/components/feedback/ErrorState";
import { FullPageSpinner } from "@/components/feedback/LoadingSpinner";
import { Badge } from "@/components/ui/badge";
import { AddToCartButton } from "@/features/products/components/AddToCartButton";
import { useProduct } from "@/features/products/hooks";
import { formatCurrency } from "@/lib/decimal";

export function ProductDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { data: product, isPending, isError, error } = useProduct(id);

  if (isPending) return <FullPageSpinner />;
  if (isError) return <ErrorState error={error} />;

  const outOfStock = product.stock_quantity === 0;

  return (
    <div className="mx-auto flex max-w-2xl flex-col gap-4">
      <div className="flex items-start justify-between gap-4">
        <h1 className="text-2xl font-semibold">{product.name}</h1>
        {outOfStock ? (
          <Badge variant="destructive">Out of stock</Badge>
        ) : (
          <Badge variant="secondary">{product.stock_quantity} in stock</Badge>
        )}
      </div>
      <p className="text-muted-foreground">{product.description}</p>
      <p className="text-xl font-semibold">{formatCurrency(product.price)}</p>
      <AddToCartButton
        productId={product.id}
        stockQuantity={product.stock_quantity}
        disabled={outOfStock}
      />
    </div>
  );
}
