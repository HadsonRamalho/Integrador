use crate::model;
use crate::controller::locadora::locadora;

pub struct Locadora{
    pub idlocadora: String,
    pub idendereco: String,
    pub cnpj: String,
    pub numerocontabanco: String,
    pub numeroagenciabanco: String,
    pub nomebanco: String,
    pub nomelocadora: String
}

pub async fn _cadastra_locadora(locadora: Locadora) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO locadora (idlocadora, idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora)
          VALUES (:idlocadora, :idendereco, :cnpj, :numerocontabanco, :numeroagenciabanco, :nomebanco, :nomelocadora)", 
         params! {"idlocadora" =>  locadora.idlocadora, "idendereco" => locadora.idendereco, "cnpj" => locadora.cnpj, "numerocontabanco" => locadora.numerocontabanco,
            "numeroagenciabanco" => locadora.numeroagenciabanco, "nomebanco" => locadora.nomebanco, "nomelocadora" => locadora.nomelocadora}).await;
    match resultado_insert{
        Ok(_) => {
            println!("Locadora cadastrada");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
    return Ok(());
}

pub async fn _busca_id_locadora(cnpj: &str) -> Result<String, mysql_async::Error>{

    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Option<String>, mysql_async::Error> = conn.exec_first("SELECT idlocadora FROM locadora WHERE cnpj = :cnpj",
     params!{"cnpj" => cnpj}).await;
    match resultado_busca{
        Ok(id) => {
            match id {
                Some(id) => {
                    return Ok(id);
                }, None =>{
                    return Ok("".to_string());
                }
            }
        },
        Err(e) => {
            return Err(e);
        }
    }
}