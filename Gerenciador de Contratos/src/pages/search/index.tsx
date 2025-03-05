import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";

const DetalhesMaquina = () => {
  const { id } = useParams();
  
  interface Maquina {
    id: number;
    nome: string;
    descricao: string;
  }
  
  const [maquina, setMaquina] = useState<Maquina | null>(null);

  useEffect(() => {
  
    const mockData = [
      { id: 1, nome: "Escavadeira", descricao: "Máquina pesada para escavação." },
      { id: 2, nome: "Roçadeira", descricao: "Equipamento para corte de vegetação." },
      { id: 3, nome: "Máquina", descricao: "Máquina genérica para múltiplas funções." },
      { id: 4, nome: "Trator", descricao: "Utilizado para puxar cargas e equipamentos." },
      { id: 5, nome: "Motoniveladora de Combate", descricao: "Para nivelamento de terrenos." },
    ];

    const maquinaSelecionada = mockData.find((m) => m.nome === String(id));
    setMaquina(maquinaSelecionada || null);
  }, [id]);

  if (!maquina) {
    return <p>Máquina não encontrada...</p>;
  }

  return (
    <div >
      <h1 >{maquina.nome}</h1>
      <p >{maquina.descricao}</p>
    </div>
  );
};

export default DetalhesMaquina;
