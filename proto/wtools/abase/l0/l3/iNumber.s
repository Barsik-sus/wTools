( function _iNumbers_s_() {

'use strict';

let _global = _global_;
let _ = _global_.wTools;
let Self = _global_.wTools;

// --
// number
// --

/**
 * @summary Checks if argument( src ) is a Number.
 * @returns Returns true if ( src ) is a Number, otherwise returns false.
 *
 * @example
 * numberIs( 5 );
 * // returns true
 *
 * @example
 * numberIs( 'song' );
 * // returns false
 *
 * @param {*} src.
 * @return {Boolean}.
 * @function numberIs
 * @namespace Tools
 */

function numberIs( src )
{
  return typeof src === 'number';
  return Object.prototype.toString.call( src ) === '[object Number]';
}

//

function numberIsNotNan( src )
{
  return _.numberIs( src ) && !isNaN( src );
}

//

function numberIsFinite( src )
{
  if( !_.numberIs( src ) )
  return false;
  return isFinite( src );
}

//

function numberIsInfinite( src )
{

  if( !_.numberIs( src ) )
  return false;

  return src === +Infinity || src === -Infinity;
}

//

function intIs( src )
{

  if( !_.numberIs( src ) )
  return false;

  return Math.floor( src ) === src;
}

//

function numbersAreAll( src )
{
  _.assert( arguments.length === 1 );

  if( _.bufferTypedIs( src ) )
  return true;

  if( _.arrayLike( src ) & !_.arrayIsEmpty( src ) )
  {
    for( let s = 0 ; s < src.length ; s++ )
    if( !_.numberIs( src[ s ] ) )
    return false;
    return true;
  }

  return false;
}

// //
//
// function numbersAreIdentical( src1, src2 )
// {
//   _.assert( arguments.length === 2, 'Expects exactly two arguments' );
//   return Object.is( src1, src2 );
// }
//
// //
//
// function numbersAreEquivalent( src1, src2, accuracy )
// {
//   _.assert( arguments.length === 2 || arguments.length === 3, 'Expects two or three arguments' );
//   if( accuracy === undefined )
//   accuracy = _.accuracy;
//   return Math.abs( src1-src2 ) <= accuracy;
// }

//

function numbersAreIdentical( a, b )
{
  _.assert( arguments.length === 2, 'Expects exactly two arguments' );

  if( _.numbersAreAll( [ a, b ] ) )
  return Object.is( a, b );

  return false;
}

//

function numbersAreIdenticalNotStrictly( a, b )
{
  /*
  it takes into account -0 === +0 case
  */

  _.assert( arguments.length === 2, 'Expects exactly two arguments' );

  if( _.numbersAreAll( [ a, b ] ) )
  return Object.is( a, b ) || a === b;

  return false;
}

//

function numbersAreEquivalent( a, b, accuracy )
{
  _.assert( arguments.length === 2 || arguments.length === 3, 'Expects two or three arguments' );

  if( _.numbersAreAll( [ a, b ] ) )
  {
    if( Object.is( a, b ) )
    return true;
  }
  else
  {
    return false;
  }

  if( _.bigIntIs( a ) || _.bigIntIs( b ) )
  {
    if( _.intIs( a ) )
    a = BigInt( a );
    if( _.intIs( b ) )
    b = BigInt( b );
    return a === b;
  }


  if( accuracy === undefined )
  accuracy = this.accuracy;
  // console.log( '-----------------' );
  // console.log( 'ACC: ', accuracy )
  // console.log( 'DIFF: ', Math.abs( a - b ) )
  // console.log( 'ACC2: ', +( Math.abs( a - b ) ).toFixed( 10 ) )
  // console.log( 'DIFF2: ', +( accuracy ).toFixed( 10 ) )
  // console.log( '-----------------' );
  // return +( Math.abs( a - b ) ).toFixed( 10 ) <= +( accuracy ).toFixed( 10 );

  return Math.abs( a - b ) <= accuracy
}

//

function numbersAreFinite( src )
{

  if( _.longIs( src ) )
  {
    for( let s = 0 ; s < src.length ; s++ )
    if( !numbersAreFinite( src[ s ] ) )
    return false;
    return true;
  }

  if( !_.numberIs( src ) )
  return false;

  return isFinite( src );
}

//

function numbersArePositive( src )
{

  if( _.longIs( src ) )
  {
    for( let s = 0 ; s < src.length ; s++ )
    if( !numbersArePositive( src[ s ] ) )
    return false;
    return true;
  }

  if( !_.numberIs( src ) )
  return false;

  return src >= 0;
}

//

function numbersAreInt( src )
{

  if( _.longIs( src ) )
  {
    for( let s = 0 ; s < src.length ; s++ )
    if( !numbersAreInt( src[ s ] ) )
    return false;
    return true;
  }

  if( !_.numberIs( src ) )
  return false;

  return Math.floor( src ) === src;
}

// --
// fields
// --

let Fields =
{
}

// --
// routines
// --

let Routines =
{

  numberIs,
  numberIsNotNan,
  numberIsFinite,
  numberDefined : numberIsFinite,
  numberIsInfinite,
  intIs,

  numbersAreAll,
  // numbersAreIdentical,
  // numbersAreEquivalent,
  numbersAreIdentical, /* qqq2 : implement good coverage | aaa : Done. Yevhen S. */
  numbersAreIdenticalNotStrictly,
  numbersAreEquivalent, /* qqq2 : implement good coverage */

  numbersAreFinite,
  numbersArePositive,
  numbersAreInt,

}

//

Object.assign( Self, Routines );
Object.assign( Self, Fields );

// --
// export
// --

if( typeof module !== 'undefined' )
module[ 'exports' ] = Self;

})();
