import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { formatCurrency } from "@/lib/decimal";

interface CartSummaryProps {
  total: number;
  itemCount: number;
}

export function CartSummary({ total, itemCount }: CartSummaryProps) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Order Summary</CardTitle>
      </CardHeader>
      <CardContent className="flex justify-between text-sm text-muted-foreground">
        <span>Items ({itemCount})</span>
        <span>{formatCurrency(total)}</span>
      </CardContent>
      <CardFooter className="flex justify-between border-t pt-4 text-lg font-semibold">
        <span>Total</span>
        <span>{formatCurrency(total)}</span>
      </CardFooter>
    </Card>
  );
}
