use debian_packaging::repository::{builder::{RepositoryBuilder, NO_PROGRESS_CB, NO_SIGNING_KEY}, filesystem::{FilesystemRepositoryReader, FilesystemRepositoryWriter}};

use super::{configuration::Configuration, error::Result, Repository};
//pub fn create_repositories() -> Result<()> {

const distribution_path: &str = "your_distribution_path"; // Replace "your_distribution_path" with the actual path

pub async fn publicate_repositories(repos: &Vec<Repository>, config: &Configuration) -> Result<()> {
   for repo in repos {
       let repo_builder = RepositoryBuilder::new_recommended(
           repo.architectures.iter(),
           repo.components.iter(),
           &repo.suite,
           &repo.codename,
       );
       let writer = FilesystemRepositoryWriter::new("/tmp/test");
       let resolver = FilesystemRepositoryReader::new("/tmp/test");
       repo_builder.publish(&writer, &resolver, distribution_path, 1, &NO_PROGRESS_CB, NO_SIGNING_KEY).await;
       println!("{:?}", repo_builder);
   }
   Ok(())
}