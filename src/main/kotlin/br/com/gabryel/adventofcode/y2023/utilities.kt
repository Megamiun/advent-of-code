package br.com.gabryel.adventofcode.y2023

import java.time.LocalDateTime

fun logTimed(message: String) {
    println("[${LocalDateTime.now()}] $message")
}

fun IntRange.size() = (last - first) + 1

fun LongRange.size() = (last - first) + 1

fun IntRange.intersects(other: IntRange) =
    first in other || last in other || other.first in this || other.last in this

fun LongRange.intersects(other: LongRange) =
    first in other || last in other || other.first in this || other.last in this

fun IntRange.displace(num: Int) = first + num .. last + num
