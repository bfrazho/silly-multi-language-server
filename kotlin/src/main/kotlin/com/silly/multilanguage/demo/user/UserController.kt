package com.silly.multilanguage.demo.user

import org.springframework.beans.factory.annotation.Autowired
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RestController
import reactor.core.publisher.Mono

@RestController
class UserController(@Autowired val userService: UserService) {
    @PostMapping("/users")
    fun createUser(@RequestBody user: UserRequest): Mono<UserResponse> {
        return userService.createUser(user)
    }
}