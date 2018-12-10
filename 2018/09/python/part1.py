#!/usr/bin/env python3

with open('../input', 'r') as file:
    input_list = file.read().strip().split(' ')
    players = int(input_list[0])
    last_marble = int(input_list[6])
    scores = [ 0 for i in range(players) ]
    game_board = []

    marble = 0
    marble_index = 0

    for turn in range(0, last_marble+1):
        if (marble_index + 1) >= len(game_board):
            marble_index -= len(game_board)
        elif (marble_index + 1) < 1:
            marble_index = len(game_board) + marble_index

        if marble > 0 and marble % 23 == 0:
            marble_index -= 7
            if (marble_index + 1) >= len(game_board):
                marble_index -= len(game_board)
            elif (marble_index + 1) < 1:
                marble_index = len(game_board) + marble_index
            scores[turn % players - 1] += marble
            scores[turn % players - 1] += game_board[marble_index]
            del game_board[marble_index]
            marble += 1
        else:
            marble_index += 2
            game_board.insert(marble_index, marble)
            marble += 1

#        print(turn, game_board, scores)
    print(max(scores))

