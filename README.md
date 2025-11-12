# A Tic-Tac-Toe game in Rust

My code project for the 2023 summer vacation was to convert a Swift tac-tac-toe game that I have previously written to the Rust language and learn some Rust doing it. 

It was a fun experience and I really enjoyed learning the basics of Rust.

In 2025 I have expanded it to support neural network play. Inspired by https://www.geeksforgeeks.org/numpy/implementation-of-neural-network-from-scratch-using-numpy/ I first wrote a python implementation of basic pattern recognition and then converted it into Rust. Then I expanded it to tic-tac-toe pattern training using basic tree search as training algorithm. 

Some features

* Play as X or O against the computer
* Random selection of who starts
* Most functions have unit tests
* computer vs computer is also a test. The output of the computer playing aginst itself can be viewed by selecting profile "Unit test with output" as "Run and Debug" profile in VS Code.
* computer tree seach vs neural net test where the assert is still a draw. This might not always happen if the initial matrix generation is unsuitable. 
  * The test can be run with a different number training rounds to see the impact of training. Tree search will almost always win without training. With 5 training rounds it will mostly be a draw.  


Todo
* Add neural net against human player