
//! Basic app boilerplate
// TODO: remake this crap!

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::Query;

use crate::db::HssDatabase;
use crate::Part;
use crate::ui::cli::CliMenuApp;

const POOL_SIZE: u32 = 2;

/// Basic app struct impl
pub struct App<T>
where
    T: CliMenuApp
{
    /// database pool
    pub pool: Pool<ConnectionManager>,
    /// application
    pub app: T,
}

impl<T> App<T>
where
    T: CliMenuApp
{
    // TODO: remove database stuff

    /// create new app
    pub async fn new() -> Self {
        let mgr = match ConnectionManager::build(HssDatabase::Bom) {
            Ok(conn_mgr) => conn_mgr,
            Err(_) => panic!("ConnectionManager failed to connect to database")
        };

        match Pool::builder()
            .max_size(POOL_SIZE)
            .build(mgr)
            .await {
                Ok(pool) => Self { pool, app: T::init() },
                Err(_) => panic!("Pool failed to build")
            }
    }

    /// Create list of ['Parts'](crate::Part) from Bom
    // TODO: move this to db::bom
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