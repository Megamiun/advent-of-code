package br.com.gabryel.adventofcode.y2023.d01

private val SUBSTITUTIONS = listOf(
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

fun sumTrebuchetCalibrations(lines: List<String>) =
    lines.sumOf(String::getCalibrationValue)

fun sumTrebuchetCalibrationsWithWrittenNumbers(lines: List<String>) =
    lines.sumOf { it.getCalibrationValue(SUBSTITUTIONS) }

private fun String.getCalibrationValue(substitutions: List<Pair<String, Int>> = emptyList()): Int {
    val numbers = windowed(5, partialWindows = true) { window ->
        window.first().digitToIntOrNull()
            ?: substitutions.firstOrNull { (key) -> window.startsWith(key) }?.second
    }.filterNotNull()

    return (numbers.first() * 10) + numbers.last()
}