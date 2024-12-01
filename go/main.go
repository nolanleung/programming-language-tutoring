package main

import (
	"fmt"
	"os"
	"strconv"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Please provide a number")
		os.Exit(1)
	}

	i := 0
	num, err := strconv.Atoi(os.Args[1])
	if err != nil {
		fmt.Println("Please provide a valid number")
		os.Exit(1)
	}

	for i < num {
		fmt.Println(i)
		i += 1
	}
}
