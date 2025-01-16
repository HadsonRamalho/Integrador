import { Button } from "@/components/ui/button"
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu"
import { Input } from "@/layouts"


export function DropdownMenuDemo() {
  return (
    <DropdownMenu>
  <DropdownMenuTrigger>Open</DropdownMenuTrigger>
  <DropdownMenuContent>
    <DropdownMenuLabel>Selecione </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
     <Input
     placeholder="CEP"
     type="cep"
     />
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
      <Input
      placeholder="País"
      />
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
      <Input
      placeholder="Estado"/>
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
      <Input
      placeholder="Cidade"/>
    </DropdownMenuLabel>
    <DropdownMenuSeparator />
    <DropdownMenuLabel>
      <Input
      placeholder="Moeda"/>
    </DropdownMenuLabel>
    <DropdownMenuItem>
    <Button>Confirmar</Button>
    </DropdownMenuItem>
  </DropdownMenuContent>
</DropdownMenu>

  )
}
