
use crate::{
    prelude::*,
    Part
};

/// SimTrans Transactions
/// 
/// [reference](http://hssappserv1:3000/HelpCenter/content/transaction-reference.htm)
/// [variables](http://hssappserv1:3000/HelpCenter/content/using-simtrans/transaction-variables.htm)
/// 
/// for work order transactions, you always need
///     - TransType
///     - District
///     - OrderNo
///     - ItemName(or FileName)
/// 
/// Work order transactions
/// 
/// |TransType|Description|
/// |---|---|
/// |SN80|Add (missing/undefined) part|
/// |SN81|Modify existing part|
/// |SN81B|Modify existing part(excludes in-process and completed)|
/// |SN82|Cancel(delete) part|
/// |SN83|Add std shape|
/// |SN84|Add (existing) part|
/// |SN84A|Add (existing) part (add quantity to existing)|
/// |SN85|Add CAD part|
/// |SN86|Add BOM file|
/// |SN87|Modify work order attribute|
/// |SN88|Add existing to new work order (SN84, but fails if work order exists)|
/// |SN89|Cancel work order|
/// |SN89I|Cancel all not-in-progress work orders|
/// |SN89N|Cancel all work orders|


// TODO: design decision
//  - import and update mechanism with all data
//  - import/update in pieces
//      - initial part insert
//      - change qty
//      - change matl
//      - add/change material master
//      - add/change chargeref
//      - add/change other data (dwg, ops, etc.)
pub async fn import_workorder(client: &mut DbClient, parts: Vec<Part>) -> Result<()> {
    // let sn_mark = |mark: String| mark.replacen("-", "_", 1);

    for _part in parts {
        client
            .execute(
                "
                    INSERT INTO
                        TransAct (
                            TransType, District,                        -- 'SN84', 2
                            OrderNo, ItemName, Qty, ItemData9,          -- workorder, part name, qty, piece mark
                            Material, Customer, DwgNumber,              -- matl grade, state, dwg
                            Remark, ItemData6, ItemData7, ItemData8,    -- remark, op1, op2, op3
                            Data1, Data2,                               -- job, shipment
                            ItemData5,                                  -- charge ref
                            ItemData10, ItemData11, ItemData12          -- material master, raw size, part size
                            ItemData14                                  -- 'HighHeatNum'
                        )
                    VALUES
                        (
                            'SN84', 2,              -- <TransType>, <District>
                            @P1, @P2, @P3, @P4,     -- workorder, part name, qty, piece mark
                            @P5, @P6, @P7,          -- matl grade, state, dwg
                            @P8, @P9, @P10, @P11,   -- remark, op1, op2, op3
                            @P12, @P13,             -- job, shipment
                            @P14,                   -- charge ref
                            @P15, @P16, @P17,       -- material master, raw size, part size
                            'HighHeatNum'           -- <HeatSwapKeyWord>
                        )
                ",
                &[]
            )
                .await?;
    }

    Ok(())
}
