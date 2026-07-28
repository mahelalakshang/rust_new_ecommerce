# ShopFront

React + Vite + TypeScript storefront for the e-commerce API in `services/api`.

## Stack

- React Router v6, TanStack Query for server state, React Context for auth
- Tailwind CSS + shadcn/ui components
- react-hook-form + zod for form validation

## Setup

```bash
npm install
cp .env.example .env   # adjust VITE_API_URL if the backend isn't on 127.0.0.1:3001
npm run dev
```

The backend (`services/api`) must be running and reachable at `VITE_API_URL`, with CORS configured to allow this dev server's origin (`http://localhost:5173` by default — see `CORS_ORIGIN` in the backend's `.env`).

## Notes on the backend contract

- There is no public product/category browsing — every page except `/login` and `/signup` requires a logged-in user.
- `price`, `subtotal`, and cart `total` come back from the API as JSON strings (Rust `Decimal`); they're parsed via `src/lib/decimal.ts` before reaching any component. Raw vs. parsed types are split in `src/types/` (`ProductResponseRaw` vs `Product`, etc.) — always parse at the hook layer (`features/*/hooks.ts`), never in a component.
- Adding to cart (`POST /cart/`) increments existing quantity; updating from the cart page (`PATCH /cart/{id}`) sets it. See `src/features/cart/hooks.ts`.
- No checkout/orders flow exists yet — the cart page is the end of the purchase path.
- To test admin features, promote a user to `role = 'admin'` directly in Postgres (no UI for this exists).

## Structure

Feature-based: `src/features/{auth,products,categories,cart,admin}` each own their API calls (`api.ts`), data hooks (`hooks.ts`), and pages/components. Shared code lives in `src/components`, `src/lib`, `src/types`, `src/hooks`.
