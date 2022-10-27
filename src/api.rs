use poem_openapi::OpenApi;

pub struct Api;

#[OpenApi]
impl Api {
    // #[oai(path = "/login", method = "post")]
    // async fn login(&self, pool: Data<&DbPool>, data: Json<VoLogin>) -> Result<Json<Token>> {
    //     let user = SYS_USER_SERVICE.login(&pool, data.0).await?;
    //     Ok(Json(gen_user_token(user)))
    // }
    // 扫描
    // 查询列表 -q 设备, 时间, 类型
    // 缩略图
}