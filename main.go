package main

import (
	"fmt"
	"reflect"
)

type Sample struct {
	Id    int    `tag:"none"`
	Value string `tag:"display"`
}

func main() {
	u := Sample{
		Id:    44,
		Value: "Value",
	}

	t := reflect.TypeOf(u)

	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		if value, ok := f.Tag.Lookup("tag"); ok {
			fmt.Println(value)
		}
	}
}
