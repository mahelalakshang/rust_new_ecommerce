import { Outlet } from "react-router-dom";

export function AuthLayout() {
  return (
    <div className="flex min-h-svh items-center justify-center bg-muted/40 p-4">
      <Outlet />
    </div>
  );
}
