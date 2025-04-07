import { NavLink } from "react-router-dom";
import "./navbar.css";
import maq from "@/assets/maq.png";
import { ModeToggle } from "../mode-toggle/mode-toggle";
import { useEffect, useState } from "react";
import { ProfileDropdownMenu } from "../profile-dropdown-menu";
import { DropdownMenuDemo } from "../dropdown-menu";
import { MachineDropdownMenu } from "../machine-dropdown-menu";
import { MenuIcon } from "lucide-react";

export function NavBar() {
  const [logged, setLogged] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);

  useEffect(() => {
    const id = localStorage.getItem("USER_ID");
    if (id) {
      setLogged(true);
      return;
    }
  }, []);

  return (
    <nav>
      <ul className="nav-left">
        <li>
          <NavLink
            to="/"
            className={({ isActive }) =>
              ` class1 ${isActive ? "link-nav-active" : ""}`
            }
          >
            <img
              className="imagem"
              src={maq}
              alt=""
              style={{ width: "100%", height: "100%" }}
            />
          </NavLink>
        </li>

        <li>
          <NavLink
            to="/"
            className={({ isActive }) =>
              ` class1 ${isActive ? "link-nav-active" : ""} hidden sm:flex`
            }
          >
            Pagina Inicial
          </NavLink>
        </li>

        <li>
          <NavLink
            to="/howworks"
            className={({ isActive }) =>
              `class1 ${isActive ? "link-nav-active" : ""} hidden sm:flex`
            }
          >
            Como Funciona
          </NavLink>
        </li>
        <li className=" hidden sm:flex">
          {logged ? (
            <MachineDropdownMenu
              triggerColor={""}
            ></MachineDropdownMenu>
          ) : (
            <NavLink
              to="/machine"
              className={({ isActive }) =>
                `class1  ${isActive ? "link-nav-active" : ""} hidden sm:flex`
              }
            >
              Máquinas
            </NavLink>
          )}
        </li>
        <li>
          <NavLink
            to="/about"
            className={({ isActive }) =>
              `class1 ${isActive ? "link-nav-active" : ""} hidden sm:flex`
            }
          >
            Sobre Nós
          </NavLink>
        </li>
      </ul>
      {menuOpen && (
        <ul
          style={{ zIndex: 1000 }}
          className="sm:hidden absolute top-0 left-0 w-full h-[60svh] bg-[hsl(var(--machine-card-bg))] text-red flex flex-col items-center justify-center space-y-6 text-lg"
        >
          <li>
            <NavLink
              to="/"
              className="class1"
              onClick={() => setMenuOpen(false)}
            >
              <p className="text-[hsl(var(--text))]">Início</p>
            </NavLink>
          </li>
          <li>
            <NavLink
              to="/howworks"
              className="class1"
              onClick={() => setMenuOpen(false)}
            >
              <p className="text-[hsl(var(--text))]">Como Funciona</p>
            </NavLink>
          </li>
          <li>
            {logged ? (
              <MachineDropdownMenu triggerColor="" />
            ) : (
              <NavLink
                to="/machine"
                className="class1"
                onClick={() => setMenuOpen(false)}
              >
                <p >Máquinas</p>
              </NavLink>
            )}
          </li>
          <li>
            <NavLink
              to="/about"
              className="class1"
              onClick={() => setMenuOpen(false)}
            >
              <p >Sobre</p>
            </NavLink>
          </li>
          <li>
            <DropdownMenuDemo triggerColor="" />
          </li>
          <li>
            {logged ? (
              <ProfileDropdownMenu />
            ) : (
              <NavLink
                to="/login"
                className="class1"
                onClick={() => setMenuOpen(false)}
              >
                <p className="text-[hsl(var(--text))]">Entrar</p>
              </NavLink>
            )}
          </li>
          <li>
            <ModeToggle />
          </li>
          <button
            className="absolute top-4 right-4 text-white text-3xl"
            onClick={() => setMenuOpen(false)}
          >
            <p className="text-[hsl(var(--text))]">✖</p>
          </button>
        </ul>
      )}

      <ul className="nav-right">
        <li className="class1  hidden sm:flex ">
          <DropdownMenuDemo
            triggerColor={""}
          ></DropdownMenuDemo>
        </li>

        <li className=" hidden sm:flex">
          {logged ? (
            <ProfileDropdownMenu></ProfileDropdownMenu>
          ) : (
            <NavLink
              to="/login"
              className={({ isActive }) =>
                `class1  ${isActive ? "link-nav-active" : ""} hidden sm:flex`
              }
            >
              Entrar
            </NavLink>
          )}
        </li>

        <li className=" hidden sm:flex">
          <ModeToggle></ModeToggle>
        </li>

        <li className="flex sm:hidden">
          <button
            onClick={() => {
              setMenuOpen(!menuOpen);
            }}
          >
            <MenuIcon color="white" />
          </button>
        </li>
      </ul>
    </nav>
  );
}
