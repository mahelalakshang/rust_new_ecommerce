/** Parses a backend Decimal-as-string field (e.g. product price, cart total). */
export function parseDecimal(value: string): number {
  const parsed = Number(value);
  if (Number.isNaN(parsed)) {
    throw new Error(`Expected a numeric decimal string from the API, got: "${value}"`);
  }
  return parsed;
}

export function formatCurrency(value: number, currency = "USD"): string {
  return new Intl.NumberFormat("en-US", { style: "currency", currency }).format(value);
}
