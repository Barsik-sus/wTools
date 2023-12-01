mod private
{
  use std::path::Path;
  use anyhow::anyhow;
  use crate::process;
  use crate::process::CmdReport;
  use crate::wtools::error::Result;

  /// Cargo publish.
  pub fn publish< P >( path : P, dry : bool ) -> Result< CmdReport >
  where
    P : AsRef< Path >
  {
    let command = "cargo publish";

    if dry
    {
      Ok
      (
        CmdReport
        {
          command : command.to_string(),
          path : path.as_ref().to_path_buf(),
          out : String::new(),
          err : String::new(),
        }
      )
    }
    else
    {
      process::start_sync( command, path.as_ref() ).map_err( | e | anyhow!( "{e}" ) )
    }
  }
}

//

crate::mod_interface!
{
  protected use publish;
}
