import ReactDOM from "react-dom/client";
import "./index.css";
import Providers from "./contexts/providers.tsx";
import { RouterProvider } from "react-router-dom";
import router from "./routes/index.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <Providers>
    <RouterProvider router={router} />
  </Providers>
  
);
