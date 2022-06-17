
use simple_excel_writer::*;

use workorder_bb8::{
    App,
    part::Part
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new().await;
    let dwg = Some(String::from("167"));

    let mut wb = Workbook::create("C:\\temp\\1190163A-1_LB.xlsx");
    let mut sheet = wb.create_sheet("LB");

    for _ in 0..2 {
        sheet.add_column(Column { width: 15.0 });
    }


    let mut conn = app.pool.get().await?;
    let res = conn
        .query(
            "EXEC BOM.SAP.GetBOMData @Job=@P1, @Ship=@P2",
            &[&"1190163A", &1i32]
        )
        .await?
        .into_results()
        .await?
        .into_iter()
        .flatten()
        .map( |row| Part::from(row) )
        .filter( |part| part.dwg == dwg );

    wb.write_sheet(&mut sheet, |sw| {
        sw.append_row(row!["Mark", "Shape"," Length"])?;
        for part in res {
            sw.append_row(row![part.mark, part.matl.comm.to_string(), part.matl.len as f64])?;
        }

        Ok(())
    }).expect("Failed to write data");

    wb.close().expect("Failed to close workbook");

    Ok(())
}
