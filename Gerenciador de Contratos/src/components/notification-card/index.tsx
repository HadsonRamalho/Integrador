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
  CardDescription,
} from "@/components/ui/card";
import { Notification } from "@/interfaces/notifications";
import { formatDate } from "@/services/api/format/format";
import { useState } from "react";

export const NotificationCard: React.FC<{ notification: Notification }> = ({ notification }) => {
  const navigate = useNavigate();
  
  const [showAlert, setShowAlert] = useState(false);
  const toggleAlert = () => setShowAlert(!showAlert);

  const verifyAccountStatus = () => {
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
          className="hover:cursor-pointer p-4 m-0 border-[hsl(var(--primary))] transition-transform duration-150 ease-in-out hover:scale-[1.015]"
          onClick={() => {
            verifyAccountStatus();
          }}>
        <CardContent className=" p-0 m-0">
          <div className="flex justify-center items-center">
            <p className="m-4 text-[hsl(var(--primary))] text-[1.1rem]"><strong>{notification.titulo}</strong></p>
          </div>
            <p className="flex justify-center items-center">
              {notification.mensagem}
            </p>
          <p className="flex justify-center items-center mt-2">
            <strong> Data: {formatDate(notification.datacriacao)}</strong>
          </p>
        </CardContent>
      </Card>
    </div>
  );
};