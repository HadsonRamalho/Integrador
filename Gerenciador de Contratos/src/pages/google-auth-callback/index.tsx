import React, { useEffect, useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import Layout from '@/layouts/default';

function GoogleAuthCallback() {
  const [searchParams] = useSearchParams();
  const [message, setMessage] = useState("Carregando...");
  useEffect(() => {
    const sendCodeToBackend = async () => {
      const code = searchParams.get('code');
      console.log('code: ', code);

      if (!code) {
        console.error('Código não encontrado na URL');
        return;
      }

      try {
        const res = await fetch(
            "https://g6v9psc0-3003.brs.devtunnels.ms/auth/google",
            {
              method: "POST",
              headers: {
                "Content-Type": "application/json",
              },
              body: JSON.stringify({
                code: code
              }),
            }
          );
          console.log('autenticando...');
        if (!res.ok) {
            const erro = await res.text();
            console.log("Erro ao tentar autenticar: ", erro);
            throw new Error(erro);
        }
        console.log("Autenticado!");
        setMessage('autenticado');
        alert('Autenticação realizada com sucesso!');
      } catch (error) {
        console.error('Erro ao enviar código ao backend:', error);
        alert('Falha na autenticação!');
        setMessage('erro ao autenticar');
      }
    };

    sendCodeToBackend();
  }, [searchParams]);

  return (
    <Layout>
      <div>
        <h1>Autenticando...</h1>
        <p style={{color: 'white'}}>{message}</p>
      </div>
    </Layout>
  );
}

export default GoogleAuthCallback;
