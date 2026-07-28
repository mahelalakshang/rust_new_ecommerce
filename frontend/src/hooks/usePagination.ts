import { useState } from "react";

const DEFAULT_LIMIT = 10;

export function usePagination(limit: number = DEFAULT_LIMIT) {
  const [offset, setOffset] = useState(0);

  const nextPage = () => setOffset((prev) => prev + limit);
  const prevPage = () => setOffset((prev) => Math.max(0, prev - limit));
  const reset = () => setOffset(0);

  return { limit, offset, setOffset, nextPage, prevPage, reset };
}
