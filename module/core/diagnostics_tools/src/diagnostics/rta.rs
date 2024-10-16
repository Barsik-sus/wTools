/// Internal namespace.
pub( crate ) mod private
{

  ///
  /// Asserts that a boolean expression is true at runtime.
  ///
  /// This will invoke the panic! macro if the provided expression cannot be evaluated to true at runtime.
  ///
  /// ### Basic use-case.
  ///
  /// ``` rust
  /// use diagnostics_tools::prelude::*;
  /// a_true!( 1 == 1, "something wrong" );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! a_true
  {
    () => {};
    (
      $( $Rest : tt )*
    )
    =>
    {
      assert!( $( $Rest )* );
    };
  }

  ///
  /// Asserts that a boolean expression is false at runtime.
  ///
  /// This will invoke the panic! macro if the provided expression cannot be evaluated to false at runtime.
  ///
  /// ### Basic use-case.
  ///
  /// ``` should_panic
  /// use diagnostics_tools::prelude::*;
  /// a_true!( 1 == 2, "something wrong" );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! a_false
  {
    () => {};
    (
      $( $Rest : tt )*
    )
    =>
    {
      assert!( ! $( $Rest )* );
    };
  }

  ///
  /// Asserts that a boolean expression is true at runtime.
  ///
  /// This will invoke the panic! macro if the provided expression cannot be evaluated to true at runtime.
  /// Like [a_true!], this macro also has a second version, where a custom panic message can be provided.
  ///
  /// ### Basic use-case.
  ///
  /// ``` rust
  /// use diagnostics_tools::prelude::*;
  /// a_dbg_true!( 1 == 1, "something wrong" );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! a_dbg_true
  {
    () => {};
    (
      $( $Rest : tt )*
    )
    =>
    {
      debug_assert!( $( $Rest )* );
    };
  }

  ///
  /// Asserts that a boolean expression is false at runtime.
  ///
  /// This will invoke the panic! macro if the provided expression cannot be evaluated to false at runtime.
  /// Like [a_false!], this macro also has a second version, where a custom panic message can be provided.
  ///
  /// ### Basic use-case.
  ///
  /// ``` should_panic
  /// use diagnostics_tools::prelude::*;
  /// a_dbg_true!( 1 == 2, "something wrong" );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! a_dbg_false
  {
    () => {};
    (
      $( $Rest : tt )*
    )
    =>
    {
      debug_assert!( ! $( $Rest )* );
    };
  }

  ///
  /// Asserts that two expressions are identical.
  ///
  /// This macro will invoke the panic! macro if the two expressions have different values at runtime.
  /// Like [a_id!], this macro also has a second version where a custom panic message can be provided.
  ///
  /// ### Basic use-case.
  ///
  /// ``` rust
  /// use diagnostics_tools::prelude::*;
  /// a_dbg_id!( 1, 1, "something wrong" );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! a_dbg_id
  {
    (
      $( $arg:tt )*
    )
    =>
    {
      if cfg!( debug_assertions )
      {
        $crate::a_id!( $( $arg )* );
      }
    };

  }

  ///
  /// Asserts that two expressions are not identical with each other.
  ///
  /// This will invoke the panic! macro if two experessions have the same value at runtime.
  /// Like [a_id!], this macro also has a second version, where a custom panic message can be provided.
  ///
  /// ### Basic use-case.
  ///
  /// ``` rust
  /// use diagnostics_tools::prelude::*;
  /// a_dbg_not_id!( 1, 2, "something wrong" );
  /// ```
  ///

  #[ macro_export ]
  macro_rules! a_dbg_not_id
  {
    (
      $( $arg:tt )*
    )
    =>
    {
      if cfg!( debug_assertions )
      {
        $crate::a_not_id!( $( $arg )* );
      }
    };

  }

  ///
  /// Asserts that two expressions are identical to each other (using [`PartialEq`]). Prints nice diff.
  ///

  #[macro_export]
  macro_rules! a_id
  {
    ( $left:expr , $right:expr $(,)? )
    =>
    ({
      $crate::dependency::pretty_assertions::assert_eq!( $left, $right );
    });
    ($left:expr, $right:expr, $($arg:tt)*)
    =>
    ({
      $crate::dependency::pretty_assertions::assert_eq!( $left, $right, $($arg)+ );
    });
  }

  ///
  /// Asserts that two expressions are not identical to each other (using [`PartialEq`]). Prints nice diff.
  ///

  #[macro_export]
  macro_rules! a_not_id
  {
    ( $left:expr , $right:expr $(,)? )
    =>
    ({
      $crate::dependency::pretty_assertions::assert_ne!( $left, $right );
    });
    ($left:expr, $right:expr, $($arg:tt)*)
    =>
    ({
      $crate::dependency::pretty_assertions::assert_ne!( $left, $right, $($arg)+ );
    });
  }

  pub use a_id;
  pub use a_not_id;
  pub use a_true;
  pub use a_false;
  pub use a_dbg_true;
  pub use a_dbg_false;
  pub use a_dbg_id;
  pub use a_dbg_not_id;
}

#[ doc( inline ) ]
#[ allow( unused_imports ) ]
pub use protected::*;

/// Protected namespace of the module.
pub mod protected
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::orphan::*;
}

/// Orphan namespace of the module.
pub mod orphan
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::exposed::*;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::a_id as assert_eq;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::a_not_id as assert_ne;

}

/// Exposed namespace of the module.
pub mod exposed
{
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::prelude::*;
}

/// Prelude to use essentials: `use my_module::prelude::*`.
pub mod prelude
{

  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::pretty_assertions::assert_eq as a_id;
  // #[ doc( inline ) ]
  // #[ allow( unused_imports ) ]
  // pub use ::pretty_assertions::assert_ne as a_not_id;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::a_id;
  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::a_not_id;

  #[ doc( inline ) ]
  #[ allow( unused_imports ) ]
  pub use super::private::
  {
    a_true,
    a_false,
    a_dbg_true,
    a_dbg_false,
    a_dbg_id,
    a_dbg_not_id,
  };

}

