package user

import (
	"bytes"
	"encoding/json"
	"net/http"
	"main/server/myjson"
	"github.com/gin-gonic/gin"
)

func SetUpUserRoutes(router *gin.Engine, baseUrl string) *gin.Engine {
	userServer := UserServer {
		baseUrl: baseUrl,
	}
	router.POST("/users", userServer.createUser)
	return router
}

type UserRequest struct {
	Name string `json:"name"`
	Job string `json:"job"`
}
type UserResponse struct {
	Name string `json:"name"`
	Job string `json:"job"`
	Id string `json:"id"`
}

type UserServer struct {
	baseUrl string
}


func (userServer UserServer) createUser(context *gin.Context){
	var userRequest UserRequest
	context.BindJSON(&userRequest)
	requestInput, _ := json.Marshal(userRequest)
	response, err := http.Post(userServer.baseUrl+"/users","application/json", bytes.NewBuffer(requestInput))
	if err != nil {
		println(err.Error())
	}
	var userResponse UserResponse
	myjson.BindJSON(response.Body, &userResponse)
	context.JSON(http.StatusOK, userResponse)
}