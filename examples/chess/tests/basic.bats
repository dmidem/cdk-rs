# Executed before each test.
setup() {
  cd examples/chess
  # Make sure the directory is clean.
  npm install

  dfx start --clean --background
}

# executed after each test
teardown() {
  dfx stop
}

@test "Can play chess against AI" {
  dfx deploy
  run dfx canister call chess_rs new '("test", true)'
  [ "$output" == "()" ]
  run dfx canister call chess_rs move '("test", "e2e4")'
  [ "$output" == "(true)" ]
  run dfx canister call chess_rs move '("test", "d2d3")'
  [ "$output" == "(true)" ]
  run dfx canister call chess_rs getFen '("test")'
  [ "$output" == '(opt "rnb1kbnr/pp1ppppp/1qp5/8/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 1 3")' ]
}
