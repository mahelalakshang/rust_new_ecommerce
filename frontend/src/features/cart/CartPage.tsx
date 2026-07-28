import { Link } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { EmptyState } from "@/components/feedback/EmptyState";
import { ErrorState } from "@/components/feedback/ErrorState";
import { FullPageSpinner } from "@/components/feedback/LoadingSpinner";
import { CartItemRow } from "@/features/cart/components/CartItemRow";
import { CartSummary } from "@/features/cart/components/CartSummary";
import { useCart } from "@/features/cart/hooks";

export function CartPage() {
  const { data: cart, isPending, isError, error } = useCart();

  if (isPending) return <FullPageSpinner />;
  if (isError) return <ErrorState error={error} />;

  if (cart.items.length === 0) {
    return (
      <EmptyState
        message="Your cart is empty."
        action={
          <Button asChild>
            <Link to="/">Browse products</Link>
          </Button>
        }
      />
    );
  }

  const itemCount = cart.items.reduce((sum, item) => sum + item.quantity, 0);

  return (
    <div className="grid gap-6 lg:grid-cols-3">
      <div className="lg:col-span-2">
        <h1 className="mb-4 text-2xl font-semibold">Your Cart</h1>
        <div className="rounded-lg border px-4">
          {cart.items.map((item) => (
            <CartItemRow key={item.id} item={item} />
          ))}
        </div>
      </div>
      <div>
        <CartSummary total={cart.total} itemCount={itemCount} />
      </div>
    </div>
  );
}
