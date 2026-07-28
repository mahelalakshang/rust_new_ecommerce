import { request } from "@/lib/api-client";
import type { AuthResponse, LoginRequest, SignupRequest, User } from "@/types/auth";

export function signup(data: SignupRequest): Promise<AuthResponse> {
  return request<AuthResponse>("/auth/signup", { method: "POST", body: data, auth: false });
}

export function login(data: LoginRequest): Promise<AuthResponse> {
  return request<AuthResponse>("/auth/login", { method: "POST", body: data, auth: false });
}

export function fetchMe(): Promise<User> {
  return request<User>("/auth/me");
}
