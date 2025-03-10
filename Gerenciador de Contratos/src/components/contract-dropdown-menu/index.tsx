import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { NavLink } from "react-router-dom";

export const ContractDropdownMenu = () => {    
  return(
      <div>
      <DropdownMenu>
      <DropdownMenuTrigger>
      <p style={{color: 'white'}}>Contratos</p>
      </DropdownMenuTrigger>
      <DropdownMenuContent>
          <DropdownMenuSeparator />
          <DropdownMenuItem>
          <NavLink to={"/"} className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
                <p style={{color: "black", fontSize: '15px'}}>Meus Contratos</p>
              </NavLink>
          </DropdownMenuItem>
          <DropdownMenuSeparator/>   
          <DropdownMenuItem>
              <NavLink to={"/"}>
                <p style={{color: "black", fontSize: '15px'}}>Solicitações de Contratos</p>
              </NavLink>
          </DropdownMenuItem> 
          <DropdownMenuSeparator />
      </DropdownMenuContent>
      </DropdownMenu>
      </div>
  );
}