import React from "react";

interface EquipmentSelectProps {
  onChange: (event: React.ChangeEvent<HTMLSelectElement>) => void;
  className?: string;
}

const equipmentCategories: { [key: string]: string[] } = {
  "Movimentação de Terra e Escavação": [
    "Retroescavadeira",
    "Escavadeira Hidráulica",
    "Motoniveladora",
    "Pá Carregadeira",
    "Trator de Esteira",
    "Minicarregadeira",
  ],
  "Transporte e Logística": [
    "Caminhão Basculante (Caçamba)",
    "Caminhão Pipa",
    "Caminhão Comboio",
    "Carreta Prancha",
  ],
  "Perfuração e Exploração Mineral": [
    "Perfuratriz Hidráulica",
    "Jumbo de Perfuração",
    "Sonda de Perfuração",
    "Martelo Pneumático",
  ],
  "Demolição e Rompimento de Rochas": [
    "Rompe Rochas Hidráulico",
    "Britador de Mandíbula",
    "Britador Cônico",
    "Martelete Demolidor",
  ],
  "Processamento e Beneficiamento de Minérios": [
    "Peneira Vibratória",
    "Esteira Transportadora",
    "Moega Alimentadora",
    "Classificador Espiral",
    "Ciclone Hidráulico",
  ],
  "Compactação e Pavimentação": [
    "Rolo Compactador",
    "Placa Vibratória",
    "Compactador de Solo (Sapo)",
    "Usina de Asfalto",
  ],
  "Equipamentos Manuais e Auxiliares": [
    "Enxada",
    "Pá",
    "Picareta",
    "Marreta",
    "Carrinho de Mão",
    "Serra Circular",
  ],
};

const EquipmentSelect: React.FC<EquipmentSelectProps> = ({ onChange, className }) => {
  return (
    <select
      onChange={onChange}
      className={`border p-2 rounded w-full ${className || ""}`}
    >
      <option value="">Selecione uma categoria...</option>
      {Object.entries(equipmentCategories).map(([category, items]) => (
        <optgroup key={category} label={category}>
          {items.map((item) => (
            <option key={item} value={item}>
              {item}
            </option>
          ))}
        </optgroup>
      ))}
    </select>
  );
};

export default EquipmentSelect;
