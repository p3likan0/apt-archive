use debian_packaging::repository::{builder::{RepositoryBuilder, NO_PROGRESS_CB, NO_SIGNING_KEY}, filesystem::{FilesystemRepositoryReader, FilesystemRepositoryWriter}};

use super::{configuration::Configuration, error::Result, Repository};


pub async fn publicate_repositories(repos: &Vec<Repository>, config: &Configuration) -> Result<()> {
   for repo in repos{
      let distribution_path = format!("dists/{}", &repo.name);
      let writer = FilesystemRepositoryWriter::new(&config.repo_root_path);
      let resolver = FilesystemRepositoryReader::new(&config.repo_root_path);
      let rp: RepositoryBuilder = RepositoryBuilder::new_recommended(repo.architectures.iter(), repo.components.iter(), repo.suite.clone(), repo.codename.clone());
      let _ = tokio::task::spawn_blocking(move || {
         tokio::runtime::Handle::current().block_on(async {
            let _ = rp.publish(&writer, &resolver, &distribution_path, 1, &NO_PROGRESS_CB, NO_SIGNING_KEY).await?;
            println!("{:?}", rp);
            Ok::<(), super::error::RepoError>(())
         })
      }).await?;
   }
   Ok(())
}