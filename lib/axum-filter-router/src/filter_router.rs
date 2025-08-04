use proc_macro2::TokenStream;
use quote::quote;

use crate::filter::Filter;
use crate::router_with_state::RouterWithState;

#[derive(Debug, Default)]
struct Dependency {
    method: bool,
    scheme: bool,
    host: bool,
    port: bool,
    path: bool,
    version: bool,
    query: bool,
    authority: bool,
    raw_query: bool,
    uri: bool,
    header: bool,
    scheme_header: bool,
    host_header: bool,
    cookie: bool,
}

pub(super) fn expand(item: RouterWithState) -> syn::Result<TokenStream> {
    let mut dependency = Dependency::default();
    for router in &item.routers {
        match router.filter {
            Filter::Method(_) => {
                dependency.method = true;
            }
            Filter::Scheme(_) => {
                dependency.scheme = true;
            }
            Filter::Host(_) => {
                dependency.host = true;
            }
            Filter::Port(_) => {
                dependency.port = true;
            }
            Filter::Path(_) => {
                dependency.path = true;
            }
            Filter::Version(_) => {
                dependency.version = true;
            }
            Filter::Query(..) => {
                dependency.query = true;
            }
            Filter::Authority(_) => {
                dependency.authority = true;
            }
            Filter::RawQuery(_) => {
                dependency.raw_query = true;
            }
            Filter::Uri(_) => {
                dependency.uri = true;
            }
            Filter::Header(..) => {
                dependency.header = true;
            }
            Filter::SchemeHeader(_) => {
                dependency.scheme_header = true;
            }
            Filter::HostHeader(_) => {
                dependency.host_header = true;
            }
            Filter::Cookie(..) => {
                dependency.cookie = true;
            }
            _ => {}
        }
    }

    let mut args = vec![];
    let mut pre_body = vec![];
    if dependency.method {
        args.push(quote!(method: ::axum::http::Method));
        pre_body.push(quote!(let method = method.as_str();));
    }
    if dependency.scheme
        || dependency.host
        || dependency.port
        || dependency.path
        || dependency.version
        || dependency.query
        || dependency.authority
        || dependency.raw_query
        || dependency.uri
    {
        args.push(quote!(uri: ::axum::http::Uri));
    }
    if dependency.scheme {
        pre_body.push(quote!(let scheme = uri.scheme_str().unwrap_or_default();));
    }
    if dependency.host {
        pre_body.push(quote!(let host = uri.host().unwrap_or_default();));
    }
    if dependency.port {
        pre_body
            .push(quote!(let port = uri.port().map(|port| port.to_string()).unwrap_or_default();));
    }
    if dependency.path {
        pre_body.push(quote!(let path = uri.path();));
    }
    if dependency.version {
        args.push(quote!(version: ::axum::http::Version));
        pre_body.push(quote!(let version = format!("{version:?}");));
    }
    if dependency.query {
        args.push(quote!(
            ::axum_extra::extract::Query(query): ::axum_extra::extract::Query<::std::collections::HashMap<
                ::std::string::String, ::std::collections::HashSet<::std::string::String>
            >>
        ));
    }
    if dependency.authority {
        pre_body.push(quote!(
            let authority = uri
                .authority()
                .map(|authority| authority.as_str())
                .unwrap_or_default();
        ));
    }
    if dependency.raw_query {
        pre_body.push(quote!(let raw_query = uri.query().unwrap_or_default();));
    }
    if dependency.uri {
        pre_body.push(quote!(let uri = uri.to_string();));
    }
    if dependency.header {
        args.push(quote!(headers: ::axum::http::HeaderMap));
    }
    if dependency.scheme_header {
        args.push(
            quote!(::axum_extra::extract::Scheme(scheme_header): ::axum_extra::extract::Scheme),
        );
    }
    if dependency.host_header {
        args.push(quote!(::axum_extra::extract::Host(host_header): ::axum_extra::extract::Host));
    }
    if dependency.cookie {
        args.push(quote!(cookie: ::axum_extra::extract::CookieJar));
    }

    let state = item.state;
    args.push(quote!(::axum::extract::State(state): ::axum::extract::State<#state>));
    args.push(quote!(request: ::axum::extract::Request));
    let routers_iter = item.routers.into_iter();

    Ok(quote! {
        async |#(#args),*| -> ::axum::response::Response {
            #(#pre_body)*
            #(#routers_iter)*
        }
    })
}
