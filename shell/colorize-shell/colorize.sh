#!/bin/sh

colorize() {
  printf "\x1b[38;2;${1};${2};${3}m${4}\x1b[0m\n"
}

colorize $1 $2 $3 $4