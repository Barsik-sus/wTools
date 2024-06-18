mod private
{
  use crate::*;
  // use path::AbsolutePath;
  use action;
  // use error::untyped::Error;
  use error::untyped::Error;
  use error::{ Result, err };
  use std::fmt::{ Display, Formatter };

  #[ derive( Debug, Default ) ]
  struct ReadmeHeadersRenewReport
  {
    main_header_renew_report : action::MainHeaderRenewReport,
    main_header_renew_error : Option< Error >, // qqq : for Petro : typed error
    modules_headers_renew_report : action::ModulesHeadersRenewReport,
    modules_headers_renew_error : Option< Error >, // qqq : for Petro : typed error
  }

  impl Display for ReadmeHeadersRenewReport
  {
    fn fmt( &self, f : &mut Formatter< '_ > ) -> std::fmt::Result
    {
      match ( &self.main_header_renew_error, &self.modules_headers_renew_error )
      {
        ( Some( main ), Some( modules ) ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : \n{}\nError : \n{:?}\nModules headers renew report : \n{}\nError : \n{:?}",
            self.main_header_renew_report, main, self.modules_headers_renew_report, modules
          )?;
        }
        ( Some( main ), None ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : \n{}\nError : \n{:?}\nModules headers renew report : \n{}",
            self.main_header_renew_report, main, self.modules_headers_renew_report
          )?;
        }
        ( None, Some( modules) ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : \n{}\nModules headers renew report : \n{}\nError : \n{:?}\n",
            self.main_header_renew_report, self.modules_headers_renew_report, modules
          )?;
        }
        ( None, None ) =>
        {
          writeln!
          (
            f,
            "Main header renew report : \n{}\n\nModules headers renew report : \n{}",
            self.main_header_renew_report, self.modules_headers_renew_report
          )?;
        }
      }
      Ok( () )
    }
  }


  /// Aggregates two commands: `generate_modules_headers` & `generate_main_header`
  pub fn readme_headers_renew() -> Result< () >
  {
    let mut report = ReadmeHeadersRenewReport::default();
    // let absolute_path = AbsolutePath::try_from( std::env::current_dir()? )?;
    let crate_dir = CrateDir::try_from( std::env::current_dir()? )?;
    let mut fail = false;

    match action::readme_header_renew( crate_dir.clone() )
    {
      Ok( r ) =>
      {
        report.main_header_renew_report = r;
      }
      Err( ( r, error ) ) =>
      {
        fail = true;
        report.main_header_renew_report = r;
        report.main_header_renew_error = Some( Error::from( error ) );
      }
    };
    match action::readme_modules_headers_renew( crate_dir )
    {
      Ok( r ) =>
      {
        report.modules_headers_renew_report = r;
      }
      Err( ( r, error ) ) =>
      {
        fail = true;
        report.modules_headers_renew_report = r;
        report.modules_headers_renew_error = Some( Error::from( error ) );
      }
    }

    if fail
    {
      eprintln!( "{report}" );
      Err( err!( "Something went wrong" ) )
    }
    else
    {
      println!( "{report}" );
      Ok( () )
    }
  }
}

crate::mod_interface!
{
  /// Generate header's.
  orphan use readme_headers_renew;
}