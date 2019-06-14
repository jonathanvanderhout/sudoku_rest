var express = require("express");
var app = express();
var bodyParser = require('body-parser');
var sudokuSolve = require("./sudoku_rust_wrapper").sudokuSolve
app.use(bodyParser.json()); // support json encoded bodies
app.use(bodyParser.urlencoded({ extended: true })); // support encoded bodies
app.use(function(req, res, next) {
  res.header("Access-Control-Allow-Origin", "*");
  res.header("Access-Control-Allow-Headers", "Origin, X-Requested-With, Content-Type, Accept");
  next();
});

app.use(function(request, response, next) {
 console.log("In comes a " + request.method + " to " + request.url);
 next();
});

app.post("/", (req,res)=>{
  console.log("did we even get here?")

  var result = sudokuSolve(req.body)
  console.log(req.body)

  console.log(result)

  res.json(result)
})

app.listen(3002, function() {
  console.log("App started on port 3002");
});
