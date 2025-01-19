package br.com.gabryel.adventofcode.y2022.d02

fun getRPSHandStancePoints(lines: List<String>) = lines.parse().sumOf { it.handStacePoints() }

fun getRPSWinStancePoints(lines: List<String>) = lines.parse().sumOf { it.winStacePoints() }

private data class Match(val opponent: Char, val you: Char) {
    private fun getHandStanceChoicePoints() = you - 'W'

    private fun handStanceWinPoints() = when ("$opponent$you") {
        "AX", "BY", "CZ" -> 3
        "AY", "BZ", "CX" -> 6
        else -> 0
    }

    private fun winStanceWinPoints() = (you - 'X') * 3

//  private fun winStanceChoicePoints() = (((opChar - 'A') + (youChar - 'Y')).mod(3)) + 1
    private fun winStanceChoicePoints() = when ("$opponent$you") {
        "AY", "BX", "CZ" -> 1
        "BY", "CX", "AZ" -> 2
        else -> 3
    }

    fun handStacePoints() = getHandStanceChoicePoints() + handStanceWinPoints()

    fun winStacePoints() = winStanceChoicePoints() + winStanceWinPoints()
}

private fun List<String>.parse() = map {
    val (opponent, you) = it.split(" ")
    Match(opponent.first(), you.first())
}
