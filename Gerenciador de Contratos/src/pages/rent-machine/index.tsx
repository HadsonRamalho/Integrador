import { Button } from "@/components/ui/button";
import { Machine } from "@/interfaces/machine";
import { User } from "@/interfaces/user";
import Layout from "@/layouts/default";
import { loadMachinePublicId } from "@/services/api/machine/machine";
import { loadUserById } from "@/services/api/user/user";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

const RentMachine = () => {
  const { publicid } = useParams();
  const [machine, setMachine] = useState<Machine>();
  const [step, setStep] = useState("carregando");
  const [user, setUser] = useState<User>();
  const [error, setError] = useState(false);

  useEffect(() => {
    const listMachines = async () => {
      if (publicid) {
        const machine = await loadMachinePublicId(publicid);
        console.log(machine);
        setMachine(machine);
      }
    };
    listMachines();
  }, [publicid]);

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
        setStep("");
      }
    };
    loadUser();    
  }, []);
  
  useEffect(() => {
    if(machine && user){
      setStep("cadastro-locatario");
    }
  }, [machine, user]);

  return (
    <Layout>
      <main>
        {step === "cadastro-locatario" ? (
          <div>
            <h1>Cadastro de Locatário</h1>
          </div>
        ) : step === "informações-contrato" ? (
          <div>
            <h1>Informações do Contrato</h1>
          </div>
        ) : step === "revisão-solicitação" ? (
          <div>
            <h1>Revisão da Solicitação</h1>
          </div>
        ) : step === "carregando" ? 
        (<div>
          <h1>Carregando...</h1>
        </div>) 
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
