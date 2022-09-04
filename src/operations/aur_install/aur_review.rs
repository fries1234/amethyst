use crate::{
    builder::pager::PagerBuilder,
    internal::{
        dependencies::DependencyInformation,
        error::{AppError, AppResult},
        structs::Options,
        utils::get_cache_dir,
    },
    multi_select, prompt,
};

use super::{repo_dependency_installation::RepoDependencyInstallation, BuildContext};

pub struct AurReview {
    pub options: Options,
    pub packages: Vec<String>,
    pub dependencies: Vec<DependencyInformation>,
    pub contexts: Vec<BuildContext>,
}

impl AurReview {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn review_pkgbuild(self) -> AppResult<RepoDependencyInstallation> {
        if !self.options.noconfirm {
            let to_review = multi_select!(&self.packages, "Select packages to review");

            for pkg in to_review.into_iter().filter_map(|i| self.packages.get(i)) {
                let pkgbuild_path = get_cache_dir().join(pkg).join("PKGBUILD");
                PagerBuilder::default().path(pkgbuild_path).open().await?;
            }
            if !prompt!(default yes, "Do you still want to install those packages?") {
                return Err(AppError::UserCancellation);
            }
        }
        Ok(RepoDependencyInstallation {
            options: self.options,
            dependencies: self.dependencies,
            contexts: self.contexts,
        })
    }
}
