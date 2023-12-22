package br.com.gabryel.adventofcode.y2023

import java.time.LocalDateTime

fun logTimed(message: String) {
    println("[${LocalDateTime.now()}] $message")
}

fun IntRange.size() = (last - first) + 1

fun LongRange.size() = (last - first) + 1
