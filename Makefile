# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: jlohmann <jlohmann@student.42heilbronn.de> +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2023/08/20 09:42:03 by jlohmann          #+#    #+#              #
#    Updated: 2023/08/20 09:54:04 by jlohmann         ###   ########.fr        #
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
