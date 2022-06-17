
use bb8::Pool;
use bb8_tiberius::{ConnectionManager};
use tiberius::{AuthMethod, Config, Query};

use crate::part::Part;

const HOST: &str = "HSSSQLSERV";
const POOL_SIZE: u32 = 2;

pub struct App {
    pub pool: Pool<ConnectionManager>
}

impl App {
    pub async fn new() -> Self {
        let mut config = Config::new();
        config.host(HOST);
        config.authentication(AuthMethod::Integrated);
        config.trust_cert();

        let mgr = match ConnectionManager::build(config) {
            Ok(conn_mgr) => conn_mgr,
            Err(_) => panic!("ConnectionManager failed to connect to database")
        };

        match Pool::builder()
            .max_size(POOL_SIZE)
            .build(mgr)
            .await {
                Ok(pool) => Self { pool },
                Err(_) => panic!("Pool failed to build")
            }
    }

    pub async fn init_bom(self, job: &str, shipment: i32) -> Result<Vec<Part>, crate::Error> {
        let mut _query = Query::new("EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2");
        // query.bind(&job);
        // query.bind(&shipment);

        
        let res = self.pool.get()
            .await?
            .query(
                "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
                &[&job, &shipment]
            )
            .await?
            .into_results()
            .await?
            .into_iter()
            .flatten()
            .map( |row| Part::from(row) )
            .collect();

        Ok(res)
    }
}