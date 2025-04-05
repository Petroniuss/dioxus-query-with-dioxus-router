use std::time::Duration;

use crate::Route;
use dioxus::prelude::*;
use dioxus_query::*;
use dioxus_query::prelude::*;
use tokio::time::sleep;
use tracing::*;

const BLOG_CSS: Asset = asset!("/assets/styling/blog.css");

// -- example from dixous-query
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum QueryKey {
    User(usize),
}

#[derive(Debug)]
pub enum QueryError {
    UserNotFound(usize),
    Unknown
}

#[derive(PartialEq, Debug)]
pub enum QueryValue {
    UserName(String),
}

// --  fetching user occurs multiple times for the same id!
async fn fetch_user(keys: Vec<QueryKey>) -> QueryResult<QueryValue, QueryError> {
    if let Some(QueryKey::User(id)) = keys.first() {
        tracing::info!("Fetching user {id}");
        match id {
            0 => QueryResult::Ok(QueryValue::UserName("Marc".to_string())),
            1 => QueryResult::Ok(QueryValue::UserName("Patryk".to_string())),
            _ => QueryResult::Err(QueryError::UserNotFound(*id)),
        }
    } else {
        QueryResult::Err(QueryError::Unknown)
    }
}

// -- example from dixous jumpstart
#[component]
pub fn Blog(id: i32) -> Element {
    tracing::info!("Rendering app!");
    let value = use_get_query([QueryKey::User(id as usize)], fetch_user);

    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}

        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { 
                "{value.result().value():?}"
            }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}
