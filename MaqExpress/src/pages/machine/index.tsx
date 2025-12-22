import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import { listMachine } from "@/services/api/machine/machine";
import { Button } from "@/components/ui/button";
import { Machine as Maquina } from "@/interfaces/machine";
import "@/components/machine/machine.css";
import MachineFilter from "@/components/machine-filter";
import { MachineCard } from "@/components/machine-card";

export default function Machine() {
  const [machines, setMachines] = useState<Maquina[]>([]);
  const [filter, setFilter] = useState("");
  const [filteredMachines, setFilteredMachines] = useState<Maquina[]>([]);

  useEffect(() => {
    const listMachines = async () => {
      const machines = await listMachine();
      console.log(machines);
      setMachines(machines);
    };
    listMachines();
  }, []);

  useEffect(() => {
    setFilteredMachines(
      machines.filter((machine)=> 
        filter ? machine.categoria === filter : true
      )
    );
  }, [filter, machines]);

  return (
    <Layout>
      <main className="mt-10 mb-10">
      <MachineFilter machines={machines} filter={filter} setFilter={setFilter} />

      <div className={`machine-grid ${filteredMachines.length === 1 ? 'single-item' : ''}`} >
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
                  className="mt-6"
                  style={{paddingBottom: '10px', maxHeight: '800px', height: '650px'}}
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
