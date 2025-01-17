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
  const [status, setStatus] = useState("");
  const [description, setDescription] = useState("");
  const [category, setCategory] = useState("");
  const [machineImages, setMachineImages] = useState<(File | null)[]>([]);
  const imageIds: string[] = [];

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
          status,
          descricao: description,
          categoria: category}),
      });

      if (!response.ok) {
        throw new Error(await response.text() || "Erro ao cadastrar imagem");
      }
      
      const res = await response.json();
      return res.idmaquina;
    } catch(err){
      console.error(err);
    }
  }

  const tryCreateMachine = async () => {
    const machineid = await createMachine();
    await submitImages();
    await connectMachineImage(machineid);
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
      <h2 style={{color: 'hsl(var(--text))', fontSize: "25px"}}>Cadastro de Máquina</h2>
      <div className="container-maquinas">
      <Card className="form-maquinas">
        <CardHeader>
        </CardHeader>
        <CardContent className="form-content">
          <CardDescription>
            <Label htmlFor="machine-name">Nome da Máquina</Label>
            <Input
              id="machine-name"
              placeholder="Nome da Máquina"
              onChange={(e) => setName(e.target.value)}
              value={name}
            />
          </CardDescription>
          <CardDescription>
            <Label htmlFor="serial-number">Número de Série</Label>
            <Input
              id="serial-number"
              placeholder="Número de Série"
              onChange={(e) => setSerialNumber(e.target.value)}
              value={serialNumber}
            />
          </CardDescription>
          <CardDescription>
            <Label htmlFor="rent-value">Valor de Aluguel</Label>
            <Input
              id="rent-value"
              type="number"
              value={rentValue}
              onChange={(e) => setRentValue(e.target.value ? Number(e.target.value) : 0)}
              min="0.01"
              step="0.01"
              required
            />
          </CardDescription>
          <CardDescription>
            <Label htmlFor="rent-disponibility">Disponível para Aluguel?</Label>
            <select
              id="rent-disponibility"
              onChange={(e) => setRentDisponibility(e.target.value)}
              value={rentDisponibility}
            >
              <option value="sim">Sim</option>
              <option value="não">Não</option>
            </select>
          </CardDescription>
          <CardDescription>
            <Label htmlFor="status">Status</Label>
            <Input
              id="status"
              placeholder="Status"
              onChange={(e) => setStatus(e.target.value)}
              value={status}
            />
          </CardDescription>
          <CardDescription>
            <Label htmlFor="description">Descrição da Máquina</Label>
            <textarea
              id="description"
              placeholder="Descrição da Máquina"
              onChange={(e) => setDescription(e.target.value)}
              value={description}
            />
          </CardDescription>
          <CardDescription>
            <Label htmlFor="category">Categoria da Máquina</Label>
            <Input
              id="category"
              placeholder="Categoria da Máquina"
              onChange={(e) => setCategory(e.target.value)}
              value={category}
            />
          </CardDescription>
          <CardDescription>
            <Label>Imagens da Máquina</Label>
            {machineImages.map((image, index) => (
              <div key={index}>
                <Input
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
          </CardDescription>
          <CardDescription className="button-group-maquinas">
            <Button onClick={addImageInput}>Adicionar Imagem</Button>
            <Button onClick={tryCreateMachine}>Cadastrar Máquina</Button>
          </CardDescription>
        </CardContent>
      </Card>
      </div>
    </Layout>
  );
  
}