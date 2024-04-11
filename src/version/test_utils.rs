use crate::VersionTag;

use super::{PreRelease, Semantic};

#[allow(dead_code)]
pub(crate) fn gen_current_version(
    version_prefix: &str,
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<PreRelease>,
    build_meta_data: Option<String>,
) -> VersionTag {
    VersionTag {
        refs: "refs/tags/".to_string(),
        tag_prefix: "".to_string(),
        version_prefix: version_prefix.to_string(),
        semantic_version: Semantic {
            major,
            minor,
            patch,
            pre_release,
            build_meta_data,
        },
    }
}
