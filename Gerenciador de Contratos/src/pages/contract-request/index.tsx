import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader } from "@/components/ui/card";
import { Input } from "@/layouts";
import { useEffect, useState } from "react";
import "@/components/contract-dropdown-menu/index.css";
import "@/components/helpcenter/helpcenter.css"
import Layout from "@/layouts/default";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Label } from "@/components/ui/label";
import { ContractRequest as SolicitacaoContrato } from "@/interfaces/contract-request";
import { loadContractRequestsByOwnerId, loadContractRequestsByRenterId, updateContractRequestStatus } from "@/services/api/contract-request";
import { formatDate } from "@/services/api/format/format";
import { loadUserById } from "@/services/api/user/user";
import { User } from "@/interfaces/user";
import { loadMachineById } from "@/services/api/machine/machine";
import { Machine } from "@/interfaces/machine";
import { loadContractByRequestId, loadPdfByRequestId } from "@/services/api/contract";
import { pdf } from "@react-pdf/renderer";
import { PdfDocument } from "../pdf-example";
import { Contract } from "@/interfaces/contract";
import { loadAddressUserId } from "@/services/api/address/address";
import { Address } from "@/interfaces/address";

export default function ContractRequest() {

  const [requests, setRequests] = useState<SolicitacaoContrato[]>();
  const [renterRequests, setRenterRequests] = useState<SolicitacaoContrato[]>();
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

  const loadRenterRequests = async (id: string) => {
    try{
      const res = await loadContractRequestsByRenterId(id);
      setRenterRequests(res);
    }catch(error){
      console.error(error);
    }
  }

  useEffect(() => {
    const id = localStorage.getItem("USER_ID");
    if (id && updated){
      loadRequests(id);
      loadRenterRequests(id);
    }
  }, [updated]);

  const RequestInfo = ({ request }: { request: SolicitacaoContrato }) => {
    const [addressInfo, setAddressInfo] = useState<Address>();

    const loadRenterAddress = async () => {
      try {
        const address = await loadAddressUserId(request.idlocatario);
        setAddressInfo(address);
      } catch (error) {
        console.error(error);
      }
    }

    useEffect(() => {
      loadRenterAddress();
    }, []);

    return(
      <Card className="border-[hsl(var(--primary))] bg-[hsl(var(--hover))] m-2 mt-4">
        <CardContent>
          <p className="text-black mt-4 mb-4">Endereço do Solicitante</p>
          <CardContent className="grid grid-cols-1 md:grid-cols-4 gap-4 mt-4 p-0 m-0">
            <Label className="mt-2 mb-2">CEP</Label>
            <Input
              value={addressInfo?.cep}
              disabled={true}
              className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

            <Label className="mt-2 mb-2">Estado</Label>
            <Input
              value={addressInfo?.estado}
              disabled={true}
              className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>

            <Label className="mt-2 mb-2">Cidade</Label>
            <Input
              value={addressInfo?.cidade}
              disabled={true}
              className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
      

            <Label className="mt-2 mb-2">Bairro</Label>
            <Input
              value={addressInfo?.bairro}
              disabled={true}
              className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
            
            <Label className="mt-2 mb-2">Rua</Label>
            <Input
              value={addressInfo?.logradouro}
              disabled={true}
              className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
          
            <Label className="mt-2 mb-2">Número</Label>
            <Input
              value={addressInfo?.numero}
              disabled={true}
              className="p-2 text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/> 
          </CardContent>
        </CardContent>
      </Card>
    );

  }

  const ContractInfo = ({ request }: { request: SolicitacaoContrato }) => {
    const [contractInfo, setContractInfo] = useState<Contract>();
    const [loadingPdf, setLoadingPdf] = useState(false);

    const handleLoadPdf = async () => {
      setLoadingPdf(true);
      try {
      const data = await loadPdfByRequestId(request.idsolicitacao);
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

    const loadContractInfo = async () => {
      try{
        const res = await loadContractByRequestId(request.idsolicitacao);
        setContractInfo(res);
      }catch(error){
        console.error(error);
      }
    }
    useEffect(() => {
      loadContractInfo();
    }, []);

    return (
    <Card className="border-[hsl(var(--primary))] bg-[hsl(var(--hover))] mb-2 w-full">
      <CardContent className="grid grid-cols-1 md:flex gap-4 mt-4">
        <Label className="mt-2 mb-2 w-full md:w-[30%]">Data do Contrato</Label>
        <Input
          value={formatDate(contractInfo?.datacontrato)}
          disabled={true}
          className="p-2 w-full md:w-[50%] text-black bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
        
        <Label className="mt-2 mb-2 w-full md:w-[50%]">Status do Contrato</Label>
        <Input
          value={contractInfo?.statuscontrato}
          disabled={true}
          className="p-2 text-black w-full md:w-[50%] bg-white rounded-md border-[1px] border-[hsl(var(--primary))] w-[100%]"/>
        <Button onClick={handleLoadPdf} disabled={loadingPdf}>
         {loadingPdf ? ("Carregando...") : ("Ver contrato")}</Button>  
      </CardContent>
    </Card>
    )
  }

  const RequestCard = ({ request }: { request: SolicitacaoContrato }) => {
    const [requestOrigin, setRequestOrigin] = useState<User>();
    const [requestMachine, setRequestMachine] = useState<Machine>();
    const [showRequestInfo, setShowRequestInfo] = useState(false);


    const tempoAluguel = request.prazolocacao + " " + request.medidatempolocacao;


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
      <CardContent>
        <Card className="border-[hsl(var(--primary))] bg-[hsl(var(--hover))] mb-0 mt-4">
        <CardContent className="grid grid-cols-1 md:grid-cols-4 gap-4 mt-4">
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
            <CardFooter className="flex justify-center">
            <Button onClick={() => setShowRequestInfo(!showRequestInfo)}>
              {showRequestInfo ? ("Ocultar detalhes da solicitação")
              : ("Ver detalhes da solicitação")} </Button>
            </CardFooter>
            {showRequestInfo && <RequestInfo request={request} />}
        </Card>
      </CardContent>
      {request.origemsolicitacao !== localStorage.getItem("USER_ID") && (
        <div>
        {request.statussolicitacao === "Aguardando aprovação" && (
          <CardFooter className="grid grid-cols-1 md:flex justify-center gap-4">
            <Button className="bg-[#882727]"
            onClick={() => {handleUpdateRequest("Solicitação recusada")}}>Recusar Aluguel</Button>
            <Button
            onClick={() => {handleUpdateRequest("Solicitação aprovada")}}>Aprovar Aluguel</Button>
          </CardFooter>
        )}
        </div>
      )}

      {request.statussolicitacao === "Solicitação aprovada" && (
        <CardFooter className="grid grid-cols-1 md:flex justify-center gap-4">
            <ContractInfo request={request}/>      
        </CardFooter>
      )}
    </Card>
    )
  }

  return(
    <Layout>
      <main className="contract-dropdown-container w-full m-0">
      <Card className="w-full md:w-[60%] mt-2 bg-[hsl(var(--machine-card-bg))] pb-4 border-[hsl(var(--primary))] mb-10 m-0">
      <CardHeader className="text-[hsl(var(--text))] text-[1.25rem]">
        <strong>Solicitações de Contrato</strong>
      </CardHeader>
        <CardContent className="flex flex-col items-center justify-center h-full text-center p-5">
        <Tabs defaultValue="recebidas" className="w-full">
            <TabsList className="grid bg-[hsl(var(--background))] w-full h-full p-0 pb-4 m-0 sm:flex sm:flex-col  bg-[hsl(var(--background))]">
            <TabsTrigger 
              className="text-[hsl(var(--primary))] data-[state=active]:bg-[hsl(var(--primary))] 
              data-[state=active]:text-[hsl(var(--text))] data-[state=active]:border-2 
              data-[state=active]:border-[hsl(var(--primary))]" 
              value="recebidas"
            >
              Solicitações recebidas
            </TabsTrigger>
            <TabsTrigger 
              className="text-[hsl(var(--primary))] data-[state=active]:bg-[hsl(var(--primary))]
               data-[state=active]:text-[hsl(var(--text))] data-[state=active]:border-2 
               data-[state=active]:border-[hsl(var(--primary))]" 
              value="emitidas"
            >
              Solicitações emitidas
            </TabsTrigger>
            </TabsList>
          <TabsContent value="recebidas">

            {(requests && requests?.length > 0) ? requests?.map((req) => (
              <RequestCard request={req}/>
            ))
            :(
              <p className="text-[hsl(var(--text))] mt-4">
              Você ainda não possui solicitações de contrato.
            </p>
            )}

          </TabsContent>
          <TabsContent value="emitidas">
            
          {(renterRequests && renterRequests.length > 0) ? renterRequests?.map((req) => (
            <RequestCard request={req}/>
          )) 
          : (
            <p className="text-[hsl(var(--text))] mt-4">
              Você ainda não possui solicitações de contrato.
            </p>
          )}

          </TabsContent>
        </Tabs>
        </CardContent>
        </Card>
      </main>
    </Layout>
  );
}