import { SidebarNav } from "@/components/layout/SidebarNav";
import { useAuth } from "@/features/auth/AuthContext";

export function Sidebar() {
  const { status } = useAuth();

  // NotFoundPage renders inside RootLayout but outside ProtectedRoute, so this
  // can be reached while logged out.
  if (status !== "authenticated") return null;

  return (
    <aside className="hidden w-56 shrink-0 lg:block">
      <div className="sticky top-20">
        <SidebarNav />
      </div>
    </aside>
  );
}
