package server

import (
	"bytes"
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
        myjson.BindJSON(request.Body, &userRequest)
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

func TestFailedToSerializeInputForPostUserReturns400(t *testing.T) {
    badRequestBody := `{`
    router := SetUpRouter("http://localhost:8080")
    
    request, _ := http.NewRequest("POST", "/users", bytes.NewBuffer([]byte(badRequestBody)))
	recorder := httptest.NewRecorder()
	router.ServeHTTP(recorder, request)
    
    assert.Equal(t, http.StatusBadRequest, recorder.Code)
    output, _ := io.ReadAll(recorder.Body)
    assert.Equal(t, "unexpected EOF", string(output))
}

func TestFailedToCallEndpointReturns500(t *testing.T) {
	requestBody := `{"name": "the name", "job": "the role"}`
	router := SetUpRouter("http://localhost:100000000000000000000")
    
	request, _ := http.NewRequest("POST", "/users", bytes.NewBuffer([]byte(requestBody)))
	recorder := httptest.NewRecorder()
	router.ServeHTTP(recorder, request)
    

    assert.Equal(t, http.StatusInternalServerError, recorder.Code)
    output, _ := io.ReadAll(recorder.Body)
    assert.Equal(t, "Post \"http://localhost:100000000000000000000/users\": dial tcp: address 100000000000000000000: invalid port", string(output))
}