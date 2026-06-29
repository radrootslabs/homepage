use leptos_router::AsPath;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RouteKey {
    Home,
    Contact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalisedRoutePath {
    HomeEn,
    HomeEs,
    ContactEn,
    ContactEs,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalisedRoute {
    pub key: RouteKey,
    pub locale: &'static str,
    pub path: LocalisedRoutePath,
}

pub const LOCALISED_ROUTES: &[LocalisedRoute] = &[
    LocalisedRoute {
        key: RouteKey::Home,
        locale: "en",
        path: LocalisedRoutePath::HomeEn,
    },
    LocalisedRoute {
        key: RouteKey::Home,
        locale: "es",
        path: LocalisedRoutePath::HomeEs,
    },
    LocalisedRoute {
        key: RouteKey::Contact,
        locale: "en",
        path: LocalisedRoutePath::ContactEn,
    },
    LocalisedRoute {
        key: RouteKey::Contact,
        locale: "es",
        path: LocalisedRoutePath::ContactEs,
    },
];

impl AsPath for LocalisedRoutePath {
    fn as_path(&self) -> &'static str {
        match self {
            Self::HomeEn => "/",
            Self::HomeEs => "/es",
            Self::ContactEn => "/contact",
            Self::ContactEs => "/contacto",
        }
    }
}

pub fn localised_route(key: RouteKey, locale: &str) -> LocalisedRoute {
    LOCALISED_ROUTES
        .iter()
        .copied()
        .find(|route| route.key == key && route.locale == locale)
        .expect("localised route should be declared")
}

pub fn localised_path(key: RouteKey, locale: &str) -> &'static str {
    localised_route(key, locale).path.as_path()
}

#[cfg(test)]
mod tests {
    use super::{LOCALISED_ROUTES, localised_route};
    use crate::i18n;
    use leptos_router::AsPath;
    use std::collections::BTreeSet;

    #[test]
    fn declares_paths_for_supported_locales() {
        for locale in i18n::SUPPORTED_LOCALES {
            for route in LOCALISED_ROUTES {
                localised_route(route.key, locale);
            }
        }
    }

    #[test]
    fn declares_unique_paths() {
        let mut paths = BTreeSet::new();

        for route in LOCALISED_ROUTES {
            assert!(paths.insert(route.path.as_path()));
        }
    }
}
