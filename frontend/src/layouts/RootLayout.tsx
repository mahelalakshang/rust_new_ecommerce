import { Outlet } from "react-router-dom";

import { Header } from "@/components/layout/Header";
import { Sidebar } from "@/components/layout/Sidebar";

export function RootLayout() {
  return (
    <div className="flex min-h-svh flex-col">
      <Header />
      <div className="mx-auto flex w-full max-w-6xl flex-1 gap-6 px-4 py-6">
        <Sidebar />
        {/* min-w-0 stops grid children from forcing the flex parent wider than the viewport */}
        <main className="min-w-0 flex-1">
          <Outlet />
        </main>
      </div>
    </div>
  );
}
