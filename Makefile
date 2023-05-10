##
## EPITECH PROJECT, 2022
## MyTeams
## File description:
## Makefile
##

SRC			=	src/main.rs

NAME		=	raytracer

CC			=	cargo build --release

all:
	$(CC)
	cp target/release/$(NAME) .

debug: CPPFLAGS += -g3
debug: re

fclean:	clean
	$(RM) $(NAME)

re:	fclean all

.PHONY: all clean fclean re
