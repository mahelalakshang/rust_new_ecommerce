import { Button } from "@/components/ui/button";

interface PaginationProps {
  total: number;
  offset: number;
  itemsLength: number;
  onPrev: () => void;
  onNext: () => void;
}

export function Pagination({ total, offset, itemsLength, onPrev, onNext }: PaginationProps) {
  if (total === 0) return null;

  const start = offset + 1;
  const end = offset + itemsLength;
  const hasPrev = offset > 0;
  const hasNext = offset + itemsLength < total;

  return (
    <div className="flex items-center justify-between gap-4 pt-4">
      <p className="text-sm text-muted-foreground">
        Showing {start}–{end} of {total}
      </p>
      <div className="flex gap-2">
        <Button variant="outline" size="sm" onClick={onPrev} disabled={!hasPrev}>
          Previous
        </Button>
        <Button variant="outline" size="sm" onClick={onNext} disabled={!hasNext}>
          Next
        </Button>
      </div>
    </div>
  );
}
