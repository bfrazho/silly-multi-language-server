package com.silly.multilanguage.demo.user

import org.springframework.beans.factory.annotation.Autowired
import org.springframework.beans.factory.annotation.Value
import org.springframework.stereotype.Service
import org.springframework.web.reactive.function.client.WebClient
import org.springframework.web.reactive.function.client.bodyToMono
import reactor.core.publisher.Mono

@Service
class UserService(
    @Autowired val webClient: WebClient,
    @Value("\${userEndpointBaseUrl}") val userEndpointBaseUrl: String
) {

    fun createUser(userRequest: UserRequest): Mono<UserResponse> {
        return webClient.post()
            .uri("${userEndpointBaseUrl}/users")
            .bodyValue(userRequest)
            .retrieve()
            .bodyToMono<UserResponse>()
    }
}