# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: jwillert <jwillert@student.42heilbronn.de> +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2023/08/20 09:42:03 by jlohmann          #+#    #+#              #
#    Updated: 2023/08/20 16:39:09 by jwillert         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

NAME=bsq

release:
	cargo build --release
	@cp target/release/$(NAME) .

debug:
	cargo build
	@cp target/debug/$(NAME) .

check:
	cargo check

clean:
	cargo clean

test: debug
	cd ./tester && ./test.sh