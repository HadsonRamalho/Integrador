import { NavLink } from "react-router-dom";
import "./navbar.css";
import maq from "@/assets/maq.png";
import { ModeToggle } from "../mode-toggle/mode-toggle";
import { useEffect, useState } from "react";
import { ProfileDropdownMenu } from "../profile-dropdown-menu";
import { DropdownMenuDemo } from "../dropdown-menu";
import { MachineDropdownMenu } from "../machine-dropdown-menu";


export function NavBar() {
  const [logged, setLogged] = useState(false);
  
  useEffect(() => {
    const id = localStorage.getItem("USER_ID");
    if(id){
      setLogged(true);
      return;
    }
  },[])

  return (
    <nav className="navbar">
     <ul className="nav-left">
        <li>
          <NavLink
           to="/"
            className={({ isActive }) => ` class1 ${isActive ? "link-nav-active" : ""}`
            }
          >
            <img className="imagem" src={maq} alt="" style={{width: '100%', height: '100%'}}/>
          </NavLink>
        </li>

        <li>
          <NavLink 
          to="/"
          className={({isActive}) => ` class1 ${isActive ? "link-nav-active" : ""}`
          }
          >
            Pagina Inicial   
          </NavLink>
        </li>
      
        <li>
          <NavLink
            to="/howworks"
            className={({ isActive }) => `class1 ${isActive ? "link-nav-active" : ""}`
          }
          >
            Como Funciona
          </NavLink>
        </li>
        <li>
        {logged ? (
            <MachineDropdownMenu>
            </MachineDropdownMenu>
          ) : (
            <NavLink 
            to="/machine"
            className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
              Máquinas
            </NavLink>
            )            
          }              
        </li>
        <li>
          <NavLink
            to="/pdf-example"
            className={({ isActive }) => `class1 ${isActive ? "link-nav-active" : ""}`
          }
          >
            Exemplo de PDF
          </NavLink>
        </li>
        </ul>

        <ul className="nav-right">
        <li className="class1">
          <DropdownMenuDemo></DropdownMenuDemo>
        </li>       

        <li>        
          {logged ? (
            <ProfileDropdownMenu titulo={"Meu Perfil"} >
            </ProfileDropdownMenu>
          ) : (
            <NavLink 
            to="/login"
            className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
              Entrar
            </NavLink>
            )            
          }          
        </li>

        <li>
          <ModeToggle></ModeToggle>
        </li>        

      </ul>
    </nav>
  );
}
