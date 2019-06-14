package main

import (
    "flag"
    "log"
    "os"
)
import "fmt"
import "./carillon"

func main() {
    log.Printf("main(%s)\n", os.Args)

    flag.Usage = help
    dbPath := flag.String("db", ".statedb", "database directory path")
    showHelp := flag.Bool("h", false, "show this message")
    flag.Parse()

    if *showHelp {
        help()
        os.Exit(1)
    }

    sm, err := carillon.NewStateMachine(*dbPath)
    if err != nil {
        fmt.Println(err)
        log.Fatalf("%s", err)
        os.Exit(-1)
    } else {
        defer sm.Close()

        msg := &carillon.Message{Payload: []byte("x=10")}
        smErr := sm.Run(msg)
        if smErr != nil {
            _, err := fmt.Fprintf(os.Stderr, "ERROR: %s\n", smErr)
            if err != nil {
                log.Fatalf("%s", err)
            }
            os.Exit(1)
        }
        log.Println("exiting carillon")
    }
}

func help() {
    _, err := fmt.Fprintf(os.Stderr, "USAGE: %s\n", os.Args[0])
    if err == nil {
        flag.PrintDefaults()
    }
}
