if( typeof module !== 'undefined' )
require( 'wTools' );
var _ = wTools;

console.log( mapToStr( { src : { a : 1, b : 2, c : 3 } } ) ) ;
// log : a:1;b:2;c:3
console.log( mapToStr( { src : { a : 1, b : 2, c : 3 }, keyValDelimeter : ':' } ) );
// log : a:1;b:2;c:3
console.log( mapToStr( { src : { a : 1, b : 2, c : 3 }, keyValDelimeter : ':', entryDelimeter : ';' } ) ) ;
// log : a:1;b:2;c:3

// Oficated routine
function mapToStr( o )
{
  if( _.strIs( o ) )
  o = { src : o }
  _.routineOptions( mapToStr, o );
  _.assert( arguments.length === 1, 'Expects single argument' );
  let result = '';
  for( let s in o.src )
  {
    result += s + o.keyValDelimeter + o.src[ s ] + o.entryDelimeter;
  }
  result = result.substr( 0, result.length-o.entryDelimeter.length );
  return result
}

mapToStr.defaults =
{
  src : null,
  keyValDelimeter : ':',
  entryDelimeter : ';',
}
