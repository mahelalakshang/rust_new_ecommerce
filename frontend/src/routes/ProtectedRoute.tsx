import { Navigate, Outlet, useLocation } from "react-router-dom";

import { FullPageSpinner } from "@/components/feedback/LoadingSpinner";
import { useAuth } from "@/features/auth/AuthContext";

export function ProtectedRoute() {
  const { status } = useAuth();
  const location = useLocation();

  if (status === "idle" || status === "loading") {
    return <FullPageSpinner />;
  }

  if (status === "unauthenticated") {
    return <Navigate to="/login" replace state={{ from: location }} />;
  }

  return <Outlet />;
}
