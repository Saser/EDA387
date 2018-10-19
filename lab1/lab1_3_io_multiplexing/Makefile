FLAGS = -Wall -Wextra -g3

all: server client-simple client-multi

server: server-iterative.cpp
	g++ $(FLAGS) $^ -o $@

client-simple: client-simple.cpp
	g++ $(FLAGS) $^ -o $@

client-multi: client-multi.cpp
	g++ $(FLAGS) $^ -o $@
