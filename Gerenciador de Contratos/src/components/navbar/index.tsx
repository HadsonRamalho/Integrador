import { NavLink } from "react-router-dom";
import "./navbar.css";

export function NavBar() {
  return (
    <nav>
      <ul>
        <li>
          <NavLink
            className={({ isActive }) => (isActive ? "link-nav-active" : "")}
            to="/"
          >
            Página inicial
          </NavLink>
        </li>
        <li>
          <NavLink
            className={({ isActive }) => (isActive ? "link-nav-active" : "")}
            to="/sobre"
          >
            Sobre
          </NavLink>
        </li>
      </ul>
    </nav>
  );
}
