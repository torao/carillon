/*
 * Replicated State Machine
 */
package carillon

import (
    "github.com/tendermint/iavl"
    tmdb "github.com/tendermint/tendermint/libs/db"
    "log"
)

type StateDB *iavl.MutableTree

type StateMachine struct {
    // このステートマシンの現在の論理クロック
    logicalClock uint64

    // State DB
    db StateDB

    // 仮想マシン
    vm VirtualMachine

}

func NewStateMachine(stateDBPath string) (*StateMachine, error) {

    // 指定されたディレクトリに配置されているデータベースをオープン
    db := iavl.NewMutableTree(tmdb.NewDB("name", tmdb.GoLevelDBBackend, stateDBPath), 128)

    // ステートマシンの構築と初期状態のロード
    sm := new(StateMachine)
    sm.logicalClock = 0
    sm.db = iavl.NewMutableTree(tmdb.NewMemDB(), 128)

    sm.vm = &SimpleVM{}
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
}
