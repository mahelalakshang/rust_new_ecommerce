import { ClipboardList, FolderPlus, Package, PackagePlus, type LucideIcon } from "lucide-react";

export interface NavItem {
  to: string;
  label: string;
  icon: LucideIcon;
  /** Match this route exactly — needed for "/" so it isn't active on every page. */
  end?: boolean;
}

/** Single source of truth for sidebar navigation. Add new pages here. */
export function getNavItems(isAdmin: boolean): NavItem[] {
  const items: NavItem[] = [
    { to: "/", label: "Products", icon: Package, end: true },
    { to: "/orders", label: "Orders", icon: ClipboardList },
  ];

  if (isAdmin) {
    items.push(
      { to: "/admin/categories/new", label: "New Category", icon: FolderPlus },
      { to: "/admin/products/new", label: "New Product", icon: PackagePlus },
    );
  }

  return items;
}
