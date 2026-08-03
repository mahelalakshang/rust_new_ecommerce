import { Link } from "react-router-dom";

import { EmptyState } from "@/components/feedback/EmptyState";
import { ErrorState } from "@/components/feedback/ErrorState";
import { FullPageSpinner } from "@/components/feedback/LoadingSpinner";
import { Pagination } from "@/components/layout/Pagination";
import { Badge } from "@/components/ui/badge";
import { useOrders } from "@/features/orders/hooks";
import { usePagination } from "@/hooks/usePagination";
import { formatCurrency } from "@/lib/decimal";

export function OrderHistoryPage() {
  const { limit, offset, nextPage, prevPage } = usePagination();
  const { data, isPending, isError, error } = useOrders({ limit, offset });

  if (isPending) return <FullPageSpinner />;
  if (isError) return <ErrorState error={error} />;

  if (data.items.length === 0) {
    return <EmptyState message="You haven't placed any orders yet." />;
  }

  return (
    <div className="flex flex-col gap-6">
      <h1 className="text-2xl font-semibold">Your Orders</h1>
      <div className="divide-y rounded-lg border">
        {data.items.map((order) => (
          <Link
            key={order.id}
            to={`/orders/${order.id}`}
            className="flex items-center justify-between gap-4 px-4 py-3 hover:bg-accent/50"
          >
            <div className="flex flex-col gap-1">
              <span className="font-medium">Order #{order.id.slice(0, 8)}</span>
              <span className="text-sm text-muted-foreground">
                {new Date(order.created_at).toLocaleString()}
              </span>
            </div>
            <div className="flex items-center gap-3">
              <Badge variant="secondary" className="capitalize">
                {order.status}
              </Badge>
              <span className="font-semibold">{formatCurrency(order.total_amount)}</span>
            </div>
          </Link>
        ))}
      </div>
      <Pagination
        total={data.total}
        offset={offset}
        itemsLength={data.items.length}
        onPrev={prevPage}
        onNext={nextPage}
      />
    </div>
  );
}
