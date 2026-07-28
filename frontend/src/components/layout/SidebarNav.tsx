import { NavLink } from "react-router-dom";

import { getNavItems } from "@/components/layout/nav-config";
import { useAuth } from "@/features/auth/AuthContext";
import { cn } from "@/lib/utils";

interface SidebarNavProps {
  /** Called after a link is clicked — the mobile drawer uses this to close itself. */
  onNavigate?: () => void;
}

export function SidebarNav({ onNavigate }: SidebarNavProps) {
  const { isAdmin } = useAuth();
  const items = getNavItems(isAdmin);

  return (
    <nav className="flex flex-col gap-1">
      {items.map(({ to, label, icon: Icon, end }) => (
        <NavLink
          key={to}
          to={to}
          end={end}
          onClick={onNavigate}
          className={({ isActive }) =>
            cn(
              "flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors",
              isActive
                ? "bg-accent text-accent-foreground"
                : "text-muted-foreground hover:bg-accent/50 hover:text-foreground",
            )
          }
        >
          <Icon className="size-4 shrink-0" />
          {label}
        </NavLink>
      ))}
    </nav>
  );
}
