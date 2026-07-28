import { Link } from "react-router-dom";

import { Badge } from "@/components/ui/badge";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { formatCurrency } from "@/lib/decimal";
import type { Product } from "@/types/product";

export function ProductCard({ product }: { product: Product }) {
  const outOfStock = product.stock_quantity === 0;

  return (
    <Link to={`/products/${product.id}`}>
      <Card className="h-full py-4 transition-shadow hover:shadow-md">
        <CardHeader className="px-4">
          <CardTitle className="line-clamp-1 text-base">{product.name}</CardTitle>
        </CardHeader>
        <CardContent className="px-4">
          <p className="line-clamp-2 text-sm text-muted-foreground">{product.description}</p>
        </CardContent>
        <CardFooter className="justify-between px-4">
          <span className="font-semibold">{formatCurrency(product.price)}</span>
          {outOfStock ? (
            <Badge variant="destructive">Out of stock</Badge>
          ) : (
            <Badge variant="secondary">{product.stock_quantity} in stock</Badge>
          )}
        </CardFooter>
      </Card>
    </Link>
  );
}
