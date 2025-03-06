import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Machine } from "@/interfaces/machine";
import { Renter } from "@/interfaces/renter";
import { User } from "@/interfaces/user";
import Layout from "@/layouts/default";
import { loadMachinePublicId } from "@/services/api/machine/machine";
import { createRenter, loadRenterByUserId } from "@/services/api/renter/renter";
import { loadUserById } from "@/services/api/user/user";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import "../../components/rent-machine/rent-machine.css";
import { Address } from "@/interfaces/address";
import { loadAddressUserId } from "@/services/api/address/address";
import { Input } from "@/layouts";
import { Label } from "@radix-ui/react-dropdown-menu";

const RentMachine = () => {
  const { publicid } = useParams();
  const [machine, setMachine] = useState<Machine>();
  const [step, setStep] = useState("carregando");
  const [user, setUser] = useState<User>();
  const [renter, setRenter] = useState<Renter>();
  const [address, setAddress] = useState<Address>();
  const [renterId, setRenterId] = useState<string>();
  const [error, setError] = useState(false);

  const navigate = useNavigate();

  useEffect(() => {
    const listMachines = async () => {
      if (publicid) {
        const machine = await loadMachinePublicId(publicid);
        console.log(machine);
        setMachine(machine);
        if(machine.disponivelaluguel !== "Sim"){
          alert("Essa máquina não está disponível para aluguel no momento.");
          navigate("/");
        }
      }
    };
    listMachines();
  }, [publicid]);

  useEffect(() => {
    const loadUser = async () => {
      const id = localStorage.getItem("USER_ID");
      if(!id){
        setError(true);
        return;
       }
      try{
        const user = await loadUserById(id);
        console.log(user);
        setUser(user);
      } catch(err){
        setError(true);
        console.error(err);
        setStep("");
      }
    };
    loadUser();    
  }, []);
  
  useEffect(() => {
    if(machine && user && !renter){
      setStep("cadastro-locatario");
    }
  }, [machine, user, renter]);

  useEffect(() => {
    const loadRenter = async (id: string) => {
      try {
        const loadedRenter = await loadRenterByUserId(id);
        setStep("revisão-solicitação");
        setRenter(loadedRenter);
        console.log(loadedRenter);
      } catch (err) {
        console.error(err);
        setStep("cadastro-locatario");
      }
    };
    if (user) {
      loadRenter(user.idusuario);
    }
    if (renterId && user){
      loadRenter(user.idusuario);
    }
  }, [renterId, user]);

  useEffect(() => {
    const loadAddress = async (id: string) => {
      try {
        const loadedAddress = await loadAddressUserId(id);
        setAddress(loadedAddress);
      } catch (err) {
        console.error(err);
        setError(true);
        setStep("cadastro-locatario");
      }
    };
    if (user) {
      loadAddress(user.idusuario);
    }
  }, [user]);

  const tryCreateRenter = async () => {
    if (!user || !address){
      return;
    }
    try{
      const createdRenterId = await createRenter(user.idusuario, address.idendereco);
      setRenterId(createdRenterId);
      setStep("revisão-solicitação");
    } catch(error){
      console.error(error);
    }
  }

  if (error){
    return (
      <Layout>
        <main>
          <Card className="m-4 p-4">
            <p>Houve um erro ao carregar as informações. Reporte o problema aqui:</p>
            <Button>Relatar problema</Button>
          </Card>
        </main>
      </Layout>
    )
  }

  return (
    <Layout>
      <main>
        {step === "cadastro-locatario" ? (
          <div className="rent-machine-container">
            <Card className="bg-[hsl(var(--machine-card-bg))] w-[60vw] m-4 border-[hsl(var(--primary))]">
              <CardHeader>
              <CardTitle className="text-[1.5rem] text-[hsl(var(--primary))]">Minhas Informações</CardTitle>
              </CardHeader>
              <CardDescription>
                <p>Confirme seus dados antes de alugar a máquina. Só é necessário realizar esse processo uma vez.</p>
                <p>Se precisar atualizar alguma informação, acesse o seu perfil.</p>
              </CardDescription>
                <CardContent className="flex flex-col items-center w-full">
                <Card className="w-[60%] mt-2 bg-[hsl(var(--machine-card-bg))] pb-4 border-[hsl(var(--primary))]">
                  <Label className="text-[hsl(var(--text))] mt-2">Nome</Label>
                  <Input
                  value={user?.nome}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))] mt-2">E-mail</Label>
                  <Input
                  value={user?.email}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))] mt-2">Documento</Label>
                  <Input
                  value={user?.documento}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>
                </Card>
                {address ? (
                  <Card className="mt-2 w-[60%] bg-[hsl(var(--machine-card-bg))] pb-10 border-[hsl(var(--primary))]">
                  <Label className="text-[hsl(var(--text))] mt-2">CEP</Label>
                  <Input
                  value={address?.cep}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  
                  <Label className="text-[hsl(var(--text))] mt-2">País</Label>
                  <Input
                  value={address?.pais}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))]  mt-2">Estado</Label>
                  <Input
                  value={address?.estado}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))]  mt-2">Cidade</Label>
                  <Input
                  value={address?.cidade}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))]  mt-2">Bairro</Label>
                  <Input
                  value={address?.bairro}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))]  mt-2">Rua</Label>
                  <Input
                  value={address?.logradouro}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))]  mt-2">Número</Label>
                  <Input
                  value={address?.numero}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>

                  <Label className="text-[hsl(var(--text))]  mt-2">Complemento</Label>
                  <Input
                  value={address?.complemento}
                  readOnly={true}
                  className="p-2 text-[hsl(var(--text))] bg-[hsl(var(--machine-card-bg))] rounded-md border-[1px] border-[hsl(var(--primary))] w-[50%]"/>
                </Card>
                ) : (
                  <Card className="w-[60%] mt-2 border-[hsl(var(--primary))] bg-[hsl(var(--machine-card-bg))]">
                    <CardContent className="pt-2 pb-2">
                      <p>Atualize seu endereço para prosseguir.</p>
                    </CardContent>
                    <CardContent>
                      <Button onClick={() => {navigate('/user-profile')}}>Atualizar endereço</Button>
                    </CardContent>
                  </Card>
                )}
                </CardContent>
                <CardFooter className="flex justify-center">
                {
                  (address && user) && 
                  (<Button onClick={tryCreateRenter}>Confirmar informações</Button>)
                }
                </CardFooter>
            </Card>            
          </div>
        ) : step === "revisão-solicitação" ? (
          <div>
            <h1>Revisão da Solicitação</h1>
          </div>
        ) : step === "carregando" ? 
        (<div>
          <h1>Carregando...</h1>
        </div>) 
        : step === "informações-contrato" ? (
          <div>
            <h1>Informações do Contrato</h1>
          </div>
        ) 
        : step === "status-solicitação" ? (
          <div>
            <h1>Status da Solicitação</h1>
          </div>
        ) : 
        <div>
          <p>
            Houve um erro ao carregar as informações. Reporte o problema aqui:
          </p>
          <Button>Relatar problema</Button>
        </div>}
      </main>
    </Layout>
  );
};

export default RentMachine;
