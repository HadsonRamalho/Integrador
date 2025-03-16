/* eslint-disable @typescript-eslint/no-explicit-any */
import "@/components/create-machine/index.css";
import EquipmentSelect from "@/components/equipment-select";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { BankAccount } from "@/interfaces/bank-account";
import Layout from "@/layouts/default";
import { createBankAccount, loadBankAccountByUserId } from "@/services/api/bank-account";
import { useEffect, useState } from "react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { loadUserById } from "@/services/api/user/user";
import { User } from "@/interfaces/user";
import { useNavigate } from "react-router-dom";
import { createImage, createMachineImage } from "@/services/api/image/image";
import { MachineInput } from "@/interfaces/machine";
import { createMachine } from "@/services/api/machine/machine";
import { MachineImage } from "@/interfaces/image";

export default function CreateMachine() {
  const [name, setName] = useState("");
  const [serialNumber, setSerialNumber] = useState("");
  const [rentValue, setRentValue] = useState(0);
  const [rentDisponibility, setRentDisponibility] = useState("Sim");
  const [description, setDescription] = useState("");
  const [machineImages, setMachineImages] = useState<(File | null)[]>([]);
  const imageIds: string[] = [];
  const [loading, setIsLoading] = useState(false);
  const [selectedEquipment, setSelectedEquipment] = useState("");

  const navigate = useNavigate();
  
  const [user, setUser] = useState<User>();
  const [bankAccount, setBankAccount] = useState<BankAccount>();

  const [bankName, setBankName] = useState<string>();
  const [bankAccountNumber, setBankAccountNumber] = useState<string>();
  const [bankAgency, setBankAgency] = useState<string>();
  const [accountId, setAccountId] = useState<string>();
  const [cadastrando, setCadastrando] = useState(false);

  useEffect(() => {
    const loadUser = async () => {
      const id = localStorage.getItem("USER_ID");
      if(!id){
        return;
      }
      try{
        const user = await loadUserById(id);
        setUser(user);
      } catch(err){
        console.error(err);
      }
    };
    loadUser();    
  }, []);

  const loadBankAccount = async () => {
    if(!user){
      return;
    }
    try{
      const res = await loadBankAccountByUserId(user.idusuario);
      setBankAccount(res);
    } catch(error){
      console.error(error);
    }
  }

  const handleSubmitBankAccount = async () => {
    if(!user){
      console.warn("Usuário não está logado");
      return;
    }
    if(!bankAccountNumber || !bankAgency || !bankName){
      alert("Preencha todos os campos.");
      return;
    }
    try{
      setCadastrando(true);
      const account = await createBankAccount(user.idusuario, bankAccountNumber, bankAgency, bankName);
      setAccountId(account);
      console.log("Conta criada!");
      setCadastrando(false);
    } catch(error){
      console.log(error);
      setCadastrando(false);
    }
  }
 
  useEffect(() => {
    if(user){
      loadBankAccount();
    }
  }, [user, accountId]);

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
        const res = await createImage(singleFormData);
        imageIds.push(res.idimagem);   
      } catch (err) {
        console.error("Erro na requisição de imagem:", err);
        //alert("Erro ao enviar imagem.");
      }
    }
    //alert("Imagens enviadas com sucesso!");
    setMachineImages([]);
    return imageIds;
  };

  const newMachine = async () => {
    const idusuario = localStorage.getItem("USER_ID");
    if (!idusuario){
      alert("Você precisa estar logado para cadastrar uma máquina.");
      return;
    }
    try{
      const maquina: MachineInput = {
        idusuario,
        nome: name,
        numeroserie: serialNumber, 
        valoraluguel: rentValue,
        disponivelaluguel: rentDisponibility,
        status: "Ativo",
        descricao: description,
        categoria: selectedEquipment};
      
      const res = await createMachine(maquina);
      
      return res;
    } catch(err){
      console.error(err);
    }
  }

  const tryCreateMachine = async () => {
    try{
      setIsLoading(true);
    if (!name || !serialNumber || !rentValue || !description || !selectedEquipment){
      alert("Preencha todos os campos");
      setIsLoading(false);
      return;
    }
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
    const machineid = await newMachine();
    await submitImages();
    await connectMachineImage(machineid?.idmaquina);
    setIsLoading(false);
    } catch(error){
      alert("Erro ao cadastrar a máquina.");
      setIsLoading(false);
    }
  }

  const connectMachineImage = async (idmaquina: any) => {
    try {
      for (const idimagem of imageIds) {
        const data: MachineImage = {
          idimagem,
          idmaquina: idmaquina
        };
        const res = await createMachineImage(data);
        console.log("Imagem vinculada à máquina:", res);              
      }
    } catch (err) {
      console.error("Erro ao vincular imagens:", err);
    }
  }

  return (
  <Layout>
    <div className="flex justify-center items-center min-h-screen">
      <Card className="form-maquinas border-[hsl(var(--primary))] mt-10 w-full max-w-3xl mb-10">
        <CardHeader>
          <h2 className="text-[25px] text-[hsl(var(--text))]">Cadastro de Máquina</h2>
        </CardHeader>
        <CardContent className="form-content space-y-4">
          <CardDescription>
            <Label htmlFor="machine-name" className="mb-1">Nome da Máquina</Label>
            <Input
              id="machine-name"
              className="text-black   mb-4 border-[hsl(var(--primary))] rounded-m border-[1px] bg-neutral-100"
              required
              placeholder="Nome da Máquina"
              onChange={(e) => setName(e.target.value)}
              value={name}
            />
  
            <Label htmlFor="serial-number" className="mb-1">Número de Série</Label>
            <Input
              id="serial-number"
              className="text-black mb-4 border-[hsl(var(--primary))] rounded-m border-[1px] bg-neutral-100 "              
              required
              placeholder="Número de Série"
              onChange={(e) => setSerialNumber(e.target.value)}
              value={serialNumber}
            />
  
            <Label htmlFor="rent-value" className="mb-1">Valor de Aluguel</Label>
            <Input
              id="rent-value"
              type="number"
              className="text-black bg-[hsl(var(--background))] mb-4 border-[hsl(var(--primary))] rounded-m border-[1px] bg-neutral-100"              
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
              className="w-full  h-[30px] text-black mb-4 border-[hsl(var(--primary))] rounded-m border-[1px] bg-neutral-100"
              onChange={(e) => setRentDisponibility(e.target.value)}
              value={rentDisponibility}
              required
            >
              <option value="Sim">Sim</option>
              <option value="Não">Não</option>
            </select>
            <br></br>
            <Label htmlFor="description" className="mb-1">Descrição da Máquina</Label>
            <textarea
              id="description"
              className="w-full p-2 border  rounded-md mb-4 h-20 text-black mb-4 border-[hsl(var(--primary))] rounded-m border-[1px] bg-neutral-100"
              placeholder="Descrição da Máquina"
              onChange={(e) => setDescription(e.target.value)}
              required
              value={description}
            />
  
            <Label htmlFor="category" className="mb-1">Categoria da Máquina</Label>
            <EquipmentSelect className="text-black mb-4 border-[hsl(var(--primary))] rounded-m border-[1px] w-full bg-neutral-100" onChange={(e) => setSelectedEquipment(e.target.value)} />
  
            <Label className="mb-1">Imagens da Máquina</Label>
            <div className="space-y-2">
              {machineImages.map((image, index) => (
                <div key={index} className="mb-2">
                  <Input
                    className="mb-4 border-[hsl(var(--primary))]  rounded-m border-[1px] bg-neutral-100"

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
      <div>
        <AlertDialog open={!bankAccount}>
          <AlertDialogTrigger asChild></AlertDialogTrigger>
          <AlertDialogContent className="border-[hsl(var(--primary))]">
            <AlertDialogHeader>
            <AlertDialogTitle>
              Cadastre uma conta bancária
            </AlertDialogTitle>
            <AlertDialogDescription>
              <p>É necessário adicionar uma conta bancária antes de cadastrar uma máquina.</p>
              <p>Se precisar atualizar alguma informação, acesse o seu perfil.</p>
            </AlertDialogDescription>
            </AlertDialogHeader>
              <div style={{ alignItems: "center", gap: "10px" }}>
                <Label className="text-[hsl(var(--text))] mb-2">Nome do Titular</Label>
                <Input className="border-[hsl(var(--primary))] text-[hsl(var(--text))]"
                  value={user?.nome}
                  disabled={true}/>
                <Label className="text-[hsl(var(--text))] mb-2">Nome do Banco</Label>
                <Input className="border-[hsl(var(--primary))] text-[hsl(var(--text))]"
                onChange={(e) => setBankName(e.target.value)}
                value={bankName}/>
                <Label className="text-[hsl(var(--text))] mb-2">Número da Conta</Label>
                <Input className="border-[hsl(var(--primary))] text-[hsl(var(--text))]"
                onChange={(e) => setBankAccountNumber(e.target.value)}
                value={bankAccountNumber}/>
                <Label className="text-[hsl(var(--text))] mb-2">Número da Agência</Label>
                <Input className="border-[hsl(var(--primary))] text-[hsl(var(--text))]"
                onChange={(e) => setBankAgency(e.target.value)}
                value={bankAgency}/>
              </div>
            <AlertDialogFooter>
              <AlertDialogCancel 
              className="bg-[#882727] text-[hsl(var(--text))]"
              onClick={() => {navigate('/')}}
              > Farei isso depois
              </AlertDialogCancel>
              <AlertDialogAction
              onClick={handleSubmitBankAccount}
              disabled={cadastrando}>
                {cadastrando ?
                ("Cadastrando...")
                : ("Cadastrar")}
              </AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      </div>
    </div>
    </Layout>
  );
  
  
}