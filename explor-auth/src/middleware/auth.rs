use actix_web::{
    Error, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;
use serde_json::json;
use std::future::{Ready, ready};

use crate::model::AppState;

/// Middleware d'authentification par cookie
///
/// Vérifie la présence et la validité du cookie de session à chaque requête.
/// Si le cookie est valide, la requête continue, sinon retourne 401 Unauthorized.
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Récupérer le cookie de session
        let session_cookie = req.cookie("session_id");

        if let Some(cookie) = session_cookie {
            let session_id = cookie.value().to_string();

            // Récupérer l'AppState pour vérifier la session via le service
            if let Some(app_state) = req.app_data::<actix_web::web::Data<AppState>>() {
                // Utiliser le service d'authentification pour vérifier la session
                if app_state.auth_service.verify_session(&session_id).is_ok() {
                    // Session valide, continuer la requête
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res.map_into_left_body())
                    });
                }
            }
        }

        // Pas de cookie ou session invalide -> 401 Unauthorized
        let (req, _pl) = req.into_parts();
        let response: HttpResponse = HttpResponse::Unauthorized().json(json!({
            "metadata": {
                "status": "error",
                "message": "Unauthorized - Valid session required"
            }
        }));

        Box::pin(async move {
            Ok(ServiceResponse::new(
                req,
                response.map_into_right_body::<B>(),
            ))
        })
    }
}
