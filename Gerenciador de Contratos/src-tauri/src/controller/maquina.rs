use crate::{
    controller::{self, gera_hash},
    model,
};

#[tauri::command]
pub async fn estrutura_maquina(nomemaquina: String, valoraluguel: String, numserie: String) {
    let idmaquina = gera_hash(&numserie);
    let valoraluguel = valoraluguel.trim().parse().unwrap();
    let maquina = model::maquina::Maquina {
        idmaquina,
        nomemaquina,
        valoraluguel,
        numserie,
    };
}
