use crate::controllers::maquinas::lista_todas_maquinas;

pub async fn mostra_maquinas(){
    let r = lista_todas_maquinas().await.unwrap();
    println!("{}", r.1.0.first().unwrap().idmaquina);
}