mod private
{
  use crate::*;

  use std::*;
  // use std::collections::BTreeMap;
  // use cargo_metadata::camino::{ Utf8Path, Utf8PathBuf };
  // use serde::Deserialize;
  // use serde_json::Value;
  use wtools::error::
  {
    thiserror,
    Result
  };
  // use path::AbsolutePath;

  // aaa : for Bohdan : for Petro : what crate_dir is?
  // aaa : `crate_dir` is path to a folder with `Cargo.toml` file(not a path to `Cargo.toml` file)

  /// Stores information about the current workspace.
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    // aaa : for Bohdan : for Petro : describe all fields
    // aaa : for Bohdan : for Petro : is Option required?
    // aaa : The `Option` removed
    /// Metadata of the workspace, containing detailed information about the packages, dependencies, and other workspace-related data.
    pub metadata : cargo_metadata::Metadata,

    /// The directory containing the manifest file (`Cargo.toml`) of the workspace.
    pub crate_dir : CrateDir,
  }

  /// Represents errors related to workspace operations.
  #[ derive( Debug, thiserror::Error ) ]
  pub enum WorkspaceInitError
  {
    /// Something went wrong with path to a workspace.
    #[ error( "Path error. Details: {0}" ) ]
    Path( #[ from ] PathError ),
    /// Something went wrong with the workspace' data
    #[ error( "Can not load workspace data. Details: {0}" ) ]
    Metadata( #[ from ] cargo_metadata::Error ),
  }

  impl Workspace
  {

    // aaa : typed errors
    // aaa : done
    /// Load data from current directory
    pub fn from_current_path() -> Result< Self, WorkspaceInitError >
    {
      let current_path = AbsolutePath::try_from( env::current_dir().unwrap_or_default() ).map_err( PathError::Io )?;
      let metadata = cargo_metadata::MetadataCommand::new()
      .no_deps()
      .exec()?;
      Ok( Self
      {
        metadata,
        crate_dir : CrateDir::try_from( current_path )?,
      })
    }

    // qqq : typed errors
    /// Load data from current directory
    pub fn with_crate_dir( crate_dir : CrateDir ) -> Result< Self, WorkspaceInitError >
    {
      Ok
      (
        Self
        {
          metadata : cargo_metadata::MetadataCommand::new()
          .current_dir( crate_dir.as_ref() )
          .no_deps()
          .exec()?,
          crate_dir,
        }
      )
    }
  }

  impl From< cargo_metadata::Metadata > for Workspace
  {
    fn from( metadata : cargo_metadata::Metadata ) -> Self
    {
      // SAFE: `workspace_root` is a path to a`Cargo.toml` file, therefor the parent is the directory
      let path = metadata.workspace_root.as_std_path().parent().unwrap().to_path_buf();
      let path = AbsolutePath::try_from( path ).unwrap();
      Self
      {
        metadata,
        crate_dir : CrateDir::try_from( path ).unwrap(),
      }
    }
  }

  impl Workspace
  {

    // aaa : replace all Vec by Iterators over refs
    // aaa : understood

    /// Returns list of all packages
    pub fn packages< 'a >( &'a self )
    ->
    core::iter::Map
    <
      slice::Iter< 'a, cargo_metadata::Package >,
      impl Fn( &'a cargo_metadata::Package ) -> WorkspacePackageRef< 'a > + Clone,
    >
    {
      self.metadata.packages.iter().map( WorkspacePackageRef::from )
    }

    // /// Returns list of all packages
    // pub fn packages( &self ) -> Result< Vec< WorkspacePackageRef< '_ > >, WorkspaceError >
    // {
    //   self
    //   .metadata
    //   .as_ref()
    //   .ok_or_else( || WorkspaceError::MetadataError )
    //   .map( | metadata | metadata.packages.clone() )
    //   .map( | p | p.into_iter().map( WorkspacePackageRef::from ).collect() )
    // }

    /// Returns the path to workspace root
    pub fn workspace_root( &self ) -> &std::path::Path
    {
      self.metadata.workspace_root.as_std_path()
    }

    /// Returns the path to target directory
    pub fn target_directory( &self ) -> &std::path::Path
    {
      self.metadata.target_directory.as_std_path()
    }

    // qqq : bad : for Petro : that should not be here as it's very task specific
    /// Return discord url
    pub fn discord_url( &self ) -> Option< String >
    {
      self.metadata.workspace_metadata[ "discord_url" ].as_str().map( | url | url.to_string() )
    }

    /// Return the master branch
    pub fn master_branch( &self ) -> Option< String >
    {
      self.metadata.workspace_metadata.get( "master_branch" ).and_then( | b | b.as_str() ).map( | b | b.to_string() )
    }

    /// Return the repository url
    pub fn repository_url( &self ) -> Option< String >
    {
      self.metadata.workspace_metadata.get( "repo_url" ).and_then( | b | b.as_str() ).map( | b | b.to_string() )
    }

    /// Return the workspace_name
    pub fn workspace_name( &self ) -> Option< String >
    {
      self.metadata.workspace_metadata.get( "workspace_name" ).and_then( | b | b.as_str() ).map( | b | b.to_string() )
    }

    /// Find a package by its manifest file path
    pub fn package_find_by_manifest< 'a, P >( &'a self, manifest_file : P ) -> Option< WorkspacePackageRef< 'a > >
    where
      P : AsRef< std::path::Path >,
    {
      self
      .packages()
      // .iter()
      // .find( | &p | p.manifest_file().as_std_path() == manifest_file.as_ref() )
      .find( | &p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref() )
      // .cloned()
    }

    // xxx : aaa : for Bohdan : should not be here entity/workspace-graph.rs
    // aaa : moved out
  }
}

//

crate::mod_interface!
{
  exposed use WorkspaceInitError;
  exposed use Workspace;
}
