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

export const ProfileDropdownMenu = ({titulo}:{titulo:string}) => {
    const { signOut } = useAuth();
    const navigate = useNavigate();
    const LogOut = () => {
        signOut();
        localStorage.removeItem("USER_ID");
        navigate("/login");
    };
    
    return(
        <div>
        <DropdownMenu>
        <DropdownMenuTrigger><p style={{color: "white"}}>{titulo}</p></DropdownMenuTrigger>
        <DropdownMenuContent>
            <DropdownMenuLabel>
                <Avatar>
                <AvatarImage src="https://i.pinimg.com/736x/f1/13/b7/f113b7eb12a6e28b201152535c8b89da.jpg" />                    
                </Avatar>
                <p>Meu Perfil</p>
            </DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem>
                <NavLink to={"/user-profile"} className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
                  <p style={{color: "black", fontSize: '15px'}}>Minhas Informações</p>
                </NavLink>
            </DropdownMenuItem>
            <DropdownMenuItem onClick={LogOut}>
                <p style={{color: "black"}}>Sair da conta</p>
            </DropdownMenuItem>
        </DropdownMenuContent>
        </DropdownMenu>
        </div>
    );
}