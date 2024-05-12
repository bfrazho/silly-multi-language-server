package com.silly.multilanguage.demo.controller


data class UserRequest(
    val name: String,
    val job: String
)

data class UserResponse(
    val name: String,
    val job: String,
    val id: String
)

