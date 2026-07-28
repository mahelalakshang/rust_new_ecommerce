import { EmptyState } from "@/components/feedback/EmptyState";
import { ErrorState } from "@/components/feedback/ErrorState";
import { Pagination } from "@/components/layout/Pagination";
import { ProductGrid, ProductGridSkeleton } from "@/features/products/components/ProductGrid";
import { useProducts } from "@/features/products/hooks";
import { usePagination } from "@/hooks/usePagination";

export function ProductListPage() {
  const { limit, offset, nextPage, prevPage } = usePagination();
  const { data, isPending, isError, error } = useProducts({ limit, offset });

  return (
    <div className="flex flex-col gap-6">
      <h1 className="text-2xl font-semibold">Products</h1>

      {isPending && <ProductGridSkeleton />}

      {isError && <ErrorState error={error} />}

      {data && data.items.length === 0 && <EmptyState message="No products yet." />}

      {data && data.items.length > 0 && (
        <>
          <ProductGrid products={data.items} />
          <Pagination
            total={data.total}
            offset={offset}
            itemsLength={data.items.length}
            onPrev={prevPage}
            onNext={nextPage}
          />
        </>
      )}
    </div>
  );
}
