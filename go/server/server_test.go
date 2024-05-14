package server

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"

	"main/server/myjson"
	"main/server/user"

	"github.com/stretchr/testify/assert"
)

func TestCanPostUserHappyPath(t *testing.T) {
	requestBody := `{"name": "the name", "job": "the role"}`
    server := httptest.NewServer(http.HandlerFunc(func(responseWriter http.ResponseWriter, request *http.Request) {
        assert.Equal(t, "/users", request.URL.Path)
        var userRequest user.UserRequest
        jsonString, _ := io.ReadAll(request.Body)
        json.Unmarshal(jsonString, &userRequest)
        assert.Equal(t, user.UserRequest{
            Name: "the name",
            Job: "the role",
        }, userRequest)
        responseWriter.WriteHeader(http.StatusOK)
        responseWriter.Write([]byte(`{"name":"the name", "job": "the role", "id": "123"}`))
    }))
    defer server.Close()
	router := SetUpRouter(server.URL)
    
	request, _ := http.NewRequest("POST", "/users", bytes.NewBuffer([]byte(requestBody)))
	recorder := httptest.NewRecorder()
	router.ServeHTTP(recorder, request)
    
    var userResponse user.UserResponse
    myjson.BindJSON(recorder.Body, &userResponse)
	assert.Equal(t, user.UserResponse{
        Name: "the name",
        Job: "the role",
        Id: "123",
    }, userResponse)
	assert.Equal(t, http.StatusOK, recorder.Code)
}
