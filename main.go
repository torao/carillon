package main

import (
    "log"
    "os"
)
import "fmt"
import "./carillon"

func main() {
    fmt.Println("main()")
    sm, err := carillon.NewStateMachine("./.statedb")
    if err != nil {
        fmt.Println(err)
        log.Fatalf("%s", err)
        os.Exit(-1)
    } else {
        defer sm.Close()
        fmt.Println("hello, world")
    }
}
