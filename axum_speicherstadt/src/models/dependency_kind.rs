use serde::Deserialize;

#[derive(Copy, Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum DependencyKind {
    #[default]
    Normal,
    Dev,
    Build,
}
