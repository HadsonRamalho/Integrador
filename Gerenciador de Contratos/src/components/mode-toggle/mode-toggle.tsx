import { Moon, Sun } from "lucide-react"

import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { useTheme } from "../theme-provider/theme-provider"

export function ModeToggle() {
  const { setTheme } = useTheme()
  
  const handleClick = () =>{
    const currentTheme = localStorage.getItem("vite-ui-theme");
    if( currentTheme === "light"){
      setTheme ("dark");
      return;
    }
    setTheme("light");
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="outline" size="icon" onClick={handleClick} className="bg-primary hover:bg-[hsl(150,30%,59%)] dark:hover:bg-[hsl(150,30%,59%)]">
          <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
          <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
          <span className="sr-only">Toggle theme</span>
        </Button>
      </DropdownMenuTrigger>
      
    </DropdownMenu>
  )
}
