/// Internal namespace.
mod private
{
  use crate::*;
  use package::{ DependenciesOptions, DependenciesSort };
  use tools::path;
  use std::
  {
    path::PathBuf,
    collections::HashSet,
  };
  use core::fmt::Formatter;
  use workspace::Workspace;
  use package::CrateId;
  use wtools::error::for_app::Error;

  #[ derive( Debug, Default, Clone ) ]
  pub struct PublishReport
  {
    workspace_root_dir : PathBuf,
    packages : Vec<( PathBuf,  package::PublishReport )>
  }

  impl std::fmt::Display for PublishReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      if self.packages.is_empty()
      {
        f.write_fmt( format_args!( "Nothing to publish" ) )?;
        return Ok( () );
      }

      for ( path, report ) in &self.packages
      {
        let report = report.to_string().replace("\n", "\n\t");
        // qqq: remove unwrap
        f.write_fmt( format_args!( "Publishing crate by `{}` path\n\t{report}\n", path.strip_prefix( &self.workspace_root_dir ).unwrap().display() ) )?;
      }

      Ok( () )
    }
  }

  ///
  /// Publish packages.
  ///

  pub fn publish( patterns : Vec< String >, dry : bool ) -> Result< PublishReport, ( PublishReport, Error ) >
  {
    let mut report = PublishReport::default();

    let mut paths = HashSet::new();
    // find all packages by specified folders
    for pattern in &patterns
    {
      let current_path = path::canonicalize( pattern ).map_err( | e | ( report.clone(), e.into() ) )?;
      // let current_paths = files::find( current_path, &[ "Cargo.toml" ] );
      paths.extend( Some( current_path ) );
    }

    let mut metadata = if paths.is_empty()
    {
      Workspace::default()
    }
    else
    {
      // FIX: patterns can point to different workspaces. Current solution take first random path from list
      Workspace::with_manifest_path( paths.iter().next().unwrap() )
    };
    report.workspace_root_dir = metadata.workspace_root().to_path_buf();

    let packages_to_publish : Vec< _ >= metadata.load().packages_get().iter().filter( | &package | paths.contains( package.manifest_path.as_std_path().parent().unwrap() ) ).cloned().collect();
    let mut queue = vec![];
    for package in &packages_to_publish
    {
      let local_deps_args = DependenciesOptions
      {
        recursive: true,
        sort: DependenciesSort::Topological,
        ..Default::default()
      };
      let deps = package::dependencies( &mut metadata, package.manifest_path.as_std_path(), local_deps_args )
      .map_err( | e | ( report.clone(), e.into() ) )?;

      for dep in deps
      {
        if !queue.contains( &dep )
        {
          queue.push( dep );
        }
      }
      let crate_id = CrateId::from( package );
      if !queue.contains( &crate_id )
      {
        queue.push( crate_id );
      }
    }

    for path in queue.into_iter().filter_map( | id | id.path )
    {
      let current_report = package::publish_single( &mut metadata, &path, dry )
      .map_err
      (
        | ( current_report, e ) |
        {
          report.packages.push(( path.clone(), current_report.clone() ));
          ( report.clone(), e.context( "Publish list of packages" ).into() )
        }
      )?;
      report.packages.push(( path, current_report ));
    }

    Ok( report )
  }
}

//

crate::mod_interface!
{
  /// Publish package.
  orphan use publish;
}
