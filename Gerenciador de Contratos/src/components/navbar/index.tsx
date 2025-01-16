import { NavLink } from "react-router-dom";
import "./navbar.css";
import maq from "@/assets/maq.png";
import { ModeToggle } from "../mode-toggle/mode-toggle";
import { DropdownMenuDemo } from "../dropdown-menu";


export function NavBar() {
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
        
          <NavLink 
          to="/login"
            className={({ isActive }) => `class1  ${isActive ? "link-nav-active" : ""}`
          }
           >
              Entrar
          </NavLink>
        </li>

       

      </ul>
    </nav>
  );
}
