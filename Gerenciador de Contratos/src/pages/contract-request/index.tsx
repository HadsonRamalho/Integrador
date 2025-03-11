import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/layouts";
import { Search } from "lucide-react";
import { useEffect, useState } from "react";
import "@/components/contract-dropdown-menu/index.css";
import "@/components/helpcenter/helpcenter.css"
import Layout from "@/layouts/default";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Label } from "@/components/ui/label";
import { ContractRequest as SolicitacaoContrato } from "@/interfaces/contract-request";
import { loadContractRequestById, loadContractRequestsByOwnerId, updateContractRequestStatus } from "@/services/api/contract-request";
import { formatDate } from "@/services/api/format/format";
import { loadUserById } from "@/services/api/user/user";
import { User } from "@/interfaces/user";
import { loadMachineById } from "@/services/api/machine/machine";
import { Machine } from "@/interfaces/machine";
import { loadPdfByRequestId } from "@/services/api/contract";
import { ContractPDF } from "@/interfaces/contract";
import { pdf } from "@react-pdf/renderer";
import { PdfDocument } from "../pdf-example";

export default function ContractRequest() {

  const [requests, setRequests] = useState<SolicitacaoContrato[]>();
  const [updated, setUpdated] = useState(true);

  const loadRequests = async (id: string) => {
    try{
      const res = await loadContractRequestsByOwnerId(id);
      setRequests(res);
      setUpdated(false);
    }catch(error){
      console.error(error);
    }
  }

  useEffect(() => {
    const id = localStorage.getItem("USER_ID");
    if (id && updated){
      loadRequests(id);
    }
  }, [updated]);

  const RequestCard = ({ request }: { request: SolicitacaoContrato }) => {
    const [requestOrigin, setRequestOrigin] = useState<User>();
    const [requestMachine, setRequestMachine] = useState<Machine>();

    const [loadingPdf, setLoadingPdf] = useState(false);

    const tempoAluguel = request.prazolocacao + " " + request.medidatempolocacao;

    const [pdfData, setPdfData] = useState<ContractPDF>();

    const handleLoadPdf = async () => {
      setLoadingPdf(true);
      try {
      const data = await loadPdfByRequestId(request.idsolicitacao);
      setPdfData(data);
      const blob = await pdf(<PdfDocument contract={data} />).toBlob();
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = "documento.pdf";
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
      } catch (error) {
      console.error(error);
      } finally {
      setLoadingPdf(false);
      }
    }

    const loadRequestOrigin = async () => {
      try{
        const res = await loadUserById(request.origemsolicitacao);
        setRequestOrigin(res);
      }catch(error){
        console.error(error);
      }
    }

    const loadRequestMachine = async () => {
      try{
        const res = await loadMachineById(request.idmaquina);
        setRequestMachine(res);
      }catch(error){
        console.error(error);
      }
    }
  
    useEffect(() => {
        loadRequestOrigin();
        loadRequestMachine();
    }, []);
    
    const handleUpdateRequest = async (status: string) => {
      if(!status){
        return;
      }
      try{
        await updateContractRequestStatus(request.idsolicitacao, status);
        setUpdated(true);
      }catch(error){
        console.error(error);
      }
    }

    return (
    <Card className="border-[hsl(var(--primary))] bg-[hsl(var(--hover))] mb-2">
      <CardContent className="grid grid-cols-4 gap-4 mt-4">
        <Label className="mt-2 mb-2">Nome do Solicitante</Label>
        <Input
          value={requestOrigin?.nome}
          disabled={true}
          className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
        
        <Label className="mt-2 mb-2">Nome da Máquina</Label>
        <Input
          value={requestMachine?.nome}
          disabled={true}
          className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

        <Label className="mt-2 mb-2">Número de Série da Máquina</Label>
        <Input
          value={requestMachine?.numeroserie}
          disabled={true}
          className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

        <Label className="mt-2 mb-2">Tempo de Aluguel</Label>
        <Input
          value={tempoAluguel}
          disabled={true}
          className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

        <Label className="mt-2 mb-2">Status da Solicitação</Label>
        <Input
          value={request?.statussolicitacao}
          disabled={true}
          className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

        <Label className="mt-2 mb-2">Data da Solicitação</Label>
        <Input
          value={formatDate(request?.datasolicitacao)}
          disabled={true}
          className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
      </CardContent>
      {request.statussolicitacao === "Aguardando aprovação" && (
        <CardFooter className="flex justify-center gap-4">
          <Button className="bg-[#882727]"
          onClick={() => {handleUpdateRequest("Solicitação recusada")}}>Recusar Aluguel</Button>
          <Button
          onClick={() => {handleUpdateRequest("Solicitação aprovada")}}>Aprovar Aluguel</Button>
        </CardFooter>
      )}
      {request.statussolicitacao === "Solicitação aprovada" && (
        <CardFooter className="flex justify-center gap-4">
          <Button onClick={handleLoadPdf} disabled={loadingPdf}>
            {loadingPdf ? ("Carregando...") : ("Ver contrato")}</Button>
        </CardFooter>
      )}
    </Card>
    )
  }

  return(
    <Layout>
      <main className="contract-dropdown-container">
      <Card className="w-[60%] mt-2 bg-[hsl(var(--machine-card-bg))] pb-4 border-[hsl(var(--primary))] mb-10">
      <CardHeader className="text-[hsl(var(--text))] text-[1.25rem]">
        <strong>Solicitações de Contrato</strong>
      </CardHeader>
        <CardContent className="flex flex-col items-center justify-center h-full text-center p-5">
        <Tabs defaultValue="recebidas" className="w-full">
            <TabsList className="bg-[hsl(var(--background))]">
            <TabsTrigger 
              className="text-[hsl(var(--primary))] data-[state=active]:bg-[hsl(var(--primary))] data-[state=active]:text-[hsl(var(--text))] data-[state=active]:border-2 data-[state=active]:border-[hsl(var(--primary))]" 
              value="recebidas"
            >
              Solicitações recebidas
            </TabsTrigger>
            <TabsTrigger 
              className="text-[hsl(var(--primary))] data-[state=active]:bg-[hsl(var(--primary))] data-[state=active]:text-[hsl(var(--text))] data-[state=active]:border-2 data-[state=active]:border-[hsl(var(--primary))]" 
              value="emitidas"
            >
              Solicitações emitidas
            </TabsTrigger>
            </TabsList>
          <TabsContent value="recebidas">

          {requests?.map((req) => (
            <RequestCard request={req}/>
          ))}

          </TabsContent>
          <TabsContent value="emitidas">Change your password here.</TabsContent>
        </Tabs>
        </CardContent>
        </Card>
      </main>
    </Layout>
  );
}