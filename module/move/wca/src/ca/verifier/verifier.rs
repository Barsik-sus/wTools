pub( crate ) mod private
{
  use crate::*;

  use ca::grammar::command::ValueDescription;
  // use former::Former;
  use std::collections::HashMap;
  use wtools::{ error, error::Result, err };

  // TODO: Remove Clone
  /// Converts a `ParsedCommand` to a `VerifiedCommand` by performing validation and type casting on values.
  ///
  /// ```
  /// # use wca::{ Command, Type, Verifier, Dictionary, ParsedCommand };
  /// # use std::collections::HashMap;
  /// # fn main() -> Result< (), Box< dyn std::error::Error > >
  /// # {
  /// # let verifier = Verifier;
  /// let dictionary = Dictionary::former()
  /// .command( Command::former().phrase( "command" ).form() )
  /// .form();
  ///
  /// let raw_command = ParsedCommand
  /// {
  ///   name: "command".to_string(),
  ///   subjects: vec![],
  ///   properties: HashMap::new(),
  /// };
  ///
  /// let grammar_command = verifier.to_command( &dictionary, raw_command )?;
  /// # Ok( () )
  /// # }
  /// ```
  #[ derive( Debug, Clone ) ]
  // #[ derive( Former ) ]
  pub struct Verifier;
  // qqq : delete on completion
  // {
  //   // TODO: Make getters
  //   /// all available commands
  //   #[ setter( false ) ]
  //   pub commands : &'a Dictionary, // qqq : for Bohdan : <- introduce Dictionary for HashMap< String, Vec< Command > >
  // }

  // impl VerifierFormer
  // {
  //   /// Insert a command to the commands list
  //   pub fn command( mut self, command : Command ) -> Self
  //   {
  //     let mut commands = self.container.commands.unwrap_or_default();
  //
  //     let command_variants = commands.entry( command.phrase.to_owned() ).or_insert_with( Vec::new );
  //     command_variants.push( command );
  //
  //     self.container.commands = Some( commands );
  //     self
  //   }
  //
  //   /// Expands the list of commands with received commands
  //   pub fn commands< V >( mut self, commands : V ) -> Self
  //   where
  //     V : Into< Vec< Command > >
  //   {
  //     let mut self_commands = self.container.commands.unwrap_or_default();
  //
  //     for command in commands.into()
  //     {
  //       let command_variants = self_commands.entry( command.phrase.to_owned() ).or_insert_with( Vec::new );
  //       command_variants.push( command );
  //     }
  //
  //     self.container.commands = Some( self_commands );
  //     self
  //   }
  // }

  impl Verifier
  {
    /// Converts raw program to grammatically correct
    ///
    /// Converts all namespaces into it with `to_namespace` method.
    pub fn to_program
    (
      &self,
      dictionary : &Dictionary,
      raw_program : Program< ParsedCommand >
    )
    -> Result< Program< VerifiedCommand > >
    {
      let commands = raw_program.commands
      .into_iter()
      .map( | n | self.to_command( dictionary, n ) )
      .collect::< Result< Vec< VerifiedCommand > > >()?;

      Ok( Program { commands } )
    }

    #[ cfg( feature = "on_unknown_suggest" ) ]
    fn suggest_command< 'a >( dictionary : &'a Dictionary, user_input: &str ) -> Option< &'a str >
    {
      let jaro = eddie::JaroWinkler::new();
      let sim = dictionary
      .commands
      .iter()
      .map( |( name, c )| ( jaro.similarity( name, user_input ), c ) )
      .max_by( |( s1, _ ), ( s2, _ )| s1.total_cmp( s2 ) );
      if let Some(( sim, variant )) = sim
      {
        if sim > 0.0
        {
          let phrase = &variant.phrase;
          return Some( phrase );
        }
      }

      None
    }

    fn find_variant< 'a >
    (
      variants: &'a Command,
      raw_command : &ParsedCommand,
    ) -> Option< &'a Command >
    {
      let mut maybe_valid_variants = vec![];

      for variant @ Command
      {
        subjects,
        properties,
        properties_aliases,
        ..
      }
      in [ variants ]
      {
        let raw_subjects_count = raw_command.subjects.len();
        let expected_subjects_count = subjects.len();
        if raw_subjects_count > expected_subjects_count { continue; }

        let mut maybe_subjects_count = 0_usize;
        for ( k, _v ) in &raw_command.properties
        {
          if properties.contains_key( k ) { continue; }
          if let Some( key ) = properties_aliases.get( k )
          {
            if properties.contains_key( key ) { continue; }
          }
          maybe_subjects_count += 1;
        }

        if raw_subjects_count + maybe_subjects_count > expected_subjects_count { continue; }

        maybe_valid_variants.push( variant );
      }

      // if maybe_valid_variants.len() == 1 { return Some( maybe_valid_variants[ 0 ] ) }
      // qqq: provide better variant selection( E.g. based on types )
      if !maybe_valid_variants.is_empty() { return Some( maybe_valid_variants[ 0 ] ) }
      else { None }
    }
    
    // qqq : for Barsik : 
    // Problem with separating properties and options: 
    // if we pass to wca a command that has an incorrectly named property, it defines this property as part of an option. 
    // You can simulate this problem by running the code from https://github.com/Wandalen/wTools/blob/alpha/module/move/wca/examples/wca_trivial.rs in this form `cargo r .echo propertyf:123` 
    // where the console shows that the option is `propertyf:123` and the property is empty. 
    // 
    // I would like to get an error in this case. 
    // 
    // A real example of the problem can be seen in the .test command in willbe where if you don't specify the option and make a mistake in the name of the properties when running it, 
    // the option will be an incorrectly written property that will produce an error with unobvious output.
    // 
    
    fn extract_subjects( command : &Command, raw_command : &ParsedCommand, used_properties : &[ &String ] ) -> Result< Vec< Value > >
    {
      let mut subjects = vec![];

      let all_subjects = raw_command
      .subjects.clone().into_iter()
      .chain
      (
        raw_command.properties.iter()
        .filter( |( key, _ )| !used_properties.contains( key ) )
        .map( |( key, value )| format!( "{key}:{value}" ) )
      )
      .collect::< Vec< _ > >();
      let mut rc_subjects_iter = all_subjects.iter();
      let mut current = rc_subjects_iter.next();

      for ValueDescription { kind, optional, .. } in &command.subjects
      {
        let value = match current.and_then( | v | kind.try_cast( v.clone() ).ok() )
        {
          Some( v ) => v,
          None if *optional => continue,
          _ => return Err( err!( "Missing not optional subject" ) ),
        };
        subjects.push( value );
        current = rc_subjects_iter.next();
      }
      if let Some( value ) = current { return Err( err!( "Can not identify a subject: `{}`", value ) ) }

      Ok( subjects )
    }

    fn extract_properties( command: &Command, raw_command : HashMap< String, String > ) -> Result< HashMap< String, Value > >
    {
      raw_command.into_iter()
      .filter_map
      (
        |( key, value )|
        // try to find a key
        if command.properties.contains_key( &key ) { Some( key ) }
        else if let Some( original_key ) = command.properties_aliases.get( &key ) { Some( original_key.clone() ) }
        else { None }
        // give a description. unwrap is safe because previous checks
        .map( | key | ( command.properties.get( &key ).unwrap(), key, value ) )
      )
      .map
      (
        |( value_description, key, value )|
        value_description.kind.try_cast( value ).map( | v | ( key.clone(), v ) )
      )
      .collect::< Result< HashMap< _, _ > > >()
    }

    fn group_properties_and_their_aliases< 'a, Ks >( aliases : &'a HashMap< String, String >, used_keys :  Ks ) -> Vec< &String >
    where
      Ks : Iterator< Item = &'a String >
    {
      let reverse_aliases =
      {
        let mut map = HashMap::< &String, Vec< &String > >::new();
        for ( property, alias ) in aliases
        {
          map.entry( alias ).or_default().push( property );
        }
        map
      };

      used_keys.flat_map( | key |
      {
        reverse_aliases.get( key ).into_iter().flatten().map( | k | *k ).chain( Some( key ) )
      })
      .collect::< Vec< _ > >()
    }

    /// Converts raw command to grammatically correct
    ///
    /// Make sure that this command is described in the grammar and matches it(command itself and all it options too).
    pub fn to_command( &self, dictionary : &Dictionary, raw_command : ParsedCommand ) -> Result< VerifiedCommand >
    {
      let variants = dictionary.command( &raw_command.name )
      .ok_or_else::< error::for_app::Error, _ >
      (
        ||
        {
          #[ cfg( feature = "on_unknown_suggest" ) ]
          if let Some( phrase ) = Self::suggest_command( dictionary, &raw_command.name )
          { return err!( "Command not found. Maybe you mean `.{}`?", phrase ) }
          err!( "Command not found. Please use `.` command to see the list of available commands." )
        }
      )?;

      let Some( cmd ) = Self::find_variant( variants, &raw_command ) else
      {
        error::for_app::bail!
        (
          "`{}` command with specified subjects not found. Available variants `{:#?}`",
          &raw_command.name,
          [ variants ]
          .into_iter()
          .map
          (
            | x |
            format!
            (
              ".{}{}",
              &raw_command.name,
              {
                let variants = x.subjects.iter().filter( | x | !x.optional ).map( | x | format!( "{:?}", x.kind ) ).collect::< Vec< _ > >();
                if variants.is_empty() { String::new() } else { variants.join( "" ) }
              }
            )
          )
          .collect::< Vec< _ > >()
        );
      };

      let properties = Self::extract_properties( cmd, raw_command.properties.clone() )?;
      let used_properties_with_their_aliases = Self::group_properties_and_their_aliases( &cmd.properties_aliases, properties.keys() );
      let subjects = Self::extract_subjects( cmd, &raw_command, &used_properties_with_their_aliases )?;

      Ok( VerifiedCommand
      {
        phrase : cmd.phrase.to_owned(),
        subjects,
        properties,
      })
    }
  }
}

//

crate::mod_interface!
{
  exposed use Verifier;
}
