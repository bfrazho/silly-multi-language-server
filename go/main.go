package main
import "main/server"

func main() {
	router := server.SetUpRouter("http://localhost:8080")
	if err := router.Run(":7000"); err != nil {
		panic("[Error] failed to start Gin server due to: " + err.Error())
	}
}