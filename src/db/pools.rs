
use async_once::AsyncOnce;
use super::{HssDatabase, DbPool};

lazy_static! {
	static ref BOM: AsyncOnce<DbPool> = AsyncOnce::new( async {
		HssDatabase::Bom.build_pool().await
	});

	static ref SIGMANEST: AsyncOnce<DbPool> = AsyncOnce::new( async {
		HssDatabase::Sigmanest.build_pool().await
	});
}
