extern crate wapc_guest as guest;

use anyhow::{anyhow, Result};

use k8s_openapi::api::core::v1::{Namespace, Service};
use k8s_openapi::api::networking::v1::Ingress;
use k8s_openapi::List;

/// A `ClusterContext` allows a waPC guest policy to retrieve cluster
/// contextual information from a Kubernetes cluster.
///
/// Right now a set of well known resources is hardcoded, but the idea
/// is to generalize this so the SDK can support any kind of
/// Kubernetes resource and custom resource definition.
pub struct ClusterContext {}

#[derive(PartialEq)]
pub enum NamespaceFilter {
    AllNamespaces,
    Namespace(String),
}

impl ClusterContext {
    /// Return the list of `Ingress` resources that exist in the
    /// cluster.
    pub fn ingresses(namespace: NamespaceFilter) -> Result<Vec<Ingress>> {
        // TODO (ereslibre): use macros to remove duplication and then
        // generalize
        Ok(
            guest::host_call("kubernetes", "ingresses", "list", &Vec::new())
                .map_err(|err| anyhow!("failed to call ingresses binding: {}", err))
                .and_then(|ingresses| {
                    Ok(
                        serde_json::from_str::<List<Ingress>>(std::str::from_utf8(&ingresses)?)
                            .map_err(|err| anyhow!("failed to unmarshal ingress list: {}", err))?
                            .items,
                    )
                })?
                .iter()
                .filter_map(|ingress| match &namespace {
                    NamespaceFilter::AllNamespaces => Some(ingress.clone()),
                    NamespaceFilter::Namespace(namespace_filter) => {
                        if let Some(ingress_namespace) = &ingress.metadata.namespace {
                            if namespace_filter == ingress_namespace {
                                Some(ingress.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
                .collect(),
        )
    }

    /// Return the list of `Namespace` resources that exist in the
    /// cluster.
    pub fn namespaces() -> Result<Vec<Namespace>> {
        // TODO (ereslibre): use macros to remove duplication and then
        // generalize
        guest::host_call("kubernetes", "namespaces", "list", &Vec::new())
            .map_err(|err| anyhow!("failed to call namespaces binding: {}", err))
            .and_then(|namespaces| {
                Ok(
                    serde_json::from_str::<List<Namespace>>(std::str::from_utf8(&namespaces)?)
                        .map_err(|err| anyhow!("failed to unmarshal namespace list: {}", err))?
                        .items,
                )
            })
    }

    /// Return the list of `Service` resources that exist in the
    /// cluster.
    pub fn services(namespace: NamespaceFilter) -> Result<Vec<Service>> {
        // TODO (ereslibre): use macros to remove duplication and then
        // generalize
        Ok(
            guest::host_call("kubernetes", "services", "list", &Vec::new())
                .map_err(|err| anyhow!("failed to call services binding: {}", err))
                .and_then(|services| {
                    Ok(
                        serde_json::from_str::<List<Service>>(std::str::from_utf8(&services)?)
                            .map_err(|err| anyhow!("failed to unmarshal service list: {}", err))?
                            .items,
                    )
                })?
                .iter()
                .filter_map(|service| match &namespace {
                    NamespaceFilter::AllNamespaces => Some(service.clone()),
                    NamespaceFilter::Namespace(namespace_filter) => {
                        if let Some(service_namespace) = &service.metadata.namespace {
                            if namespace_filter == service_namespace {
                                Some(service.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                })
                .collect(),
        )
    }
}

impl ClusterContext {
    /// Return a specific ingress object with a given name and a
    /// namespace filter. If the namespace filter allows for more than
    /// one namespace, the ingress resource found that matches one of
    /// the namespaces and the given name will be returned.
    pub fn ingress(namespace: NamespaceFilter, name: &str) -> Result<Option<Ingress>> {
        // TODO (ereslibre): use macros to remove duplication and then
        // generalize
        Ok(Self::ingresses(namespace)?
            .into_iter()
            .find(|ingress| ingress.metadata.name == Some(name.to_string())))
    }

    // Return a specific namespace with a given name.
    pub fn namespace(name: &str) -> Result<Option<Namespace>> {
        // TODO (ereslibre): use macros to remove duplication and then
        // generalize
        Ok(Self::namespaces()?
            .into_iter()
            .find(|namespace| namespace.metadata.name == Some(name.to_string())))
    }

    /// Return a specific service object with a given name and a
    /// namespace filter. If the namespace filter allows for more than
    /// one namespace, the service resource found that matches one of
    /// the namespaces and the given name will be returned.
    pub fn service(namespace: NamespaceFilter, name: &str) -> Result<Option<Service>> {
        // TODO (ereslibre): use macros to remove duplication and then
        // generalize
        Ok(Self::services(namespace)?
            .into_iter()
            .find(|service| service.metadata.name == Some(name.to_string())))
    }
}
