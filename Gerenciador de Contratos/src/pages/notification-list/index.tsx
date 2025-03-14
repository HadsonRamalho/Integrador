import Layout from "@/layouts/default";
import { useEffect, useState } from "react";
import {
  Card,
  CardContent,
  CardHeader,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { NotificationCard } from "@/components/notification-card";
import { Notification } from "@/interfaces/notifications";
import { loadNotificationsByUserId } from "@/services/api/notifications/notifications";

export const NotificationList = () => {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [filter, setFilter] = useState("");

  useEffect(() => {
    const id = localStorage.getItem("USER_ID");
    const loadNotifications = async (id: string) => {
      const notificationArray = await loadNotificationsByUserId(id);
      console.log(notificationArray || "Nenhuma notificação");
      setNotifications(notificationArray);
    };
    if (notifications.length === 0 && id) {
      loadNotifications(id);
    }
  }, [notifications]);

  const filteredMachines = notifications.filter((machine) =>
    filter ? machine.status === filter : true
  );

  return (
    <Layout>
      <main className="mt-10 mb-10 flex justify-center items-center sm:w-full">
        <Card className="bg-[hsl(var(--machine-card-bg))] m-4 border-[hsl(var(--primary))]
         sm:w-full md:w-[50vw]">
            <CardHeader className="text-center">
            <p className="text-[1.2rem]">Notificações</p>
            </CardHeader>
          <CardContent>
          <div className="FILTRO flex justify-center items-center rounded-md bg-[hsl(var(--primary))] mt-0 mb-4">
              
          </div>
          <div className={`grid`}>
          {filteredMachines.length === 0 ? (
            <Card>
              <CardHeader>
                <h2 className="text-[hsl(var(--primary))]">
                  Erro ao carregar lista de notificações
                </h2>
              </CardHeader>
              <CardContent>
                <div>
                  <p className="mb-2 text-[hsl(var(--primary))]">
                    Você ainda não possui notificações.
                    <br />
                  </p>
                  <div className="flex justify-center items-center">
                    <Button className="m-2">Relatar problema</Button>
                  </div>
                </div>
              </CardContent>
            </Card>
          ) : (
            filteredMachines.map((notification: Notification) => (
              <div
                key={notification.idnotificacao}
                className="border-[hsl(var(--primary))] mb-4">
                <NotificationCard notification={notification} />
              </div>
            ))
          )}
          </div>
          </CardContent>
        </Card>
      </main>
    </Layout>
  );
};
