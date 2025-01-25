import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
  } from "@/components/ui/dropdown-menu";
import { NavLink } from "react-router-dom";

export const MachineDropdownMenu = () => {    
    return(
        <div>
        <DropdownMenu>
        <DropdownMenuTrigger>
        <p>Máquinas</p>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
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
        </DropdownMenuContent>
        </DropdownMenu>
        </div>
    );
}