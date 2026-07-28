import { createBrowserRouter } from "react-router-dom";

import { RootLayout } from "@/layouts/RootLayout";
import { AuthLayout } from "@/layouts/AuthLayout";
import { ProtectedRoute } from "@/routes/ProtectedRoute";
import { AdminRoute } from "@/routes/AdminRoute";
import { NotFoundPage } from "@/routes/NotFoundPage";
import { LoginPage } from "@/features/auth/LoginPage";
import { SignupPage } from "@/features/auth/SignupPage";
import { ProductListPage } from "@/features/products/ProductListPage";
import { ProductDetailPage } from "@/features/products/ProductDetailPage";
import { CartPage } from "@/features/cart/CartPage";
import { CreateCategoryPage } from "@/features/admin/CreateCategoryPage";
import { CreateProductPage } from "@/features/admin/CreateProductPage";

export const router = createBrowserRouter([
  {
    element: <AuthLayout />,
    children: [
      { path: "/login", element: <LoginPage /> },
      { path: "/signup", element: <SignupPage /> },
    ],
  },
  {
    element: <RootLayout />,
    children: [
      {
        element: <ProtectedRoute />,
        children: [
          { path: "/", element: <ProductListPage /> },
          { path: "/products/:id", element: <ProductDetailPage /> },
          { path: "/cart", element: <CartPage /> },
          {
            element: <AdminRoute />,
            children: [
              { path: "/admin/categories/new", element: <CreateCategoryPage /> },
              { path: "/admin/products/new", element: <CreateProductPage /> },
            ],
          },
        ],
      },
      { path: "*", element: <NotFoundPage /> },
    ],
  },
]);
