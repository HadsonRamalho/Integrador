import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { User } from "@/interfaces/user";
import "@/components/user-profile/user-profile.css";
import { loadUserById } from "@/services/api/user/user";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";



export default function UserProfile() {
  const [user, setUser] = useState<User>();
  const [error, setError] = useState(false);
  const [updatedData, setUpdatedData] = useState(true);

  async function AtualizaUsuario(nome_novo: string, email_novo: string, documento_novo: string, senha: string) {
    try {
      const res = await fetch(
        "https://g6v9psc0-3003.brs.devtunnels.ms/atualiza_usuario",
        {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            email_antigo: user?.email,
            email_novo: email_novo,
            nome_novo: nome_novo,
            documento_novo: documento_novo,
            senha: senha
          }),
        }
      );
      if (!res.ok) {
        const erro = await res.text();
        console.log("Erro ao atualizar: ", erro);
        throw new Error(erro);
      }
      console.log("Conta atualizada!");
      setUpdatedData(true);
    } catch (erro) {
      console.error(erro);
    }
  }

  useEffect(() => {
    const loadUser = async () => {
      setUpdatedData(false);
      const id = localStorage.getItem("USER_ID");
      if(!id){
        return;
      }
      try{
        const user = await loadUserById(id);
        console.log(user);
        setUser(user);
      } catch(err){
        setError(true);
        console.error(err);
        console.log(error);
      }
    };
    loadUser();    
  }, [updatedData]);

  interface UserCardProps {
    user: User;
  }
  
  const UserCard: React.FC<UserCardProps> = ({ user }) => {
  const [nome, setNome] = useState(user.nome);
  const [email, setEmail] = useState(user.email);
  const [documento, setDocumento] = useState(user.documento);
  const [senha, setSenha] = useState("");
  const origemConta = user.origemconta;

  const handleChange = async() => {
    if(user.origemconta === 'Sistema'){
      await AtualizaUsuario(nome, email, documento, senha);
      return;
    }
    await AtualizaUsuario(nome, email, documento, email);     
  }

    return (
      <Card className="user-profile-card">
        <CardHeader>
          <CardTitle className="user-profile-card-header">Minhas Informações</CardTitle>
        </CardHeader>
        <CardContent className="user-profile-card-content">
          <Avatar>
            <AvatarImage className="user-profile-card-image" src={localStorage.getItem("PROFILE_IMAGE_URL") || "https://i.pinimg.com/736x/f1/13/b7/f113b7eb12a6e28b201152535c8b89da.jpg"} />                    
          </Avatar>
          <Label htmlFor="nome">Nome</Label>
          <Input id="nome"
           value={nome}
            onChange={(e) => setNome(e.target.value)}/>

          <Label htmlFor="e-mail">E-mail</Label>
          <Input
          id="email"
          readOnly={true}
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          />
          <Label htmlFor="documento">Documento</Label>
          <Input
          id="documento"
          value={documento}
          onChange={(e) => setDocumento(e.target.value)}/>

          { origemConta === "Sistema" ? (
            <>
            <Label htmlFor="senha">Senha</Label>
            <Input
            id="senha"
            type="password"
            value={senha}
            onChange={(e) => setSenha(e.target.value)}
            />
          </>) : (<></>)}

          <CardDescription className="user-profile-card-description">
            Data de Cadastro: {user.datacadastro}
          </CardDescription>
          <CardContent>
            <Button className="user-profile-button" onClick={handleChange}>Editar minhas informações</Button>
          </CardContent>
        </CardContent>
      </Card>
    );
  };
  
  return (
    <Layout>
      <main>        
        <div className="user-profile-container">
        <div>
          {user ? (
            <UserCard user={user} />
            ) : error ? (
            <div>
                <p>Houve um erro ao carregar o usuário. Reporte o problema aqui:</p>
                <Button>Relatar problema</Button>
            </div>
            ) : (
            <p>Carregando usuário...</p>
        )}
        </div>      
        </div>     
      </main>
    </Layout>
  );
}
