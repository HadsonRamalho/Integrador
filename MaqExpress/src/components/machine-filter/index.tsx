import React from "react";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";

interface MachineFilterProps {
  machines: { categoria: string }[];
  filter: string;
  setFilter: (filter: string) => void;
}

const MachineFilter: React.FC<MachineFilterProps> = ({ machines, filter, setFilter }) => {
  return (
    <div className="flex justify-center mb-4">
      <Card className="bg-[hsl(var(--machine-card-bg))] h-[120px] border-[hsl(var(--primary))] w-[90%] md:w-[50%]">
        <CardHeader>
          <p>Filtrar Máquinas</p>
        </CardHeader>
        <CardContent>
          <CardDescription className="mb-4">
            <div className="flex justify-center">
              <select
                className="w-[60%] pl-2 bg-white h-[30px] text-black border-[hsl(var(--primary))] rounded-md border-[1px]"
                onChange={(e) => setFilter(e.target.value)}
                value={filter}
                required
              >
                <option value="">Todas as máquinas</option>
                {machines
                  .map((machine) => machine.categoria)
                  .filter(
                    (categoria, index, self) =>
                      self.indexOf(categoria) === index
                  )
                  .map((categoria) => (
                    <option key={categoria} value={categoria}>
                      {categoria}
                    </option>
                  ))}
              </select>
            </div>
          </CardDescription>
        </CardContent>
      </Card>
    </div>
  );
};

export default MachineFilter;