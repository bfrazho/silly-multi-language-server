package myjson

import (
	"encoding/json"
	"io"
)

func BindJSON[T any](body io.Reader, myVar *T){
	bodyString, _ := io.ReadAll(body)
	json.Unmarshal(bodyString, &myVar)
}