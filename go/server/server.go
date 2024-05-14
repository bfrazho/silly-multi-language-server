package server

import (
	"main/server/user"
	"github.com/gin-gonic/gin"
)



func SetUpRouter(userBaseUrl string) *gin.Engine{
	router := gin.Default()
	user.SetUpUserRoutes(router, userBaseUrl)
	return router
}