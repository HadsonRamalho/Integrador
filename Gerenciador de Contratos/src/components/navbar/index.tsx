import { NavLink } from "react-router-dom";
import "./navbar.css";
import maq from "@/assets/maq.png";
import { ModeToggle } from "../mode-toggle/mode-toggle";
import { DropdownMenuDemo } from "../dropdown-menu";
import { useEffect, useState } from "react";


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
    <nav>
      <ul className="link-nav-active">
        <li>
          <NavLink
           to="/"
            className={({ isActive }) => ` class1 ${isActive ? "link-nav-active" : ""}`
            }
          >
            <img className="imagem" src={maq} alt="" />
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
            to="/sobre"
            className={({ isActive }) => `class1 ${isActive ? "link-nav-active" : ""}`
          }
          >
            Como Funciona
          </NavLink>
        </li>
        <li>
          <NavLink
          to ="/machine"
            className={({ isActive }) => `class1 ${isActive ? "link-nav-active" : ""}`
        }
           >
              Maquinas
          </NavLink>
        </li>
        <li className="class1">
          <DropdownMenuDemo></DropdownMenuDemo>
        </li>
        <li>
          <ModeToggle></ModeToggle>
        </li>
        
        
        
        <li>        
          {logged ? (
            <NavLink 
            to="/user-profile"
            className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
              Meu Perfil
            </NavLink>
          ) : (
            <NavLink 
            to="/login"
            className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`}>
              Entrar
            </NavLink>
            )            
          }          
        </li>

       

      </ul>
    </nav>
  );
}
