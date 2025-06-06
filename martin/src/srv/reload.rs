use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use actix_web::{HttpResponse, Result as ActixResult, route};
use actix_web::web::Data;

use crate::config::{Config, ReloadMode};
use crate::source::{TileSources};
use crate::srv::server::{map_internal_error, Catalog};
use crate::utils::OptMainCache;
#[cfg(feature = "sprites")]
use crate::sprites::SpriteSources;
#[cfg(feature = "fonts")]
use crate::fonts::FontSources;
#[cfg(feature = "styles")]
use crate::styles::StyleSources;

#[route("/reload", method = "GET", method = "POST")]
pub async fn reload_sources(
    config: Data<Arc<Mutex<Config>>>,
    tiles: Data<TileSources>,
    cache: Data<OptMainCache>,
    catalog: Data<Arc<RwLock<Catalog>>>,
    #[cfg(feature = "sprites")] sprites: Data<SpriteSources>,
    #[cfg(feature = "fonts")] fonts: Data<FontSources>,
    #[cfg(feature = "styles")] styles: Data<StyleSources>,
) -> ActixResult<HttpResponse> {
    let mut cfg = config.lock().await;
    let new_sources = cfg
        .reload_tile_sources(cache.get_ref().clone())
        .await
        .map_err(map_internal_error)?;

    match cfg.reload_mode {
        ReloadMode::Incremental => tiles.sync_incremental(new_sources),
        _ => tiles.replace(new_sources),
    }

    let mut cat = catalog.write().await;
    *cat = Catalog {
        tiles: tiles.get_catalog(),
        #[cfg(feature = "sprites")]
        sprites: sprites.get_catalog().map_err(map_internal_error)?,
        #[cfg(feature = "fonts")]
        fonts: fonts.get_catalog(),
        #[cfg(feature = "styles")]
        styles: styles.get_catalog(),
    };
    drop(cat);
    Ok(HttpResponse::Ok().finish())
}
