
use crate::prelude::*;


/// Trait to add confirmation file db operations to database Client
#[async_trait]
pub trait SnCnfDbOps {
    /// Gets the cost center for a given program and piece mark
    async fn get_cc(&mut self, program: &String, mark: &String) -> Result<Option<String>>;
}

#[async_trait]
impl SnCnfDbOps for DbClient {
    async fn get_cc(&mut self, program: &String, mark: &String) -> Result<Option<String>> {
		let sn_mark = mark.replacen("-", "_", 1);

        let res = self
            .query(
                "
					SELECT
						Data3 AS CostCenter
					FROM Part
					INNER JOIN PIPArchive AS PIP
						ON Part.PartName=PIP.PartName
						AND Part.WONumber=PIP.WONumber
					WHERE PIP.ProgramName=@P1
					AND PIP.PartName LIKE @P2
					AND PIP.TransType='SN102'
				",
                &[program, &sn_mark]
            )
            	.await?
            .into_row()
				.await?
			.map(|r| r.get::<&str, _>("CostCenter").unwrap_or_default().into());
    
        Ok(res)
    }
}