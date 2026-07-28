import { Link, useNavigate } from "react-router-dom";
import { ShoppingCart, Store } from "lucide-react";

import { Button } from "@/components/ui/button";
import { MobileNav } from "@/components/layout/MobileNav";
import { useAuth } from "@/features/auth/AuthContext";
import { CartBadge } from "@/features/cart/components/CartBadge";

export function Header() {
  const { status, user, logout } = useAuth();
  const navigate = useNavigate();
  const isAuthenticated = status === "authenticated";

  const handleLogout = () => {
    logout();
    navigate("/login");
  };

  return (
    <header className="sticky top-0 z-10 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="mx-auto flex h-14 max-w-6xl items-center justify-between gap-2 px-4">
        <div className="flex min-w-0 items-center gap-1">
          {isAuthenticated && <MobileNav />}
          <Link to="/" className="flex items-center gap-2 font-semibold">
            <Store className="size-5 shrink-0" />
            ShopFront
          </Link>
        </div>

        <div className="flex shrink-0 items-center gap-3">
          {isAuthenticated ? (
            <>
              <Link to="/cart" className="relative flex items-center">
                <ShoppingCart className="size-5" />
                <CartBadge />
              </Link>
              <span className="hidden text-sm text-muted-foreground sm:inline">
                {user?.username}
              </span>
              <Button variant="outline" size="sm" onClick={handleLogout}>
                Logout
              </Button>
            </>
          ) : (
            <>
              <Button variant="ghost" size="sm" asChild>
                <Link to="/login">Log in</Link>
              </Button>
              <Button size="sm" asChild>
                <Link to="/signup">Sign up</Link>
              </Button>
            </>
          )}
        </div>
      </div>
    </header>
  );
}
