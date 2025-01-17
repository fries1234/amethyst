use crate::{
    builder::pacman::PacmanUninstallBuilder,
    internal::{dependencies::DependencyInformation, error::AppResult, structs::Options},
    prompt,
};

pub struct MakeDependencyRemoval {
    pub options: Options,
    pub dependencies: Vec<DependencyInformation>,
}

impl MakeDependencyRemoval {
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn remove_make_deps(self) -> AppResult<()> {
        let make_depends = self
            .dependencies
            .iter()
            .flat_map(DependencyInformation::make_depends)
            .collect::<Vec<_>>();
        if !make_depends.is_empty()
            && !self.options.noconfirm
            && prompt!(default yes, "Do you want to remove the installed make dependencies?")
        {
            PacmanUninstallBuilder::default()
                .packages(make_depends)
                .no_confirm(true)
                .uninstall()
                .await?;
        }

        tracing::info!("Done!");

        Ok(())
    }
}
