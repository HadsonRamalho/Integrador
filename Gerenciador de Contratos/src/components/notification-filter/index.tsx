import React from "react";
import { Card, CardContent, CardDescription, CardHeader } from "@/components/ui/card";

interface NotificationFilterProps {
  notifications: { status: string }[];
  filter: string;
  setFilter: (filter: string) => void;
}

const NotificationFilter: React.FC<NotificationFilterProps> = ({ notifications , filter, setFilter }) => {
  return (
    <div className="flex justify-center mb-4 w-full">
      <Card className="bg-[hsl(var(--machine-card-bg))] h-[120px] border-[hsl(var(--primary))] w-full">
        <CardHeader>
          <p>Filtrar Notificações</p>
        </CardHeader>
        <CardContent>
          <CardDescription className="mb-4">
            <div className="flex justify-center">
              <select
                className="w-full md:w-[60%] pl-2 bg-white h-[30px] text-black border-[hsl(var(--primary))] rounded-md border-[1px]"
                onChange={(e) => setFilter(e.target.value)}
                value={filter}
                required
              >
                <option value="">Todas as notificações</option>
                {notifications
                  .map((notification) => notification.status)
                  .filter(
                    (status, index, self) =>
                      self.indexOf(status) === index
                  )
                  .map((status) => (
                    <option key={status} value={status}>
                      {status}
                    </option>
                  ))}
              </select>
            </div>
          </CardDescription>
        </CardContent>
      </Card>
    </div>
  );
};

export default NotificationFilter;