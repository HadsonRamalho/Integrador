import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
  } from "@/components/ui/dropdown-menu";
import { NavLink } from "react-router-dom";
import { useAuth } from "@/hooks/auth";
import { Avatar, AvatarImage } from "@/components/ui/avatar"
import { useNavigate } from "react-router-dom";


export const ProfileDropdownMenu = () => {
    const { signOut } = useAuth();
    const navigate = useNavigate();
    const LogOut = () => {
        signOut();
        localStorage.removeItem("PROFILE_IMAGE_URL");
        localStorage.removeItem("USER_ID");
        localStorage.removeItem("cidade_dropdownmenu");
        navigate("/login");
    };
    
    return(
        <div>
        <DropdownMenu>
        <DropdownMenuTrigger>
        <Avatar className="border-[hsl(var(--primary))] border-[2px]">
                <AvatarImage src={localStorage.getItem("PROFILE_IMAGE_URL") || "https://i.pinimg.com/736x/f1/13/b7/f113b7eb12a6e28b201152535c8b89da.jpg"} />                    
        </Avatar>
        </DropdownMenuTrigger>
        <DropdownMenuContent style={{zIndex: 1001}}>
            <DropdownMenuLabel className="flex items-center justify-center">
                <Avatar className="border-[hsl(var(--primary))] border-[2px]">
                <AvatarImage src={localStorage.getItem("PROFILE_IMAGE_URL") || "https://i.pinimg.com/736x/f1/13/b7/f113b7eb12a6e28b201152535c8b89da.jpg"} />                    
                </Avatar>
            
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
                <NavLink to={"/user-profile"} className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
                  <p style={{color: "black", fontSize: '15px'}}>Configuração da Conta</p>
                </NavLink>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
                <NavLink to={"/contract-request"}>
                    <p style={{color: "black", fontSize: '15px'}}>Solicitações de Contratos</p>
                </NavLink>
            </DropdownMenuItem>
            <DropdownMenuSeparator/>
            <DropdownMenuItem>
            <NavLink to={"/notification-list"}>
                    <p style={{color: "black", fontSize: '15px'}}>Notificações</p>
                </NavLink>
            </DropdownMenuItem>
            <DropdownMenuSeparator/>
            <DropdownMenuLabel>
                <strong>Suporte</strong>
            </DropdownMenuLabel>
           
            <DropdownMenuItem>
            <NavLink to={"/helpcenter"} className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
            <p style={{color: "black", fontSize: '15px'}}>Central de Ajuda</p>
            </NavLink>
            </DropdownMenuItem>
            <DropdownMenuSeparator/>
            <DropdownMenuItem onClick={LogOut}>
                <p style={{color: "black"}}>Sair da conta</p>
            </DropdownMenuItem>
        </DropdownMenuContent>
        </DropdownMenu>
        </div>
    );
}