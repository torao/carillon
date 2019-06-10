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

    return sm, nil
}

func (sm *StateMachine) Close() {
    err := sm.db.Close()
    if err != nil {
        log.Fatalf("%s", err)
    } else {
        log.Printf("database closed")
    }
}
