import { useEffect } from "react";
import { useAuth } from "@/hooks/auth";
import { useNavigate } from "react-router-dom";

export function Prohibited() {
  const { user } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    if (user) {
      navigate("/logado");
    }
  }, [user, navigate]);

  return <h1>entrada proibida</h1>;
}
