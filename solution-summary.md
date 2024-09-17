# Solution Summary for FAU Halma
## Problem Definition
* In this assignment using adversarial search in AI, I built an agent that plays Chinese Checkers variant against another AI in the server
* In order to build the AI, I need a complete API that can handle board and pieces and other functionality that is required for the Agent
* Since this is an adversarial search task the first algorithm's that comes to my mind are MiniMax ,Minimax with Alphabeta Pruning and Monte Carlo tree search
* The above algorithms are the ones that are discussed in the AI-1 lecture.
* I implemented all the above implementations to beat several AI agents that are in the server

Now I will explain several approaches that I took for different agents that I implemented across different branches and finally comes to the main branch that will beat the hard core agent
> I did not write the documentation across all the branches, but I will summarize all the different branches and the approaches that I took in the main branch itself

## Different Environments and their Branches
If you go and check the server My agent managed to get desired rating for all the environment that are given in the sever in order to pass the task, so I will go environment by environment in order to
explain different implementations and the approaches I took

### Environment 1
#### Description of the environment :
+ Here the board shape is Rhombus instead of traditional star shape of the chinese checkers
+ It's a two player mode where the players start at opposite sides of the board
+ The agents goal is to reach the opposite homes the first one reaches wins

So at first to move forward I need an API that gives me functionality of the board I will try to explain different approaches I took here.
For this at first I took the representation of the board and hardcoded the positions as pieces into a hashmap, Here I have two choices use a matrix based representation
which also works but, my first approach was hard code the board values in a hashmap and the API essentially handles the functionality of operation on the hashmap with the hardcode board.

Now when it comes to adversarial search the board representation of hardcoded pieces is the States on which the mini max algorithm with alpha beta pruning will run, but the key problem was evaluating the board
based on some metric and also restricting the state space in order to make efficient search, My observation at first was without restricting the state space in some way the minimax will at most go to 2 or 3 depth,
Compare this to any greedy agent and the greedy agent will do better and minimax is not so efficient.

so to restrict the state space I only consider forward moves for both the agents in my minimax game tree, which finally did the trick, this was the major observation that I learned by implementing this small agent.
### Environment 2, 3 and 4
#### Description of the environment :
+ In this environment the only difference is the size and shape of the board.
+ Here the shape of the board is star shaped like the traditional checkers boards.
+ Also the AI agents are challenging as the 4th environment is the most difficult one to play against,

There are not a lot of key differences compared to the environment 1, I essentially used the same agent to beat the rhombus the environments the api functionality is also the same. But some thing new I did here is to experiment with new board representation as supposed to hashmap which is a sparse representation of the board. Here I also tried the matrix representation with and array. This is useful for debugging the game because in my api for the hashmap i did not have any functionality to print the board to the console, but this array representation gives a functionality to print the board as debug output to the console,

> you will see the debug out put of the game when running the agent which is handy to observe what your algorithm is doing.

So far the methods I used were greedy or Minimax and alphabeta pruning based which are not so Intelligent and efficient respectively. In this environment my first approach was just Minimax with different heurestics. I will summarize different heurestics that i considered here.

1. Distance heurestics : there are two types of distance heurestics to consider one is the distance between the home goal and the individual pieces and the other is distance between the start and the pieces.
2. Centroid of the pieces : my intuition is to use cetroid of the pieces, essentially centroid gives me the centre of mass of the pieces and which could tell me how close the pieces are to each other to get easy hops for the pieces.

I used the above metrics to evaluate my board in the minimax and as well during greedy, which worked well, but my goal is to beat the hardcore three player variant, since minimax didnot seem optimal i had to look for another approach, Now here comes to save the day Monte Carlo Tree search which i will elaborate next.

### Environment 5,6,7 and 8
#### Description of the environment :
+ Now In this environment the difference from the previous environments is that it is three player environment.
+ In this environment minimax and alpha beta pruning will not be optimal because of the lookup depth.
+ For two player environment event though the state space is restricted during the search the maximum depth the agent could go was 6 ply's. Here also if we say it goes the same depth for each player it will only do a lookup of 1 move ahead which is not effective for three players and we cannot afford to increase the depth since i dont know when the agent will return a  valid move back.

So with above observations and short comings as I discussed previously Now its time to change the strategy and implement the Monte carlo tree search algorithm which helped me to beat the hardcore environment.

In both minimax and monte carlo tree search the agent will build a game tree and return a promising move from the game tree. But unlike minimax which is strategic in nature monte carlo relies on random sampling i.e It builds the game tree based on Random simulations of the game and returns the most promising move back.

The heart of monte carlo tree search are two things one is the simulation phase and the other is the UCT1 formula which tries to balance out exploration and exploitation factors in building the game tree. So for my agent to work at first i struggle a lot with the simulation of the game since the state space is so large, even if we try to put a random simulation it will never end. So i restricted the state space fairly for all the players which lead to the end of simulation in a fair manner. I am doing around 10,000 iterations which showed me some promising results for the game tree.


## Conclusion:

There is so much more that I didnot cover in this summary because it will make it too long and boring. My findings are for multiplayer games if we tweek the monte carlo tree search it is more effective that minimax and alpha beta pruning.
