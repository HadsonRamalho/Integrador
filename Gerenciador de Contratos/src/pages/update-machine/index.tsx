/* eslint-disable @typescript-eslint/no-explicit-any */
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Skeleton } from "@/components/ui/skeleton";
import { Machine } from "@/interfaces/machine";
import Layout from "@/layouts/default";
import {
  loadMachineImage,
  loadMachinePublicId,
  updateMachine,
} from "@/services/api/machine/machine";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

export default function UpdateMachine() {
  const { publicid } = useParams();
  const [name, setName] = useState("");
  const [serialNumber, setSerialNumber] = useState("");
  const [rentValue, setRentValue] = useState(0);
  const [rentDisponibility, setRentDisponibility] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("");
  const [image, setImage] = useState("");
  const [loadingImage, setLoadingImage] = useState(true);
  const [loading, setIsLoading] = useState(false);
  const [error, setError] = useState(false);
  const [machine, setMachine] = useState<Machine>();

  async function tryUpdateMachine() {
    setIsLoading(true);
    if (!machine) {
      return;
    }
    try {
      console.log(machine.datacadastro);
      const machineData: Machine = {
        idmaquina: machine.idmaquina,
        nome: name,
        descricao: description,
        categoria: category,
        valoraluguel: rentValue,
        disponivelaluguel: rentDisponibility,
        idpublico: machine.idpublico,
        numeroserie: serialNumber,
        status: "Ativo",
        datacadastro: machine.datacadastro,
        dataatualizacao: machine.dataatualizacao,
      };
      await updateMachine(machineData);
      alert("Máquina atualizada!");
    } catch (error) {
      console.error(error);
    } finally {
      setIsLoading(false);
    }
  }

  useEffect(() => {
    async function loadMachine(id: string) {
      const machineData = await loadMachinePublicId(id);
      setMachine(machineData);
      setRentDisponibility(machineData.disponivelaluguel);
      setName(machineData.nome);
      setCategory(machineData.categoria);
      setDescription(machineData.descricao);
      setRentValue(machineData.valoraluguel);
      setSerialNumber(machineData.numeroserie);
    }
    if (publicid) {
      loadMachine(publicid);
    }
  }, [publicid]);

  const fetchMachineImage = async (machineId: string) => {
    try {
      console.log("machineId: ", machineId);
      const response = await loadMachineImage(machineId);
      const imageUrl = response;
      setImage(imageUrl);
      setLoadingImage(false);
    } catch (error) {
      console.error(error);
      setError(true);
    } finally {
      setLoadingImage(false);
    }
  };

  useEffect(() => {
    if (machine) {
      fetchMachineImage(machine.idmaquina);
    }
  }, [machine]);

  return (
    <Layout>
      <div className="container-maquinas">
        <Card className="form-maquinas border-[hsl(var(--primary))] mt-10 ml-2 mr-2 w-[98vw] max-w-3xl mb-10">
          <CardHeader>
            <h2 className="text-[25px] text-[hsl(var(--text))]">
              Atualização de Máquina
            </h2>
          </CardHeader>
          <CardContent className="form-content space-y-4">
            <CardDescription>
              <Label className="mb-1">Imagem Principal</Label>
              <div className="machine-image-home">
                {loadingImage ? (
                  <div>
                    <Skeleton className="h-[30vh] w-[30vw] rounded-xl" />
                  </div>
                ) : error ? (
                  <div className="image-placeholder">
                    Erro ao carregar imagem
                  </div>
                ) : image ? (
                  <img
                    src={image}
                    className="p-0 w-full"
                    alt={`Imagem de ${machine?.nome}`}
                  />
                ) : (
                  <div className="image-placeholder">Imagem indisponível</div>
                )}
              </div>

              <Label htmlFor="machine-name" className="mb-1">
                Nome da Máquina
              </Label>
              <Input
                id="machine-name"
                className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
                required
                placeholder="Nome da Máquina"
                onChange={(e) => setName(e.target.value)}
                value={name}
              />

              <Label htmlFor="serial-number" className="mb-1">
                Número de Série
              </Label>
              <Input
                id="serial-number"
                className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
                required
                placeholder="Número de Série"
                onChange={(e) => setSerialNumber(e.target.value)}
                value={serialNumber}
              />

              <Label htmlFor="rent-value" className="mb-1">
                Valor de Aluguel
              </Label>
              <Input
                id="rent-value"
                type="number"
                className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
                value={rentValue}
                onChange={(e) =>
                  setRentValue(e.target.value ? Number(e.target.value) : 0)
                }
                min="0.01"
                step="0.01"
                required
              />

              <Label htmlFor="rent-disponibility" className="mb-1">
                Disponível para Aluguel?
              </Label>
              <br></br>
              <select
                id="rent-disponibility"
                className="w-[455px] bg-[hsl(var(--background))] h-[30px] ml-10 text-[hsl(var(--text))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
                onChange={(e) => setRentDisponibility(e.target.value)}
                value={rentDisponibility}
                required
              >
                <option value="sim">Sim</option>
                <option value="não">Não</option>
              </select>
              <br></br>
              <Label htmlFor="description" className="mb-1">
                Descrição da Máquina
              </Label>
              <textarea
                id="description"
                className="w-full p-2 border bg-[hsl(var(--background))] rounded-md mb-4 ml-10 h-20 text-[hsl(var(--text))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
                placeholder="Descrição da Máquina"
                onChange={(e) => setDescription(e.target.value)}
                required
                value={description}
              />

              <Label htmlFor="category" className="mb-1">
                Categoria da Máquina
              </Label>
              <Input
                id="category"
                className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
                placeholder="Categoria da Máquina"
                onChange={(e) => setCategory(e.target.value)}
                value={category}
                required
              />

              <div className="button-group-maquinas flex gap-4 mt-6">
                <Button onClick={tryUpdateMachine} disabled={loading}>
                  {loading ? <>Atualizando...</> : <>Atualizar Máquina</>}
                </Button>
              </div>
            </CardDescription>
          </CardContent>
        </Card>
      </div>
    </Layout>
  );
}
