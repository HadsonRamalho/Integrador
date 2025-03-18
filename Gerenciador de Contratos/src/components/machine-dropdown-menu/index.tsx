import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
  } from "@/components/ui/dropdown-menu";
import { NavLink } from "react-router-dom";

interface MachineDropdownMenuProps {
    triggerColor: string;
}

export const MachineDropdownMenu: React.FC<MachineDropdownMenuProps> = ({ triggerColor }) => {    
    return(
        <div>
        <DropdownMenu>
        <DropdownMenuTrigger>
        <p style={{color: triggerColor || "white"}}>Máquinas</p>
        </DropdownMenuTrigger>
        <DropdownMenuContent style={{zIndex: 1001}}>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
                <NavLink to={"/machine"} className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
                  <p style={{color: "black", fontSize: '15px'}}>Máquinas Disponíveis</p>
                </NavLink>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
            <NavLink to={"/create-machine"} className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
                  <p style={{color: "black", fontSize: '15px'}}>Cadastrar Máquinas</p>
                </NavLink>
            </DropdownMenuItem>
            <DropdownMenuSeparator/>
            <DropdownMenuItem>
              <NavLink to={"/machine-list"} className={({isActive}) =>`class1 ${isActive} ? "link-nav-active" : ""} `}>
              <p style={{color: "black", fontSize: '15px'}}>Minhas Máquinas</p>
              </NavLink>
            </DropdownMenuItem>
            <DropdownMenuSeparator/>        
        </DropdownMenuContent>
        </DropdownMenu>
        </div>
    );
}