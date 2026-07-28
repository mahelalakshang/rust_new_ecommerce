import { AlertTriangle } from "lucide-react";

import { ApiError } from "@/lib/api-client";

export function ErrorState({ error }: { error: unknown }) {
  const message =
    error instanceof ApiError ? error.body?.error ?? error.message : "Something went wrong.";

  return (
    <div className="flex flex-col items-center gap-2 rounded-lg border border-dashed py-12 text-center text-muted-foreground">
      <AlertTriangle className="size-6" />
      <p>{message}</p>
    </div>
  );
}
