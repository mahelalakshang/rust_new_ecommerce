import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { CategoryForm } from "@/features/admin/components/CategoryForm";

export function CreateCategoryPage() {
  return (
    <Card className="mx-auto max-w-md">
      <CardHeader>
        <CardTitle>New Category</CardTitle>
        <CardDescription>Create a category products can be assigned to.</CardDescription>
      </CardHeader>
      <CardContent>
        <CategoryForm />
      </CardContent>
    </Card>
  );
}
