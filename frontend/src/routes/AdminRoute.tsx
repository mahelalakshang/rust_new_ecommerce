import { useEffect } from "react";
import { Navigate, Outlet } from "react-router-dom";
import { toast } from "sonner";

import { useAuth } from "@/features/auth/AuthContext";

export function AdminRoute() {
  const { isAdmin } = useAuth();

  useEffect(() => {
    if (!isAdmin) {
      toast.error("Admins only");
    }
  }, [isAdmin]);

  if (!isAdmin) {
    return <Navigate to="/" replace />;
  }

  return <Outlet />;
}
