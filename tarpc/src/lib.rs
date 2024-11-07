#[tarpc::service]
pub trait Hello {
    async fn hello(name: String) -> String;
}
