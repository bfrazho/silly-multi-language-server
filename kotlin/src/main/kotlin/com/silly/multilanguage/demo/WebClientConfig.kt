package com.silly.multilanguage.demo

import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.web.reactive.function.client.WebClient

@Configuration
class WebClientConfig {
    @Bean
    fun defaultWebClient(): WebClient {
        return WebClient.builder().build()
    }
}
