package com.silly.multilanguage.demo

import com.fasterxml.jackson.databind.ObjectMapper
import com.github.tomakehurst.wiremock.client.WireMock
import com.github.tomakehurst.wiremock.junit5.WireMockRuntimeInfo
import com.github.tomakehurst.wiremock.junit5.WireMockTest
import com.github.tomakehurst.wiremock.matching.EqualToJsonPattern
import com.github.tomakehurst.wiremock.matching.RequestPatternBuilder
import com.marcinziolo.kotlin.wiremock.equalTo
import com.marcinziolo.kotlin.wiremock.post
import com.marcinziolo.kotlin.wiremock.returnsJson
import com.silly.multilanguage.demo.user.UserRequest
import com.silly.multilanguage.demo.user.UserResponse
import org.junit.jupiter.api.Test
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.context.SpringBootTest
import org.springframework.test.context.ActiveProfiles
import org.springframework.test.web.reactive.server.WebTestClient
import org.springframework.test.web.reactive.server.expectBody

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
@ActiveProfiles("test")
@WireMockTest(httpPort = 9999)
class UserControllerTest(@Autowired val client: WebTestClient) {

    val objectMapper: ObjectMapper = ObjectMapper()

    fun setupStub(wireMock: WireMock, userResponse: UserResponse) {
        wireMock.post {
            url equalTo "/users"
        } returnsJson {
            body = objectMapper.writeValueAsString(userResponse)
        }
    }

    @Test
    fun `can call rest endpoint for creating a user`(wireMockInfo: WireMockRuntimeInfo) {
        val userRequest = UserRequest("someone", "role")
        val userResponse = UserResponse("someone", "role", "100")
        setupStub(wireMockInfo.wireMock, userResponse)

        client.post()
            .uri("/users")
            .bodyValue(userRequest)
            .exchange()
            .expectStatus().isOk.expectBody<UserResponse>()
            .isEqualTo(userResponse)

        wireMockInfo.wireMock.verifyThat(
            RequestPatternBuilder()
                .withUrl("/users")
                .withRequestBody(EqualToJsonPattern(objectMapper.writeValueAsString(userRequest), false, false))
        )
    }

}
