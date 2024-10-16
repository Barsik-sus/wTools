/// Internal namespace.
mod private
{
  use crate::*;
  use std::
  {
    fmt::{ Formatter, Write },
    path::PathBuf,
    collections::HashSet,
  };
  use std::collections::HashMap;
  use petgraph::
  {
    prelude::*,
    algo::toposort,
    visit::Topo,
  };
  use std::str::FromStr;
  use packages::FilterMapOptions;
  use wtools::error::
  {
    for_app::{ Error, Context },
    err
  };
  use cargo_metadata::
  {
    Dependency,
    DependencyKind,
    Package
  };
  use petgraph::prelude::{ Dfs, EdgeRef };
  use former::Former;

  use workspace::Workspace;
  use path::AbsolutePath;

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFormat
  {
    /// Tree like format.
    #[ default ]
    Tree,
    /// Topologically sorted list.
    Topological,
  }

  impl FromStr for ListFormat
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "tree" => ListFormat::Tree,
        "toposort" => ListFormat::Topological,
        e => return Err( err!( "Unknown format '{}'. Available values: [tree, toposort]", e ))
      };

      Ok( value )
    }
  }

  /// Enum representing the different dependency categories.
  ///
  /// These categories include:
  /// - `Primary`: This category represents primary dependencies.
  /// - `Dev`: This category represents development dependencies.
  /// - `Build`: This category represents build-time dependencies.
  #[ derive( Debug, Copy, Clone, Hash, Eq, PartialEq ) ]
  pub enum DependencyCategory
  {
    /// Represents the primary dependencies, i.e., libraries or packages that
    /// are required for your code to run. These are typically listed in your
    /// `Cargo.toml`'s `[dependencies]` section.
    Primary,
    /// Represents the development dependencies. These are used for compiling
    /// tests, examples, or benchmarking code. They are not used when compiling
    /// the normal application or library. These are typically listed in your
    /// `Cargo.toml`'s `[dev-dependencies]` section.
    Dev,
    /// Represents build-time dependencies. These are used only to compile
    /// build scripts (`build.rs`) but not for the package code itself. These
    /// are typically listed in your `Cargo.toml`'s `[build-dependencies]` section.
    Build,
  }

  /// Enum representing the source of a dependency.
  ///
  /// This enum has the following values:
  /// * `Local` - Represents a dependency located locally.
  /// * `Remote` - Represents a dependency fetched from a remote source.
  #[ derive( Debug, Copy, Clone, Hash, Eq, PartialEq ) ]
  pub enum DependencySource
  {
    /// Represents a dependency that is located on the local file system.
    Local,
    /// Represents a dependency that is to be fetched from a remote source.
    Remote,
  }

  /// Args for `list` endpoint.
  #[ derive( Debug, Default, Copy, Clone ) ]
  pub enum ListFilter
  {
    /// With all packages.
    #[ default ]
    Nothing,
    /// With local only packages.
    Local,
  }

  impl FromStr for ListFilter
  {
    type Err = Error;

    fn from_str( s : &str ) -> Result< Self, Self::Err >
    {
      let value = match s
      {
        "nothing" => ListFilter::Nothing,
        "local" => ListFilter::Local,
        e => return Err( err!( "Unknown filter '{}'. Available values: [nothing, local]", e ) )
      };

      Ok( value )
    }
  }

  /// Additional information to include in a package report.
  #[ derive( Debug, Copy, Clone, Hash, Eq, PartialEq ) ]
  pub enum PackageAdditionalInfo
  {
    /// Include the version of the package, if possible.
    Version,
    /// Include the path to the package, if it exists.
    Path,
  }

  /// A struct representing the arguments for listing crates.
  ///
  /// This struct is used to pass the necessary arguments for listing crates. It includes the
  /// following fields:
  ///
  /// - `path_to_manifest`: A `CrateDir` representing the path to the manifest of the crates.
  /// - `format`: A `ListFormat` enum representing the desired format of the output.
  /// - `dependency_sources`: A `HashSet` of `DependencySource` representing the sources of the dependencies.
  #[ derive( Debug, Former ) ]
  pub struct ListArgs
  {
    path_to_manifest : CrateDir,
    format : ListFormat,
    info: HashSet< PackageAdditionalInfo >,
    dependency_sources: HashSet< DependencySource >,
    dependency_categories: HashSet< DependencyCategory >,
  }

  #[ derive( Debug, Clone ) ]
  pub struct ListNodeReport
  {
    pub name: String,
    pub version: Option< String >,
    pub path: Option< PathBuf >,
    pub normal_dependencies: Vec< ListNodeReport >,
    pub dev_dependencies: Vec< ListNodeReport >,
    pub build_dependencies: Vec< ListNodeReport >,
  }

  impl ListNodeReport
  {
    fn display_with_spacer( &self, spacer : &str, depth : usize ) -> Result< String, std::fmt::Error >
    {
      let mut f = String::new();

      write!( f, "{spacer}{}", self.name )?;
      if let Some( version ) = &self.version { write!( f, " {version}" )? }
      if let Some( path ) = &self.path { write!( f, " {}", path.display() )? }
      write!( f, "\n" )?;

      let spacer = format!( "{spacer}[{depth}] " );
      let depth = depth + 1;

      for dep in &self.normal_dependencies
      {
        write!( f, "{}", dep.display_with_spacer( &spacer, depth )? )?;
      }
      if !self.dev_dependencies.is_empty()
      {
        write!( f, "{spacer}[dev-dependencies]\n" )?;
        let spacer = format!( "{spacer}| " );
        for dep in &self.dev_dependencies
        {
          write!( f, "{}", dep.display_with_spacer( &spacer, depth )? )?;
        }
      }
      if !self.build_dependencies.is_empty()
      {
        write!( f, "{spacer}[build-dependencies]\n" )?;
        let spacer = format!( "{spacer}| " );
        for dep in &self.build_dependencies
        {
          write!( f, "{}", dep.display_with_spacer( &spacer, depth )? )?;
        }
      }

      Ok( f )
    }
  }

  impl std::fmt::Display for ListNodeReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      write!( f, "{}", self.display_with_spacer( "", 0 )? )?;

      Ok( () )
    }
  }

  /// Represents the different report formats for the `list` endpoint.
  #[ derive( Debug, Default, Clone ) ]
  pub enum ListReport
  {
    /// Represents a tree-like report format.
    Tree( Vec< ListNodeReport > ),
    /// Represents a standard list report format in topological order.
    List( Vec< String > ),
    /// Represents an empty report format.
    #[ default ]
    Empty,
  }

  impl std::fmt::Display for ListReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match self
      {
        Self::Tree( v ) => write!( f, "{}", v.iter().map( | l | l.to_string() ).collect::< Vec< _ > >().join( "\n" ) ),
        Self::List( v ) => write!( f, "{}", v.iter().enumerate().map( |( i, v )| format!( "[{i}] {v}" ) ).collect::< Vec< _ > >().join( "\n" ) ),
        Self::Empty => write!( f, "Nothing" ),
      }
    }
  }

  fn process_package_dependency
  (
    workspace : &Workspace,
    package : &Package,
    args : &ListArgs,
    dep_rep : &mut ListNodeReport,
    visited : &mut HashSet< String >
  )
  {
    for dependency in &package.dependencies
    {
      if dependency.path.is_some() && !args.dependency_sources.contains( &DependencySource::Local ) { continue; }
      if dependency.path.is_none() && !args.dependency_sources.contains( &DependencySource::Remote ) { continue; }
      let dep_id = format!( "{}+{}+{}", dependency.name, dependency.req, dependency.path.as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );

      let mut temp_vis = visited.clone();
      let dependency_rep = process_dependency( workspace, dependency, args, &mut temp_vis );

      match dependency.kind
      {
        DependencyKind::Normal if args.dependency_categories.contains( &DependencyCategory::Primary ) => dep_rep.normal_dependencies.push( dependency_rep ),
        DependencyKind::Development if args.dependency_categories.contains( &DependencyCategory::Dev ) => dep_rep.dev_dependencies.push( dependency_rep ),
        DependencyKind::Build if args.dependency_categories.contains( &DependencyCategory::Build ) => dep_rep.build_dependencies.push( dependency_rep ),
        _ => { visited.remove( &dep_id ); std::mem::swap( &mut temp_vis, visited ); }
      }

      *visited = std::mem::take( &mut temp_vis );
    }
  }

  fn process_dependency( workspace : &Workspace, dep: &Dependency, args : &ListArgs, visited : &mut HashSet< String > ) -> ListNodeReport
  {
    let mut dep_rep = ListNodeReport
    {
      name : dep.name.clone(),
      version : if args.info.contains( &PackageAdditionalInfo::Version ) { Some( dep.req.to_string() ) } else { None },
      path : if args.info.contains( &PackageAdditionalInfo::Path ) { dep.path.as_ref().map( | p | p.clone().into_std_path_buf() ) } else { None },
      normal_dependencies : vec![],
      dev_dependencies : vec![],
      build_dependencies : vec![],
    };

    let dep_id = format!( "{}+{}+{}", dep.name, dep.req, dep.path.as_ref().map( | p | p.join( "Cargo.toml" ) ).unwrap_or_default() );
    // if this is a cycle (we have visited this node before)
    if visited.contains( &dep_id )
    {
      dep_rep.name = format!( "{} (*)", dep_rep.name );

      return dep_rep;
    }

    // if we have not visited this node before, mark it as visited
    visited.insert( dep_id );
    if let Some( path ) = &dep.path
    {
      if let Some( package ) = workspace.package_find_by_manifest( path.as_std_path().join( "Cargo.toml" ) )
      {
        process_package_dependency( workspace, package, args, &mut dep_rep, visited );
      }
    }

    dep_rep
  }

  trait ErrWith< T, T1, E >
  {
    fn err_with( self, v : T ) -> std::result::Result< T1, ( T, E ) >;
  }

  impl< T, T1, E > ErrWith< T, T1, E > for Result< T1, E >
  {
    fn err_with( self, v : T ) -> Result< T1, ( T, E ) >
    {
      self.map_err( | e | ( v, e ) )
    }
  }

  /// Retrieve a list of packages based on the given arguments.
  ///
  /// # Arguments
  ///
  /// - `args`: ListArgs - The arguments for listing packages.
  ///
  /// # Returns
  ///
  /// - `Result<ListReport, (ListReport, Error)>` - A result containing the list report if successful,
  ///   or a tuple containing the list report and error if not successful.
  pub fn list( args : ListArgs ) -> Result< ListReport, ( ListReport, Error ) >
  {
    let mut report = ListReport::default();

    let manifest = manifest::open( args.path_to_manifest.absolute_path() ).context( "List of packages by specified manifest path" ).err_with( report.clone() )?;
    let metadata = Workspace::with_crate_dir( manifest.crate_dir() ).err_with( report.clone() )?;

    let is_package = manifest.package_is().context( "try to identify manifest type" ).err_with( report.clone() )?;

    let tree_package_report = | path : AbsolutePath, report : &mut ListReport, visited : &mut HashSet< String > |
    {
      let package = metadata.package_find_by_manifest( path ).unwrap();
      let mut package_report = ListNodeReport
      {
        name: package.name.clone(),
        version : if args.info.contains( &PackageAdditionalInfo::Version ) { Some( package.version.to_string() ) } else { None },
        path : if args.info.contains( &PackageAdditionalInfo::Path ) { Some( package.manifest_path.clone().into_std_path_buf() ) } else { None },
        normal_dependencies: vec![],
        dev_dependencies: vec![],
        build_dependencies: vec![],
      };

      process_package_dependency( &metadata, package, &args, &mut package_report, visited );

      *report = match report
      {
        ListReport::Tree(ref mut v ) => ListReport::Tree( { v.extend([ package_report ]); v.clone() } ),
        ListReport::Empty => ListReport::Tree( vec![ package_report ] ),
        ListReport::List(_ ) => unreachable!(),
      };
    };
    match args.format
    {
      ListFormat::Tree if is_package =>
      {
        let mut visited = HashSet::new();
        tree_package_report( manifest.manifest_path, &mut report, &mut visited )
      }
      ListFormat::Tree =>
      {
        let packages = metadata.packages_get().context( "workspace packages" ).err_with( report.clone() )?;
        let mut visited = packages.iter().map( | p | format!( "{}+{}+{}", p.name, p.version.to_string(), p.manifest_path ) ).collect();
        for package in packages
        {
          tree_package_report( package.manifest_path.as_path().try_into().unwrap(), &mut report, &mut visited )
        }
      }
      ListFormat::Topological =>
      {
        let root_crate = manifest
        .manifest_data
        .as_ref()
        .and_then( | m | m.get( "package" ) )
        .map( | m | m[ "name" ].to_string().trim().replace( '\"', "" ) )
        .unwrap_or_default();

        let dep_filter = move | _p: &Package, d: &Dependency |
        {
          (
            args.dependency_categories.contains( &DependencyCategory::Primary ) && d.kind == DependencyKind::Normal
            || args.dependency_categories.contains( &DependencyCategory::Dev ) && d.kind == DependencyKind::Development
            || args.dependency_categories.contains( &DependencyCategory::Build ) && d.kind == DependencyKind::Build
          )
          &&
          (
            args.dependency_sources.contains( &DependencySource::Remote ) && d.path.is_none()
            || args.dependency_sources.contains( &DependencySource::Local ) && d.path.is_some()
          )
        };

        let packages = metadata.packages_get().context( "workspace packages" ).err_with( report.clone() )?;
        let packages_map =  packages::filter
        (
          packages,
          FilterMapOptions{ dependency_filter: Some( Box::new( dep_filter ) ), ..Default::default() }
        );

        let graph = graph::construct( &packages_map );

        let sorted = toposort( &graph, None ).map_err( | e | { use std::ops::Index; ( report.clone(), err!( "Failed to process toposort for package: {:?}", graph.index( e.node_id() ) ) ) } )?;
        let packages_info = packages.iter().map( | p | ( p.name.clone(), p ) ).collect::< HashMap< _, _ > >();

        if root_crate.is_empty()
        {
          let names = sorted
          .iter()
          .rev()
          .map( | dep_idx | graph.node_weight( *dep_idx ).unwrap().to_string() )
          .map
          (
            | mut name |
            {
              if let Some( p ) = packages_info.get( &name )
              {
                if args.info.contains( &PackageAdditionalInfo::Version )
                {
                  name.push_str( " " );
                  name.push_str( &p.version.to_string() );
                }
                if args.info.contains( &PackageAdditionalInfo::Path )
                {
                  name.push_str( " " );
                  name.push_str( &p.manifest_path.to_string() );
                }
              }
              name
            }
          )
          .collect::< Vec< String > >();

          report = ListReport::List( names );
        }
        else
        {
          let node = graph.node_indices().find( | n | graph.node_weight( *n ).unwrap() == &&root_crate ).unwrap();
          let mut dfs = Dfs::new( &graph, node );
          let mut subgraph = Graph::new();
          let mut node_map = std::collections::HashMap::new();
          while let Some( n )= dfs.next( &graph )
          {
            node_map.insert( n, subgraph.add_node( graph[ n ] ) );
          }

          for e in graph.edge_references()
          {
            if let ( Some( &s ), Some( &t ) ) = ( node_map.get( &e.source() ), node_map.get( &e.target() ) )
            {
              subgraph.add_edge( s, t, () );
            }
          }

          let mut topo = Topo::new( &subgraph );
          let mut names = Vec::new();
          while let Some( n ) = topo.next( &subgraph )
          {
            let mut name = subgraph[ n ].clone();
            if let Some( p ) = packages_info.get( &name )
            {
              if args.info.contains( &PackageAdditionalInfo::Version )
              {
                name.push_str( " " );
                name.push_str( &p.version.to_string() );
              }
              if args.info.contains( &PackageAdditionalInfo::Path )
              {
                name.push_str( " " );
                name.push_str( &p.manifest_path.to_string() );
              }
            }
            names.push( name );
          }
          names.reverse();

          report = ListReport::List( names );
        }
      }
    }

    Ok( report )
  }
}

//

crate::mod_interface!
{
  /// Arguments for `list` endpoint.
  protected use ListArgs;
  /// Additional information to include in a package report.
  protected use PackageAdditionalInfo;
  /// Represents where a dependency located.
  protected use DependencySource;
  /// Represents the category of a dependency.
  protected use DependencyCategory;
  /// Argument for `list` endpoint. Sets the output format.
  protected use ListFormat;
  /// Argument for `list` endpoint. Sets filter(local or all) packages should be in the output.
  protected use ListFilter;
  /// Contains output of the endpoint.
  protected use ListReport;
  /// List packages in workspace.
  orphan use list;
}
