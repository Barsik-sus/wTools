<!-- {{# generate.module_header{} #}} -->

# Module :: sqlx_query
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/Modulesqlx_queryPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/Modulesqlx_queryPush.yml) [![docs.rs](https://img.shields.io/docsrs/sqlx_query?color=e3e8f0&logo=docs.rs)](https://docs.rs/sqlx_query) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Fsqlx_query_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20sqlx_query_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

The tool to make CLI ( commands user interface ). It is able to aggregate external binary applications, as well as functions, which are written in your language.

## Sample

<!-- {{# generate.module_sample{} #}} -->

```rust
use sqlx_query::*;

let user : User = query_as!( User, "SELECT * FROM users LIMIT 1" )
    .fetch_one( executor )
    .await?;

query!( "DELETE FROM users WHERE id = $1", user.id )
    .execute( executor )
    .await?;
}

// Expands to

let user : User =
  {
    #[ cfg( feature = "sqlx_compiletime_checks" ) ]
    let q = ::sqlx::query_as::< _, User >( "SELECT * FROM users LIMIT 1" );
    #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
    let q = ::sqlx::query_as!( User, "SELECT * FROM users LIMIT 1" );
    q
  }
    .fetch_one( executor )
    .await?;
{
  #[ cfg( feature = "sqlx_compiletime_checks" ) ]
  let q = ::sqlx::query( "DELETE FROM users WHERE id = $1" ).bind( user.id );
  #[ cfg( not( feature = "sqlx_compiletime_checks" ) ) ]
  let q = ::sqlx::query!( "DELETE FROM users WHERE id = $1", user.id );
  q
}
    .execute( executor )
    .await?;
```

### To add to your project

```sh
cargo add sqlx_query
```

### Try out from the repository

```sh
git clone https://github.com/Wandalen/wTools
cd wTools
cd sample/rust/sqlx_query_trivial
cargo run
```

