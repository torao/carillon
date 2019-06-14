/*
 * Virtual Machine
 */
package carillon

import (
    "github.com/tendermint/iavl"
    "log"
    "math/big"
    "regexp"
)

/*
 * メッセージを解釈して State DB を更新するための仮想マシン。
 */
type VirtualMachine interface {
    // 仮想マシンが State DB とバインドした時に呼び出されます。
    Init(db StateDB) error

    // この仮想マシンが使用していたリソースを開放する。
    Close() error

    // 指定されたメッセージを実行します。
    Run(msg *Message) error
}

type SimpleVM struct {
    // TODO StateDB で定義してメソッドを参照するには?
    db *iavl.MutableTree
}

func (vm *SimpleVM) Close() error { return nil }
func (vm *SimpleVM) Init(db StateDB) error {
    vm.db = db
    return nil
}

var keyValuePattern = regexp.MustCompile("([a-zA-Z0-9_\\-]*)([+\\-*/]?=)([0-9]+)")

func (vm *SimpleVM) Run(msg *Message) error {
    group := keyValuePattern.FindSubmatch(msg.Payload)
    variable := group[1]
    operator := string(group[2])
    value, _ := new(big.Int).SetString(string(group[3]), 10)

    _, bytes := vm.db.Get(variable)
    currentValue := new(big.Int).SetBytes(bytes)
    result := new(big.Int)
    switch operator {
    case "+=":
        result.Add(currentValue, value)
    case "-=":
        result.Sub(currentValue, value)
    case "*=":
        result.Mul(currentValue, value)
    case "/=":
        result.Div(currentValue, value)
    case "=":
        result.Set(value)
    default:
    }
    log.Printf("%s = %s %s %s -> %s", string(variable), currentValue, operator, value, result)
    vm.db.Set(variable, result.Bytes())
    return nil
}

type NoopVM struct{}

func (vm *NoopVM) Close() error          { return nil }
func (vm *NoopVM) Init(db StateDB) error { return nil }

func (vm *NoopVM) Run(msg *Message) error { return nil }
