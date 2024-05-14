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
	if err := context.BindJSON(&userRequest); err != nil {
		context.String(http.StatusBadRequest, err.Error())
		return
	}
	
	requestInput, _ := json.Marshal(userRequest)
	response, err := http.Post(userServer.baseUrl+"/users","application/json", bytes.NewBuffer(requestInput))
	if err != nil {
		context.String(http.StatusInternalServerError, err.Error())
		return
	}

	var userResponse UserResponse
	if err := myjson.BindJSON(response.Body, &userResponse); err != nil {
		context.String(http.StatusInternalServerError, err.Error())
		return
	}
	context.JSON(http.StatusOK, userResponse)
}