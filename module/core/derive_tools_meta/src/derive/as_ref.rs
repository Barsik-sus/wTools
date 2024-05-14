
use super::*;
use macro_tools::{ type_struct, Result };

//

pub fn as_ref( input : proc_macro::TokenStream ) -> Result< proc_macro2::TokenStream >
{
  let parsed = syn::parse::< type_struct::TypeStructParsed >( input )?;
  let field_type = parsed.first_field_type()?;
  let item_name = parsed.item.ident;

  let result = qt!
  {
    impl AsRef< #field_type > for #item_name
    {
      fn as_ref( &self ) -> &#field_type
      {
        &self.0
      }
    }
  };

  Ok( result )
}
