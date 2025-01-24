import About from "@/pages/about";
import Logged from "@/pages/logged";
import Home from "@/pages/home";
import Machine  from "@/pages/machine";
import Login from "@/pages/login";
import { createBrowserRouter } from "react-router-dom";
import PrivateRoute from "./privateRoute";
import { Prohibited } from "@/pages/prohibited";
import PasswordRecovery from "@/pages/password-recovery/index.js";
import CreateMachine from "@/pages/create-machine";
import UserProfile from "@/pages/user-profile";
import GoogleAuthCallback from "@/pages/google-auth-callback";


const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />,
    errorElement: <h1>404</h1>,
  },
  {
    path: "/sobre",
    element: <About />,
  },
  {
    path: "/machine",
    element: <Machine/>,
  },
  {
    path: "/password-recovery",
    element: <PasswordRecovery/>,
  },
  {
    path: "/login",
    element: <Login/>,
  },
  {
    path: "/create-machine",
    element: (
      <PrivateRoute>
        <CreateMachine/>
      </PrivateRoute>)
  },
  {
    path: "/user-profile",
    element: (
      <PrivateRoute>
        <UserProfile/>
      </PrivateRoute>)
  },
  {
    path: "/logado",
    element: (
      <PrivateRoute>
        <Logged />,
      </PrivateRoute>
    ),
  },
  {
    path: "/auth/google/callback",
    element: (
      <GoogleAuthCallback/>
    )
  },

  {
    path: "/entrada-proibida",
    element: <Prohibited />,
  },
]);

export default router;
