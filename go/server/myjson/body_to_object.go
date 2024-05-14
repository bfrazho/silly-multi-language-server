package myjson

import (
	"encoding/json"
	"io"
)

func BindJSON[T any](body io.Reader, myVar *T) error{
	bodyString, err := io.ReadAll(body)
	if err != nil {
		return err
	}
	if err := json.Unmarshal(bodyString, &myVar); err != nil{
		return err
	}
	return nil
}