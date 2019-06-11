/*
 * Replicated State Machine
 */
package carillon

import (
    "github.com/dgraph-io/badger"
    "log"
)

type StateMachine struct {
    // このステートマシンの現在の論理クロック
    logicalClock uint64

    // State DB
    db *badger.DB

    // 仮想マシン
    vm VirtualMachine
}

func NewStateMachine(stateDBPath string) (*StateMachine, error) {

    // 指定されたディレクトリに配置されている Badger データベースをオープン
    opts := badger.DefaultOptions
    opts.Dir = stateDBPath
    opts.ValueDir = stateDBPath
    db, err := badger.Open(opts)
    if err != nil {
        return nil, err
    }

    // ステートマシンの構築と初期状態のロード
    sm := new(StateMachine)
    sm.logicalClock = 0
    sm.db = db

    sm.vm = &NoopVM{}
    vmerr := sm.vm.Init(db)
    if vmerr != nil {
        sm.Close()
        return nil, vmerr
    }

    return sm, nil
}

func (sm *StateMachine) Run(msg *Message) error {
    return sm.vm.Run(msg)
}

func (sm *StateMachine) Close() {
    vmerr := sm.vm.Close()
    if vmerr != nil {
        log.Fatalf("%s", vmerr)
    } else {
        log.Printf("virtual machine closed")
    }

    dberr := sm.db.Close()
    if dberr != nil {
        log.Fatalf("%s", dberr)
    } else {
        log.Printf("database closed")
    }
}
