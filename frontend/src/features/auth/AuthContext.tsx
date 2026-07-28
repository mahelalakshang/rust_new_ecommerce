import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";

import { setAuthTokenGetter, setOnUnauthorized } from "@/lib/api-client";
import * as authApi from "@/features/auth/api";
import type { User } from "@/types/auth";

const TOKEN_STORAGE_KEY = "auth_token";

type AuthStatus = "idle" | "loading" | "authenticated" | "unauthenticated";

interface AuthContextValue {
  token: string | null;
  user: User | null;
  status: AuthStatus;
  isAdmin: boolean;
  login: (username: string, password: string) => Promise<void>;
  signup: (username: string, password: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [token, setToken] = useState<string | null>(null);
  const [user, setUser] = useState<User | null>(null);
  const [status, setStatus] = useState<AuthStatus>("idle");

  const tokenRef = useRef<string | null>(null);
  tokenRef.current = token;

  const logout = useCallback(() => {
    localStorage.removeItem(TOKEN_STORAGE_KEY);
    setToken(null);
    setUser(null);
    setStatus("unauthenticated");
  }, []);

  useEffect(() => {
    setAuthTokenGetter(() => tokenRef.current);
    setOnUnauthorized(() => logout());
  }, [logout]);

  useEffect(() => {
    const stored = localStorage.getItem(TOKEN_STORAGE_KEY);
    if (!stored) {
      setStatus("unauthenticated");
      return;
    }

    setToken(stored);
    // Set the ref synchronously: `setToken` won't reach `tokenRef` until the next
    // render, and `fetchMe` below reads the ref *now* to build the auth header.
    tokenRef.current = stored;
    setStatus("loading");
    authApi
      .fetchMe()
      .then((me) => {
        setUser(me);
        setStatus("authenticated");
      })
      .catch(() => {
        localStorage.removeItem(TOKEN_STORAGE_KEY);
        setToken(null);
        setUser(null);
        setStatus("unauthenticated");
      });
  }, []);

  const authenticate = useCallback(async (authPromise: Promise<{ token: string }>) => {
    setStatus("loading");
    const { token: newToken } = await authPromise;
    localStorage.setItem(TOKEN_STORAGE_KEY, newToken);
    setToken(newToken);
    tokenRef.current = newToken;
    const me = await authApi.fetchMe();
    setUser(me);
    setStatus("authenticated");
  }, []);

  const login = useCallback(
    (username: string, password: string) => authenticate(authApi.login({ username, password })),
    [authenticate],
  );

  const signup = useCallback(
    (username: string, password: string) => authenticate(authApi.signup({ username, password })),
    [authenticate],
  );

  const value: AuthContextValue = {
    token,
    user,
    status,
    isAdmin: user?.role === "admin",
    login,
    signup,
    logout,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return ctx;
}
