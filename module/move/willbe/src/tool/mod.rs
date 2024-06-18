crate::mod_interface!
{

  /// Make sha-1 hash for data.
  layer sha;
  orphan use super::sha;

  /// Errors handling.
  layer error;
  orphan use super::error;

  /// Operate over files.
  layer files;
  orphan use super::files;

  /// Http requests.
  layer http;
  orphan use super::http;

  /// Iterating things.
  layer iter;
  orphan use super::iter;

  /// Work with paths.
  layer path;
  orphan use super::path;

  /// Tools for working with dependencies graph.
  layer graph;
  orphan use super::graph;

  /// Traits and structs for templates.
  layer template;
  orphan use super::template;

  /// Git interaction module that enables seamless integration and management of version control workflows.
  layer git;
  orphan use super::git;

  /// Interaction module with the `cargo` utilities.
  layer cargo;
  orphan use super::cargo;

  /// The parse function parses an input string into a HashMap where the keys are String and the values are of type Value.
  layer query;
  orphan use super::query;

  /// Tools for parsing and extracting information from url.
  layer url;
  orphan use super::url;

  /// Tools for printing a tree
  layer tree;
  orphan use super::tree;

  /// Tools for wrap errors
  layer error_with;
  orphan use super::error_with;

  /// Repository tools.
  layer repository;
  orphan use super::repository;
}
