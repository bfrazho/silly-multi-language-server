package com.silly.multilanguage.demo.controller

import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RestController
import reactor.core.publisher.Mono

@RestController
class UserController {
    @PostMapping("/user")
    fun createUser(@RequestBody user: UserRequest): Mono<UserResponse> {
        return Mono.from{ UserResponse("","","") }
    }
}