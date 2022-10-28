use crate::{dao::Config, vo::UpdateConfig, DbPool};
use poem::{
    error::{InternalServerError, NotFound},
    web::{Data, Path},
    Result,
};
use poem_openapi::{
    payload::{Binary, Json, Response},
    OpenApi,
};
use std::path::Path as OsPath;

pub struct Api;

#[OpenApi]
impl Api {

    // 扫描
    #[oai(path = "/scan", method = "get")]
    async fn scan(&self, pool: Data<&DbPool>) -> Result<()> {
        Ok(())
    }

    // 查询列表 -q 设备, 时间, 类型
    #[oai(path = "/list", method = "get")]
    async fn list_photos(&self, pool: Data<&DbPool>) -> Result<()> {
        Ok(())
    }

    // 缩略图
    #[oai(path = "/thumb/:id", method = "get")]
    async fn thumb(
        &self,
        pool: Data<&DbPool>,
        id: Path<String>,
    ) -> Result<Response<Binary<Vec<u8>>>> {
        let config = sqlx::query_as::<_, Config>(
            "SELECT update_time, scan_path, thumb_path, scan_time from config limit 1",
        )
        .fetch_one(pool.0)
        .await
        .map_err(NotFound)?;
        
        let image_content =
            std::fs::read(OsPath::new(&config.thumb_path).join(&id.0)).map_err(NotFound)?;
        Ok(Response::new(Binary(image_content)).header("content-type", "image/jpeg"))
    }

    // 配置信息
    #[oai(path = "/config", method = "get")]
    async fn config(&self, pool: Data<&DbPool>) -> Result<Json<Config>> {
        let config = sqlx::query_as::<_, Config>(
            "SELECT update_time, scan_path, thumb_path, scan_time from config limit 1",
        )
        .fetch_one(pool.0)
        .await
        .map_err(NotFound)?;
        Ok(Json(config))
    }

    // 修改配置信息
    #[oai(path = "/config", method = "put")]
    async fn update_config(&self, pool: Data<&DbPool>, config: Json<UpdateConfig>) -> Result<()> {
        sqlx::query("INSERT OR REPLACE INTO config (scan_path, thumb_path) VALUES ( $1, $2 )")
            .bind(config.0.scan_path)
            .bind(config.0.thumb_path)
            .execute(pool.0)
            .await
            .map_err(InternalServerError)?;
        Ok(())
    }
}
