import { useParams } from "react-router-dom";

import { ErrorState } from "@/components/feedback/ErrorState";
import { FullPageSpinner } from "@/components/feedback/LoadingSpinner";
import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { useOrder } from "@/features/orders/hooks";
import { formatCurrency } from "@/lib/decimal";

export function OrderDetailPage() {
  const { id } = useParams<{ id: string }>();
  const { data: order, isPending, isError, error } = useOrder(id);

  if (isPending) return <FullPageSpinner />;
  if (isError) return <ErrorState error={error} />;

  return (
    <div className="mx-auto flex max-w-2xl flex-col gap-4">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h1 className="text-2xl font-semibold">Order #{order.id.slice(0, 8)}</h1>
          <p className="text-sm text-muted-foreground">
            Placed {new Date(order.created_at).toLocaleString()}
          </p>
        </div>
        <Badge variant="secondary" className="capitalize">
          {order.status}
        </Badge>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Items</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-3">
          {order.items.map((item) => (
            <div key={item.product_id} className="flex justify-between text-sm">
              <span>
                {item.product_name} &times; {item.quantity}
              </span>
              <span>{formatCurrency(item.unit_price * item.quantity)}</span>
            </div>
          ))}
        </CardContent>
        <CardFooter className="flex justify-between border-t pt-4 text-lg font-semibold">
          <span>Total</span>
          <span>{formatCurrency(order.total_amount)}</span>
        </CardFooter>
      </Card>
    </div>
  );
}
