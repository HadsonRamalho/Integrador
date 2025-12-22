import { Card } from "../ui/card";
import { useNavigate } from "react-router-dom";
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
import {
  CardContent,
} from "@/components/ui/card";
import { Notification } from "@/interfaces/notifications";
import { formatDate } from "@/services/api/format/format";
import { useState } from "react";
import { BellIcon, BellRing } from "lucide-react";
import { updateNotificationStatus } from "@/services/api/notifications/notifications";

export const NotificationCard: React.FC<{ notification: Notification }> = ({ notification }) => {
  const navigate = useNavigate();
  
  const [showAlert, setShowAlert] = useState(false);
  const toggleAlert = () => setShowAlert(!showAlert);

  const verifyAccountStatus = async () => {
    try{
      await updateNotificationStatus(notification.idnotificacao, "Lida");
    }catch(error){
      console.error(error);
      return;
    }
    if (localStorage.getItem("USER_ID") === notification.idusuario) {
      navigate(notification.onclick);
      return;
    }
    toggleAlert();
    return;
  };

  return (
    <div>
      <div>
        <AlertDialog open={showAlert} onOpenChange={toggleAlert}>
          <AlertDialogTrigger asChild></AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle style={{ color: "hsl(var(--text))" }}>
                Você não está conectado!
              </AlertDialogTitle>
              <AlertDialogDescription style={{ color: "hsl(var(--text))" }}>
                Você precisa estar logado pra fazer isso, por favor 
                entre na sua conta.
              </AlertDialogDescription>
              </AlertDialogHeader>
              <AlertDialogFooter>
                <AlertDialogCancel
                  className="text-[hsl(var(--text))] bg-[#882727]">
                  Cancelar
                </AlertDialogCancel>
                <AlertDialogAction
                  onClick={() => {
                    navigate("/login");
                  }}>
                  Entrar/Registrar 
                </AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </div>
        <Card
          className={`hover:cursor-pointer p-4 m-0 border-[hsl(var(--primary))] 
            transition-transform duration-150 ease-in-out hover:scale-[1.015] ${
            notification.status === "Não lida" ? "border-[2px]" : "border-[black]"
          }`}
          onClick={() => {
            verifyAccountStatus();
          }}>
        <CardContent className=" p-0 m-0">
          <div className="flex justify-center items-center">
            {notification.status === "Não lida" ? (
              <BellRing color="hsl(var(--primary))"/>
            ) : (
              <BellIcon color="black"/>
            )}
            <p className={`m-4 text-[1.1rem] ${notification.status === "Não lida" ? "text-[hsl(var(--primary))]" : "text-[black]"}`}>
              <strong>{notification.titulo}</strong>
            </p>
          </div>
            <p className="flex justify-center items-center text-[black]">
              {notification.mensagem}
            </p>
          <p className="flex justify-center items-center mt-2 text-[black]">
            <strong> Data: {formatDate(notification.datacriacao)}</strong>            
          </p>
          <p className="flex justify-center items-center mt-2 text-[black]">
            {notification.status}            
          </p>
        </CardContent>
      </Card>
    </div>
  );
};