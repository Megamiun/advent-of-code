package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.getLinesFromSystemIn

fun main() {
    val matches = getAllMatches()
    println("Total Hand Stance Points: ${matches.sumOf { it.handStacePoints }}")
    println("Total Win Stance Points: ${matches.sumOf { it.winStacePoints }}")
}

private fun getAllMatches() = getLinesFromSystemIn { line ->
    val (opponent, you) = line.split(" ")
    Match(opponent, you)
}.toList()

private data class Match(val opponent: String, val you: String) {
    private val youChar = you.first()

    private val handStanceChoicePoints = youChar - 'W'

    private val handStanceWinPoints = when ("$opponent$you") {
        "AX", "BY", "CZ" -> 3
        "AY", "BZ", "CX" -> 6
        else -> 0
    }

    private val winStanceWinPoints = (youChar - 'X') * 3

//    private val winStanceChoicePoints = (((opChar - 'A') + (youChar - 'Y')).mod(3)) + 1
    private val winStanceChoicePoints = when ("$opponent$you") {
        "AY", "BX", "CZ" -> 1
        "BY", "CX", "AZ" -> 2
        else -> 3
    }

    val handStacePoints = handStanceChoicePoints + handStanceWinPoints

    val winStacePoints = winStanceChoicePoints + winStanceWinPoints
}
