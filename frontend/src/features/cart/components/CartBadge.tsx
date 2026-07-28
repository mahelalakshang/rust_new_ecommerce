import { useCart } from "@/features/cart/hooks";

export function CartBadge() {
  const { data } = useCart();
  const count = data?.items.reduce((sum, item) => sum + item.quantity, 0) ?? 0;

  if (count === 0) return null;

  return (
    <span className="absolute -right-2 -top-2 flex size-4 items-center justify-center rounded-full bg-primary text-[10px] font-medium text-primary-foreground">
      {count}
    </span>
  );
}
