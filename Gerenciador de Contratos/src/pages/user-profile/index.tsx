import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { User } from "@/interfaces/user";
import "@/components/user-profile/user-profile.css";
import { loadUserById } from "@/services/api/user/user";
import { Avatar, AvatarImage } from "@radix-ui/react-avatar";

export default function UserProfile() {
  const [user, setUser] = useState<User>();
  const [error, setError] = useState(false);

  useEffect(() => {
    const loadUser = async () => {
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
  }, []);

  interface UserCardProps {
    user: User;
  }
  
  const UserCard: React.FC<UserCardProps> = ({ user }) => {
    return (
      <Card className="user-profile-card">
        <CardHeader>
          <CardTitle className="user-profile-card-header">Minhas Informações</CardTitle>
        </CardHeader>
        <CardContent className="user-profile-card-content">
          <Avatar>
            <AvatarImage className="user-profile-card-image" src="https://i.pinimg.com/736x/f1/13/b7/f113b7eb12a6e28b201152535c8b89da.jpg" />                    
          </Avatar>
          <h1>Nome: {user.nome}</h1>
          <p>E-mail: {user.email}</p>
          Documento: {user.documento}
          <CardDescription className="user-profile-card-description">
            Data de Cadastro: {user.datacadastro}
          </CardDescription>
          <CardContent>
            <Button className="user-profile-button">Editar minhas informações</Button>
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
