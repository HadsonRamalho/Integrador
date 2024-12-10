import { NavLink } from "react-router-dom";
import "./navbar.css";

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
            Página inicial
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
