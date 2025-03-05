import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { listMachine } from "@/services/api/machine/machine";
import { Button } from "@/components/ui/button";
import { Machine as Maquina } from "@/interfaces/machine";
import { useNavigate } from "react-router-dom";
import "@/components/machine/machine.css";
import MachineFilter from "@/components/machine-filter";
import { MachineCard } from "@/components/machine-card";

export default function Machine() {
  const navigate = useNavigate();
  const [machines, setMachines] = useState<Maquina[]>([]);
  const [filter, setFilter] = useState("");

  useEffect(() => {
    const listMachines = async () => {
      const machines = await listMachine();
      console.log(machines);
      setMachines(machines);
    };
    listMachines();
  }, []);

  return (
    <Layout>
      <main className="mt-10">
      <MachineFilter machines={machines} filter={filter} setFilter={setFilter} />
        <div className="machine-grid">
          {machines.length === 0 ? (
            <div>
              <p>
                Houve um erro ao carregar as máquinas. Reporte o problema aqui:
              </p>
              <Button>Relatar problema</Button>
            </div>
          ) : (
            machines
              .filter((machine) =>
                filter ? machine.categoria === filter : true
              )
              .map((machine: Maquina) => (
                <div
                  key={machine.idmaquina}
                  style={{ marginTop: "4vh", width: "90%", height: "600px" }}
                >
                  <MachineCard machine={machine} />
                </div>
              ))
          )}
        </div>        
      </main>
    </Layout>
  );
}
