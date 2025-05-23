import ReactDOM from "react-dom/client";
import "./index.css";
import Providers from "./contexts/providers.tsx";
import { RouterProvider } from "react-router-dom";
import router from "./routes/index.tsx";
import { ThemeProvider } from "./components/theme-provider/theme-provider.tsx";
import { PwaInstallProvider } from "./contexts/PwaInstallContext";
import InstallPrompt from "./components/install-prompt/index.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <Providers>
    <PwaInstallProvider>
      <ThemeProvider defaultTheme="light" storageKey="vite-ui-theme">
        <RouterProvider router={router} />
        <InstallPrompt />
      </ThemeProvider>
    </PwaInstallProvider>
  </Providers>
);
