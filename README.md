To setup and run: 

      git clone https://github.com/jonathanvanderhout/sudoku_rest.git

      cd sudoku_rest

      ls

      npm i

      cd sudoku_rust_wrapper

      ls

      neon build

      cd ..

      ls

      node server.js 
  

To use:

curl -d '[ [ 8, 0, 0, 0, 0, 0, 0, 0, 0 ],[ 0, 0, 3, 6, 0, 0, 0, 0, 0 ],[ 0, 7, 0, 0, 9, 0, 2, 0, 0 ],[ 0, 5, 0, 0, 0, 7, 0, 0, 0 ],[ 0, 0, 0, 0, 4, 5, 7, 0, 0 ],[ 0, 0, 0, 1, 0, 0, 0, 3, 0 ],[ 0, 0, 1, 0, 0, 0, 0, 6, 8 ],[ 0, 0, 8, 5, 0, 0, 0, 1, 0 ],[ 0, 9, 0, 0, 0, 0, 4, 0, 0 ] ]' -H "Content-Type: application/json" -X POST http://127.0.0.1:3002/

