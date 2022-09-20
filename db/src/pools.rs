
use async_once::AsyncOnce;
use super::{HssDatabase, DbPool};

lazy_static! {
	/// global pool for Bom database
	pub static ref BOM: AsyncOnce<DbPool> = AsyncOnce::new( async {
		HssDatabase::Bom.build_pool().await
	});

	/// global pool for Sigmanest database
	pub static ref SIGMANEST: AsyncOnce<DbPool> = AsyncOnce::new( async {
		HssDatabase::Sigmanest.build_pool().await
	});
}
