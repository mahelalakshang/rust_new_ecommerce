const API_URL = import.meta.env.VITE_API_URL;

if (!API_URL) {
  throw new Error("VITE_API_URL is not set — check your .env file (see .env.example)");
}

export { API_URL };
