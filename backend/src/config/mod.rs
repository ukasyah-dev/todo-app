use envconfig::Envconfig;

#[derive(Clone, Envconfig)]
pub struct Config {
    #[envconfig(from = "HTTP_ADDRESS", default = "0.0.0.0:3000")]
    pub http_address: String,
}
