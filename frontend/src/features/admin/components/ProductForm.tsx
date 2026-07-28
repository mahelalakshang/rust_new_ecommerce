import { useState } from "react";
import { useForm, Controller } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { z } from "zod";
import { toast } from "sonner";
import { Link } from "react-router-dom";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { LoadingSpinner } from "@/components/feedback/LoadingSpinner";
import { useCategories } from "@/features/categories/hooks";
import { useCreateProduct } from "@/features/products/hooks";
import { ApiError } from "@/lib/api-client";

const schema = z.object({
  name: z.string().min(2, "Name must be at least 2 characters"),
  description: z.string().min(1, "Description is required"),
  category_id: z.string().min(1, "Please select a category"),
  price: z.number({ error: "Price is required" }).nonnegative("Price must be 0 or more"),
  stock_quantity: z
    .number({ error: "Stock quantity is required" })
    .int("Must be a whole number")
    .nonnegative("Stock quantity must be 0 or more"),
});

type FormValues = z.infer<typeof schema>;

export function ProductForm() {
  const { data: categories, isPending: categoriesPending } = useCategories({
    limit: 100,
    offset: 0,
  });
  const createProduct = useCreateProduct();
  const [formError, setFormError] = useState<string | null>(null);

  const {
    register,
    control,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<FormValues>({ resolver: zodResolver(schema) });

  const onSubmit = async (values: FormValues) => {
    setFormError(null);
    try {
      await createProduct.mutateAsync(values);
      toast.success(`Product "${values.name}" created`);
      reset();
    } catch (err) {
      setFormError(err instanceof ApiError ? err.body?.error ?? err.message : "Something went wrong");
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-4">
      {formError && (
        <div className="rounded-md border border-destructive/30 bg-destructive/10 px-3 py-2 text-sm text-destructive">
          {formError}
        </div>
      )}

      <div className="flex flex-col gap-2">
        <Label htmlFor="name">Name</Label>
        <Input id="name" {...register("name")} />
        {errors.name && <p className="text-sm text-destructive">{errors.name.message}</p>}
      </div>

      <div className="flex flex-col gap-2">
        <Label htmlFor="description">Description</Label>
        <Textarea id="description" {...register("description")} />
        {errors.description && (
          <p className="text-sm text-destructive">{errors.description.message}</p>
        )}
      </div>

      <div className="flex flex-col gap-2">
        <Label htmlFor="category">Category</Label>
        {categoriesPending ? (
          <LoadingSpinner className="size-4" />
        ) : (
          <Controller
            control={control}
            name="category_id"
            render={({ field }) => (
              <Select onValueChange={field.onChange} value={field.value}>
                <SelectTrigger id="category">
                  <SelectValue placeholder="Select a category" />
                </SelectTrigger>
                <SelectContent>
                  {categories?.items.map((category) => (
                    <SelectItem key={category.id} value={category.id}>
                      {category.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            )}
          />
        )}
        {errors.category_id && (
          <p className="text-sm text-destructive">{errors.category_id.message}</p>
        )}
        {!categoriesPending && categories?.items.length === 0 && (
          <p className="text-sm text-muted-foreground">
            No categories yet —{" "}
            <Link to="/admin/categories/new" className="underline underline-offset-4">
              create one first
            </Link>
            .
          </p>
        )}
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="flex flex-col gap-2">
          <Label htmlFor="price">Price</Label>
          <Input
            id="price"
            type="number"
            step="0.01"
            min="0"
            {...register("price", { valueAsNumber: true })}
          />
          {errors.price && <p className="text-sm text-destructive">{errors.price.message}</p>}
        </div>

        <div className="flex flex-col gap-2">
          <Label htmlFor="stock_quantity">Stock quantity</Label>
          <Input
            id="stock_quantity"
            type="number"
            step="1"
            min="0"
            {...register("stock_quantity", { valueAsNumber: true })}
          />
          {errors.stock_quantity && (
            <p className="text-sm text-destructive">{errors.stock_quantity.message}</p>
          )}
        </div>
      </div>

      <Button type="submit" disabled={isSubmitting} className="w-fit">
        {isSubmitting ? "Creating…" : "Create product"}
      </Button>
    </form>
  );
}
