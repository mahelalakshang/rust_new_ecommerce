import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { ProductForm } from "@/features/admin/components/ProductForm";

export function CreateProductPage() {
  return (
    <Card className="mx-auto max-w-md">
      <CardHeader>
        <CardTitle>New Product</CardTitle>
        <CardDescription>Add a product to the catalog.</CardDescription>
      </CardHeader>
      <CardContent>
        <ProductForm />
      </CardContent>
    </Card>
  );
}
