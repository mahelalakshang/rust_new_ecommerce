export type Role = "user" | "admin";

export interface User {
  id: string;
  username: string;
  role: Role;
  created_at: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface SignupRequest {
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
}
