/* eslint-disable @typescript-eslint/no-explicit-any */
import "@/components/create-machine/index.css";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import Layout from "@/layouts/default";
import { useState } from "react";

export default function CreateMachine() {
  const [name, setName] = useState("");
  const [serialNumber, setSerialNumber] = useState("");
  const [rentValue, setRentValue] = useState(0);
  const [rentDisponibility, setRentDisponibility] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("");
  const [machineImages, setMachineImages] = useState<(File | null)[]>([]);
  const imageIds: string[] = [];
  const [loading, setIsLoading] = useState(false);

  const handleImageChange = (index: number, file: File) => {
    const updatedImages = [...machineImages];
    updatedImages[index] = file;
    setMachineImages(updatedImages);
  };

  const addImageInput = () => {
    setMachineImages((prev) => [...prev, null]);
  };

  const submitImages = async () => {
    if (machineImages.length === 0 || machineImages.every((file) => !file)) {
      alert("Por favor, selecione pelo menos uma imagem.");
      return;
    }

    for (const [, file] of machineImages.entries()) {
      if (!file) continue;

      const singleFormData = new FormData();
      singleFormData.append("file", file);

      try {
        const response = await fetch(`https://g6v9psc0-3003.brs.devtunnels.ms/cadastra_imagem`, {
          method: "POST",
          body: singleFormData,
        });

        if (!response.ok) {
          throw new Error(await response.text() || "Erro ao cadastrar imagem");
        }

        const data = await response.json();
        
        if (!data || !data.idimagem) {
          throw new Error("Erro ao obter o ID de uma imagem");
        }
        
        imageIds.push(data.idimagem);  
        return imageIds[0];
      } catch (err) {
        console.error("Erro na requisição de imagem:", err);
        alert("Erro ao enviar imagem.");
      }
    }
    alert("Imagens enviadas com sucesso!");
    setMachineImages([]);
  };

  const createMachine = async () => {
    const idusuario = localStorage.getItem("USER_ID");
    try{
      const response = await fetch(`https://g6v9psc0-3003.brs.devtunnels.ms/cadastra_maquina`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          idusuario,

          nome: name,
          numeroserie: serialNumber, 
          valoraluguel: rentValue,
          disponivelaluguel: rentDisponibility,
          status: "Ativo",
          descricao: description,
          categoria: category}),
      });

      if (!response.ok) {
        throw new Error(await response.text() || "Erro ao cadastrar máquina");
      }
      
      const res = await response.json();
      return res.idmaquina;
    } catch(err){
      console.error(err);
    }
  }

  const tryCreateMachine = async () => {
    setIsLoading(true);
    if(machineImages.length < 1){
      alert("A máquina deve possuir ao menos uma imagem.");
      setIsLoading(false);
      return;
    }
    if(rentValue < 1){
      alert("O valor do aluguel não pode ser menor que R$ 1,00");
      setIsLoading(false);
      return;
    }
    const machineid = await createMachine();
    await submitImages();
    await connectMachineImage(machineid);
    setIsLoading(false);
  }

  const connectMachineImage = async (idmaquina: any) => {
    try {
      for (const idimagem of imageIds) {
        const response = await fetch(`https://g6v9psc0-3003.brs.devtunnels.ms/cadastra_imagem_maquina`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({idimagem, idmaquina}),
        });
  
        if (!response.ok) {
          console.error("Erro ao vincular imagem à máquina. Status:", response.status);
          throw new Error("Erro ao vincular a imagem à máquina: " + await response.text());  
        } 
        const data = await response.json();
        console.log("Imagem vinculada à máquina:", data);              
      }
    } catch (err) {
      console.error("Erro ao vincular imagens:", err);
    }
  }

  return (
  <Layout>
    <div className="container-maquinas">
      <Card className="form-maquinas border-[hsl(var(--primary))] mt-10 ml-2 mr-2 w-[98vw] max-w-3xl mb-10">
        <CardHeader>
          <h2 className="text-[25px] text-[hsl(var(--text))]">Cadastro de Máquina</h2>
        </CardHeader>
        <CardContent className="form-content space-y-4">
          <CardDescription>
            <Label htmlFor="machine-name" className="mb-1">Nome da Máquina</Label>
            <Input
              id="machine-name"
              className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
              required
              placeholder="Nome da Máquina"
              onChange={(e) => setName(e.target.value)}
              value={name}
            />
  
            <Label htmlFor="serial-number" className="mb-1">Número de Série</Label>
            <Input
              id="serial-number"
              className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"              
              required
              placeholder="Número de Série"
              onChange={(e) => setSerialNumber(e.target.value)}
              value={serialNumber}
            />
  
            <Label htmlFor="rent-value" className="mb-1">Valor de Aluguel</Label>
            <Input
              id="rent-value"
              type="number"
              className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"              
              value={rentValue}
              onChange={(e) => setRentValue(e.target.value ? Number(e.target.value) : 0)}
              min="0.01"
              step="0.01"
              required
            />
  
            <Label htmlFor="rent-disponibility" className="mb-1">Disponível para Aluguel?</Label>
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
            <Label htmlFor="description" className="mb-1">Descrição da Máquina</Label>
            <textarea
              id="description"
              className="w-full p-2 border bg-[hsl(var(--background))] rounded-md mb-4 ml-10 h-20 text-[hsl(var(--text))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"
              placeholder="Descrição da Máquina"
              onChange={(e) => setDescription(e.target.value)}
              required
              value={description}
            />
  
            <Label htmlFor="category" className="mb-1">Categoria da Máquina</Label>
            <Input
              id="category"
              className="text-[hsl(var(--text))] bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px]"              placeholder="Categoria da Máquina"
              onChange={(e) => setCategory(e.target.value)}
              value={category}
              required
            />
  
            <Label className="mb-1">Imagens da Máquina</Label>
            <div className="space-y-2">
              {machineImages.map((image, index) => (
                <div key={index} className="mb-2">
                  <Input
                    className="mb-4 border-[hsl(var(--primary))] bg-[hsl(var(--background))] rounded-m border-[1px]"

                    type="file"
                    accept="image/*"
                    aria-label={`Upload da imagem ${index + 1}`}
                    onChange={(e) => {
                      const files = e.target.files;
                      if (files && files[0]) {
                        handleImageChange(index, files[0]);
                      }
                    }}
                  />
                  {image && <p>Imagem {index + 1} selecionada.</p>}
                </div>
              ))}
            </div>
  
            <div className="button-group-maquinas flex gap-4 mt-6">
              <Button onClick={addImageInput}>Adicionar Imagem</Button>
              <Button disabled={loading} onClick={tryCreateMachine}>
                {loading ? (
                  <>Cadastrando...</>
                ) : (<>Cadastrar Máquina</>)} 
                </Button>
            </div>
          </CardDescription>
        </CardContent>
      </Card>
    </div>
    </Layout>
  );
  
  
}