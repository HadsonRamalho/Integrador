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
import { useNavigate } from "react-router-dom";
import { Address, CreateUserAddress } from "@/interfaces/address";
import { createUserAddress, loadAddressByCep, loadAddressUserId, updateAddress } from "@/services/api/address/address";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";

export default function UserProfile() {
  const [user, setUser] = useState<User>();
  const [error, setError] = useState(false);
  const [updatedData, setUpdatedData] = useState(true);
  const defaultAddress: Address = {
    idendereco: "",
    pais: "",
    estado: "",
    cidade: "",
    cep: "",
    bairro: "",
    logradouro: "",
    numero: "",
    complemento: ""
  };
  const [address, setAddress] = useState<Address | null>(defaultAddress);

  const [pais, setPais] = useState<string>("Brasil");
  const [estado, setEstado] = useState<string>();
  const [cidade, setCidade] = useState<string>();
  const [bairro, setBairro] = useState<string>();
  const [cep, setCep] = useState<string>();
  const [logradouro, setLogradouro] = useState<string>();
  const [numero, setNumero] = useState<string>();
  const [complemento, setComplemento] = useState<string>();

  const navigate = useNavigate();

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

  async function DeletaUsuario(id: string) {
    try {
      const res = await fetch(
        `https://g6v9psc0-3003.brs.devtunnels.ms/deleta_usuario/?id=${encodeURIComponent(id)}`,
        {
          method: "DELETE",
          headers: {
            "Content-Type": "application/json",
          },          
        }
      );
      if (!res.ok) {
        const erro = await res.text();
        console.log("Erro ao deletar: ", erro);
        throw new Error(erro);
      }
      console.log("Conta deletada!");
      alert("Conta deletada com sucesso!");
      navigate('/login');
    } catch (erro) {
      console.error(erro);
    }
  }

  useEffect(() => {
    async function buscaEndereco(id: string) {
      try{
        const res = await loadAddressUserId(id);
        console.log(res);
        const endereco = res;
        setAddress(endereco);
        console.log(endereco);
      } catch(error){
        setAddress(null);
        console.error(error);
      }
    }
    const loadUser = async () => {
      setUpdatedData(false);
      const id = localStorage.getItem("USER_ID");
      if(!id){
        return;
      }
      try{
        const user = await loadUserById(id);
        setUser(user);
        await buscaEndereco(id);
      } catch(err){
        setError(true);
        console.error(err);
      }
    };
    loadUser();    
  }, [updatedData]);

  interface UserCardProps {
    user: User;
  }

  interface AddressCardProps {
    user: User;
    address: Address;
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
      <Card className="user-profile-card rounded-xl border-[1px] border-[hsl(var(--primary))]">
        <CardHeader>
          <CardTitle className="user-profile-card-header">Minhas Informações</CardTitle>
        </CardHeader>
        <CardContent className="user-profile-card-content">
          <Avatar>
            <AvatarImage className="user-profile-card-image" src={localStorage.getItem("PROFILE_IMAGE_URL") || "https://i.pinimg.com/736x/f1/13/b7/f113b7eb12a6e28b201152535c8b89da.jpg"} />                    
          </Avatar>
          <Label htmlFor="nome" className="text-[hsl(var(--text))] ">Nome</Label>
          <Input id="nome"
           value={nome}
           className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4 "
            onChange={(e) => setNome(e.target.value)}/>

          <Label htmlFor="e-mail" className="text-[hsl(var(--text))]">E-mail</Label>
          <Input
          id="email"
          readOnly={true}
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          />
          <Label htmlFor="documento" className="text-[hsl(var(--text))]">Documento</Label>
          <Input
          id="documento"
          value={documento}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          onChange={(e) => setDocumento(e.target.value)}/>

          { origemConta === "Sistema" ? (
            <>
            <Label htmlFor="senha" className="text-[hsl(var(--text))]">Senha</Label>
            <Input
            id="senha"
            type="password"
            className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 "
            value={senha}
            onChange={(e) => setSenha(e.target.value)}
            />
          </>) : (<></>)}

          <CardDescription className="user-profile-card-description">
            Data de Cadastro: {user.datacadastro}
          </CardDescription>
          <CardContent>
            <Button className="user-profile-button" onClick={handleChange}>Atualizar informações do perfil</Button>
          </CardContent>
        </CardContent>
      </Card>
    );
  };

  const AddressCard: React.FC<AddressCardProps> = ({ user, address }) => {
    const [pais, setPais] = useState<string>("Brasil");
    const [estado, setEstado] = useState<string>(address.estado);
    const [cidade, setCidade] = useState<string>(address.cidade);
    const [bairro, setBairro] = useState<string>(address.bairro);
    const [cep, setCep] = useState<string>(address.cep);
    const [logradouro, setLogradouro] = useState<string>(address.logradouro);
    const [numero, setNumero] = useState<string>(address.numero);
    const [complemento, setComplemento] = useState<string>(address.complemento);

    const handleUpdateAddress = async () => {
      const newAddress = {
        idendereco: address.idendereco,
        bairro,
        cep,
        cidade,
        estado,
        pais,
        complemento: complemento || "Nenhum",
        logradouro,
        numero,
        idusuario: user.idusuario
      }
      const updatedAddress = await updateAddress(newAddress);
      if (!updatedAddress) {
        alert("Erro ao atualizar endereço");
        return;
      }
      setAddress(updatedAddress);
      alert("Endereço atualizado!");
    }

    return (
      <Card className="mt-2 border-[hsl(var(--primary))] bg-[hsl(var(--machine-card-bg))]">
        <CardHeader>
          <CardTitle className="text-[1.5rem] text-[hsl(var(--primary))]">Meu Endereço</CardTitle>
        </CardHeader>
        <CardContent>
          <Label htmlFor="cep" className="text-[hsl(var(--text))]">CEP</Label>
          <Input id="cep"
          value={cep}
          className="text-black  rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          onChange={(e) => setCep(e.target.value)}/>
          <Label htmlFor="pais" className="text-[hsl(var(--text))]">País</Label>
          <Input
          id="pais"
          readOnly={true}
          value={pais}
          onChange={(e) => setPais(e.target.value)}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          />
          <Label htmlFor="estado" className="text-[hsl(var(--text))]">Estado</Label>
          <Input
          id="estado"
          value={estado}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4" 
          onChange={(e) => setEstado(e.target.value)}/>
          <Label htmlFor="cidade" className="text-[hsl(var(--text))]">Cidade</Label>
          <Input
          id="cidade"
          value={cidade}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          onChange={(e) => setCidade(e.target.value)}/>
          <Label htmlFor="bairro" className="text-[hsl(var(--text))]">Bairro</Label>
          <Input
          id="bairro"
          value={bairro}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          onChange={(e) => setBairro(e.target.value)}/>
          <Label htmlFor="logradouro" className="text-[hsl(var(--text))]">Rua</Label>
          <Input
          id="logradouro"
          value={logradouro}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          onChange={(e) => setLogradouro(e.target.value)}/>
          <Label htmlFor="numero" className="text-[hsl(var(--text))]">Número</Label>
          <Input
          id="numero"
          value={numero}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          onChange={(e) => setNumero(e.target.value)}/>
          <Label htmlFor="complemento" className="text-[hsl(var(--text))]">Complemento</Label>
          <Input
          id="complemento"
          value={complemento}
          className="text-black rounded-md border-[1px] border-[hsl(var(--primary))] bg-neutral-100 mb-4"
          onChange={(e) => setComplemento(e.target.value)}/>
          <CardContent>
            <Button onClick={handleUpdateAddress} className="user-profile-button">Atualizar endereço</Button>
          </CardContent>
        </CardContent>
      </Card>
    );
  };
  
  const handleSubtmitAddress = async () => {
    if(!user){
      return;
    }
    if (!pais || !estado || !cidade || !bairro || !cep || !logradouro || !numero) {
      alert("Preencha todos os campos!");
      return;
    }
    const endereco: CreateUserAddress = {
      bairro,
      cep,
      cidade,
      estado,
      pais,
      complemento: complemento || "Nenhum",
      logradouro,
      numero,
      idusuario: user.idusuario
    };
    const res = await createUserAddress(endereco);
    if (!res) {
      alert("Erro ao cadastrar endereço");
      return;
    }
    setAddress(res);
    alert("Endereço cadastrado!");
  }

  const handleCepChange = async () => {
    if(!cep){
      return;
    }
    const endereco = await loadAddressByCep(cep);
    if (!endereco){
      alert("CEP inválido");
      return;
    }
    setEstado(endereco.uf);
    setCidade(endereco.localidade);
    setBairro(endereco.bairro);
    setLogradouro(endereco.logradouro);
  }
   
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
        {address && user ? (
          <AddressCard user={user} address={address} />
        ) : (
          <p>Carregando endereço...</p>
        )}
        </div>      
        </div>
        <div>
          <AlertDialog open={!address}>
            <AlertDialogTrigger asChild></AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle style={{ color: "hsl(var(--text))" }}>
                  Cadastre um endereço
                </AlertDialogTitle>
                <AlertDialogDescription style={{ color: "hsl(var(--text))" }}>
                  Você precisa cadastrar um endereço para utilizar o sistema.
                </AlertDialogDescription>
              </AlertDialogHeader>
                <div style={{ display: "flex", flexDirection: "column", alignItems: "center", gap: "10px" }}>
                  <input
                  placeholder="CEP"
                  onChange={(e) => setCep(e.target.value)}
                  onBlur={() => {handleCepChange()}}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />
                  <input
                  placeholder="País"
                  disabled={true}
                  value={pais}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />
                  <input
                  placeholder="Estado"
                  onChange={(e) => setEstado(e.target.value)}
                  value={estado}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />
                  <input
                  placeholder="Cidade"
                  value={cidade}
                  onChange={(e) => setCidade(e.target.value)}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />
                  <input
                  placeholder="Bairro"
                  value={bairro}
                  onChange={(e) => setBairro(e.target.value)}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />
                  <input
                  placeholder="Rua"
                  value={logradouro}
                  onChange={(e) => setLogradouro(e.target.value)}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />
                  <input
                  placeholder="Número"
                  value={numero}
                  onChange={(e) => setNumero(e.target.value)}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />
                  <input
                  placeholder="Complemento"
                  value={complemento}
                  onChange={(e) => setComplemento(e.target.value)}
                  style={{ padding: "10px", borderRadius: "5px", border: "1px solid hsl(var(--primary))", width: "100%" }}
                  />                  
                </div>
              <AlertDialogFooter>
                <AlertDialogCancel
                  className="bg-[hsl(var(--machine-card-bg))] text-[hsl(var(--text))]"
                  onClick={() => {navigate('/')}}>
                  Vou fazer isso depois
                </AlertDialogCancel>
                <AlertDialogAction onClick={() => {
                  handleSubtmitAddress();
                }}>
                  Cadastrar
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
      </main>
    </Layout>
  );
}
