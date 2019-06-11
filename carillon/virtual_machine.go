/*
 * Virtual Machine
 */
package carillon

import "github.com/dgraph-io/badger"

/*
 * メッセージを解釈して State DB を更新するための仮想マシン。
 */
type VirtualMachine interface {
    // 仮想マシンが State DB とバインドした時に呼び出されます。
    Init(db *badger.DB) error

    // この仮想マシンが使用していたリソースを開放する。
    Close() error

    // 指定されたメッセージを実行します。
    Run(msg *Message) error
}

type NoopVM struct{}

func (vm *NoopVM) Close() error             { return nil }
func (vm *NoopVM) Init(db *badger.DB) error { return nil }
func (vm *NoopVM) Run(msg *Message) error {
    return nil
}
