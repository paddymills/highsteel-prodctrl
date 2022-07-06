
use prodctrl::cnf::ProdFileProcessor;

#[tokio::main]
async fn main() -> Result<(), prodctrl::Error> {
    let processor = ProdFileProcessor::new();
    processor.process_files()?;
    
    Ok(())
}
