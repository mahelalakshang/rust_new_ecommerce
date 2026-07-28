import { API_URL } from "@/config/env";
import type { ApiErrorBody } from "@/types/common";

export class ApiError extends Error {
  status: number;
  body: ApiErrorBody | null;

  constructor(status: number, message: string, body: ApiErrorBody | null) {
    super(message);
    this.name = "ApiError";
    this.status = status;
    this.body = body;
  }
}

let getAuthToken: () => string | null = () => null;
let onUnauthorized: () => void = () => {};

/** Called once by AuthProvider so the client can read the current token without a circular import. */
export function setAuthTokenGetter(fn: () => string | null) {
  getAuthToken = fn;
}

/** Called once by AuthProvider; invoked whenever any request comes back 401. */
export function setOnUnauthorized(fn: () => void) {
  onUnauthorized = fn;
}

interface RequestOptions {
  method?: "GET" | "POST" | "PATCH" | "DELETE";
  body?: unknown;
  /** Set false for endpoints that don't require/accept a token (signup/login). Defaults to true. */
  auth?: boolean;
}

export async function request<T>(path: string, options: RequestOptions = {}): Promise<T> {
  const { method = "GET", body, auth = true } = options;

  const headers: Record<string, string> = {};
  if (body !== undefined) {
    headers["Content-Type"] = "application/json";
  }
  if (auth) {
    const token = getAuthToken();
    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }
  }

  const res = await fetch(`${API_URL}${path}`, {
    method,
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
  });

  if (!res.ok) {
    let errorBody: ApiErrorBody | null = null;
    let message = res.statusText;
    try {
      errorBody = (await res.json()) as ApiErrorBody;
      message = errorBody.error ?? message;
    } catch {
      const text = await res.text().catch(() => "");
      if (text) message = text;
    }

    if (res.status === 401) {
      onUnauthorized();
    }

    throw new ApiError(res.status, message, errorBody);
  }

  if (res.status === 204) {
    return undefined as T;
  }

  return (await res.json()) as T;
}
