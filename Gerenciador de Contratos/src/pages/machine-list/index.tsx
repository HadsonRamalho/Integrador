import { Machine } from "@/interfaces/machine";
import Layout from "@/layouts/default";
import {
  loadUserMachines,
} from "@/services/api/machine/machine";
import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardHeader,
} from "@/components/ui/card";
import "@/components/machine-list/machine-list.css";
import { Machine as Maquina } from "@/interfaces/machine";
import { Button } from "@/components/ui/button";
import MachineFilter from "@/components/machine-filter";
import { MachineCard } from "@/components/machine-card";
import { useNavigate } from "react-router-dom";

export const MachineList = () => {
  const [machines, setMachines] = useState<Machine[]>([]);
  const [filter, setFilter] = useState("");

  const navigate = useNavigate();

  useEffect(() => {
    const id = localStorage.getItem("USER_ID");
    const loadMachines = async (id: string) => {
      const machineArray = await loadUserMachines(id);
      console.log(machineArray || "Nenhuma máquina");
      setMachines(machineArray);
    };
    console.log(machines, id);
    if (machines.length === 0 && id) {
      loadMachines(id);
    }
  }, [machines]);

  const filteredMachines = machines.filter((machine) =>
    filter ? machine.categoria === filter : true
  );

  return (
    <Layout>
      <main className="mt-10 mb-10">
        <div className="flex justify-center items-center ">
          <div className="rounded-md w-[50vw]">
            <MachineFilter
              machines={machines}
              filter={filter}
              setFilter={setFilter}
            />
          </div>
        </div>
        <Card className="bg-[hsl(var(--machine-card-bg))] m-4 border-[hsl(var(--primary))]">
            <CardHeader className="text-center">
            <p className="text-[1.2rem]">Minhas Máquinas</p>
            </CardHeader>
          <CardContent>
          <div className={`machine-list-grid `}>
          {filteredMachines.length === 0 ? (
            <Card>
              <CardHeader>
                <h2 className="text-[hsl(var(--primary))]">
                  Erro ao carregar lista de máquinas
                </h2>
              </CardHeader>
              <CardContent>
                <div>
                  <p className="mb-2 text-[hsl(var(--primary))]">
                    Você ainda não cadastrou uma máquina.
                    <br />
                  </p>
                  <div className="m-4">
                    <Button className="m-2">Relatar problema</Button>
                    <Button className="m-2" onClick={() => (navigate("/create-machine"))}>Cadastrar uma máquina</Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          ) : (
            filteredMachines.map((machine: Maquina) => (
              <div
                key={machine.idmaquina}
                className="h-full mb-4 md:h-[600px]"
                style={{ width: "90%", maxWidth: '350px', maxHeight: '500px', padding: '0' }}
              >
                <MachineCard machine={machine} />
              </div>
            ))
          )}
          </div>
          </CardContent>
        </Card>
      </main>
    </Layout>
  );
};
