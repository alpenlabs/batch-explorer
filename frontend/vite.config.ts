import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  optimizeDeps: {
    include: ["@tanstack/react-query"],
  },
  server: {
    allowedHosts: [
      "checkpoints.testnet.alpenlabs.io",
      "checkpoints.testnet-staging.stratabtc.org",
    ],
  },
});
