package br.com.gabryel.adventofcode.y2023.d01

import br.com.gabryel.adventofcode.readLines

fun main() {
    listOf("sample1", "sample2", "input").forEach {
        val lines = readLines(2023, 1, it)

        val result = lines.sumCalibrations()
        println("[$it]: $result")
    }
}

private val substitutions = listOf(
    "one" to 1,
    "two" to 2,
    "three" to 3,
    "four" to 4,
    "five" to 5,
    "six" to 6,
    "seven" to 7,
    "eight" to 8,
    "nine" to 9
)

private fun List<String>.sumCalibrations() =
    map(String::get_calibration_value).sum()

private fun String.get_calibration_value(): Int {
    val numbers = windowed(5, partialWindows = true) { window ->
        window.first().digitToIntOrNull() ?:
            substitutions.firstOrNull { (key) -> window.startsWith(key) }?.second
    }.filterNotNull()

    return (numbers.first() * 10) + numbers.last()
}
