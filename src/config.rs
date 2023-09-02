pub mod db {
    pub trait SledConfigExt {
        fn from_env() -> Self;
    }

    impl SledConfigExt for sled::Config {
        fn from_env() -> Self {
            sled::Config::new()
                .cache_capacity(cache_capacity())
                .path(path())
                .flush_every_ms(flush_every_ms())
        }
    }
    #[inline]
    fn cache_capacity() -> u64 {
        std::env::var("SCHEDULERS_DB_CACHE_CAPACITY")
            // default is 100MiB
            .unwrap_or("104857600".to_string())
            .parse()
            .unwrap_or_else(|_| panic!("Invalid SCHEDULERS_DB_CACHE_CAPACITY"))
    }

    #[inline]
    fn path() -> String {
        std::env::var("SCHEDULERS_DB_PATH").unwrap_or("data".to_string())
    }

    #[inline]
    fn flush_every_ms() -> Option<u64> {
        if let Ok(flush_every_ms) = std::env::var("SCHEDULERS_DB_FLUSH_EVERY_MS") {
            if flush_every_ms.is_empty() {
                return None;
            } else {
                let val: u64 = flush_every_ms
                    .parse()
                    .expect("Invalid SCHEDULERS_DB_FLUSH_EVERY_MS, should be a number");
                Some(val)
            }
        } else {
            None
        }
    }
}

pub mod web {
    use actix_http::body::MessageBody;
    use actix_http::{Request, Response};
    use actix_service::{IntoServiceFactory, ServiceFactory};
    use actix_web::dev::AppConfig;
    use actix_web::HttpServer;
    use std::env;
    use std::net::{Ipv4Addr, ToSocketAddrs};
    use tracing::log;

    pub trait HttpServerExt: Sized {
        fn set_workers_from_env(self) -> Self;
        fn set_binding_from_env(self) -> std::io::Result<Self>;

        fn set_hostname_from_env(self) -> Self;
    }

    impl<F, I, S, B> HttpServerExt for HttpServer<F, I, S, B>
    where
        F: Fn() -> I + Send + Clone + 'static,
        I: IntoServiceFactory<S, Request>,
        S: ServiceFactory<Request, Config = AppConfig> + 'static,
        S::Error: Into<actix_web::error::Error>,
        S::InitError: core::fmt::Debug,
        S::Response: Into<Response<B>>,
        B: MessageBody + 'static,
    {
        fn set_workers_from_env(self) -> Self {
            let w = env::var("SCHEDULERS_HTTP_WORKERS")
                .unwrap_or_else(|_| "4".to_string())
                .parse()
                .expect("Invalid number for WORKERS");
            self.workers(w)
        }

        fn set_binding_from_env(self) -> std::io::Result<Self> {
            let socks: Vec<String> = env::var("SCHEDULERS_HTTP_LISTEN")
                .unwrap_or_else(|_| format!("{}:{}", Ipv4Addr::UNSPECIFIED, 8080))
                .split(',')
                .map(|s| s.trim().into())
                .collect();
            let mut server = self;
            for sock in socks {
                server = server.bind(sock.to_socket_addrs()?.next().unwrap())?;
            }
            Ok(server)
        }

        fn set_hostname_from_env(self) -> Self {
            if let Ok(hostname) = env::var("SCHEDULERS_HTTP_HOSTNAME") {
                self.server_hostname(hostname)
            } else {
                log::info!("SCHEDULERS_HTTP_HOSTNAME not set");
                self
            }
        }
    }
}
