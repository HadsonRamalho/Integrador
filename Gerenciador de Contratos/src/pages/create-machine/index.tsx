/* eslint-disable @typescript-eslint/no-explicit-any */
import "@/components/login/login.css";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
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
    try{
      const response = await fetch(`https://g6v9psc0-3003.brs.devtunnels.ms/cadastra_maquina`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
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
      <Card className="quadro3">
        <CardHeader>
          <CardTitle>Cadastro de Máquina</CardTitle>
        </CardHeader>
        <CardContent>
          <CardDescription className="quadro3">
            <div>
              <input
              placeholder="Nome da Máquina"
              onChange={(e) => {setName(e.target.value)}}
              value={name}
              />
              <input
              placeholder="Número de Série"
              onChange={(e) => {setSerialNumber(e.target.value)}}
              value={serialNumber}
              />
              <input
               type="number"
               value={rentValue}
               onChange={(e) => setRentValue(e.target.value ? Number(e.target.value) : 0)}
               min="0.01"
               step="0.01"
               required
              />
              <input
              placeholder="Está disponível pra aluguel?"
              onChange={(e) => {setRentDisponibility(e.target.value)}}
              value={rentDisponibility}
              />
              <input
              placeholder="Status"
              onChange={(e) => {setStatus(e.target.value)}}
              value={status}
              />
              <textarea
              placeholder="Descrição da Máquina"
              onChange={(e) => {setDescription(e.target.value)}}
              value={description}
              />
              <input
              placeholder="Categoria da Máquina"
              onChange={(e) => {setCategory(e.target.value)}}
              value={category}
              />
              <label>Imagens da Máquina</label>
              {machineImages.map((image, index) => (
                <div key={index}>
                  <input
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
              <Button onClick={addImageInput}>Adicionar Imagem</Button>
              <Button onClick={tryCreateMachine}>Cadastrar Máquina</Button>
            </div>
          </CardDescription>
        </CardContent>
      </Card>
    </Layout>
  );
}