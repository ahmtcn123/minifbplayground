function scale1D(arr, n) 
{
  for (var i = arr.length *= n; i; ) 
    arr[--i] = arr[i / n | 0]
}

function scale2D(arr, n) 
{
  for (var i = arr.length; i; )
    scale1D(arr[--i], n)

  scale1D(arr, n)
}

var a = [ [0, 0, 1, 0], [0, 1, 1, 
  1],  [0, 0, 1, 0],  [0, 0, 1, 1] ]
console.log( JSON.stringify( a ).replace(/],/g, '],\n ') )

console.time( 1e5 )
scale2D(a, 1e5)
console.timeEnd( 1e5 )

var b = [ [0, 0, 1, 0], [0, 1, 1, 1],  [0, 0, 1, 0],  [0, 0, 1, 1] ]
scale2D(b, 4)
console.log( JSON.stringify( b ).replace(/],/g, '],\n ') )