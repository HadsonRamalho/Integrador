import { useEffect, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import Layout from "@/layouts/default";
import { useAuth } from "@/hooks/auth";

function GoogleAuthCallback() {
  const [searchParams] = useSearchParams();
  const [message, setMessage] = useState("Carregando...");
  const API_URL = "http://localhost:3003";
  const navigate = useNavigate();
  const { signIn } = useAuth();
  const loadUserId = async (email: string) => {
    try {
      const res = await fetch(
        `${API_URL}/busca_usuario_email/?email=${encodeURIComponent(email)}`,
        {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
        },
      );

      if (!res.ok) {
        throw new Error(await res.text());
      }

      const userid = await res.json();
      console.log("idusuario: ", userid);
      localStorage.setItem("USER_ID", userid);
      const credentials = { email: email, password: email };
      await signIn(credentials);
      return userid;
    } catch (error) {
      console.error(error);
    }
  };

  useEffect(() => {
    const sendCodeToBackend = async () => {
      const code = searchParams.get("code");
      console.log("code: ", code);

      if (!code) {
        console.error("Código não encontrado na URL");
        return;
      }

      try {
        const res = await fetch("http://localhost:3003/auth/google", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            code: code,
          }),
        });
        console.log("autenticando...");
        if (!res.ok) {
          const erro = await res.text();
          console.log("Erro ao tentar autenticar: ", erro);
          throw new Error(erro);
        }
        const obj = await res.json();
        console.log("email: ", obj.email);
        console.log("Autenticado!");
        setMessage("autenticado");
        alert("Autenticação realizada com sucesso!");
        await loadUserId(obj.email);
        localStorage.setItem("PROFILE_IMAGE_URL", obj.picture);
        navigate("/user-profile");
      } catch (error) {
        console.error("Erro ao enviar código ao backend:", error);
        alert("Falha na autenticação!");
        setMessage("erro ao autenticar");
      }
    };

    sendCodeToBackend();
  }, [searchParams]);

  return (
    <Layout>
      <div>
        <h1>Autenticando...</h1>
        <p style={{ color: "white" }}>{message}</p>
      </div>
    </Layout>
  );
}

export default GoogleAuthCallback;
