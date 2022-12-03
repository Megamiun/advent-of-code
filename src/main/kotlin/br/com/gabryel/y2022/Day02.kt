package br.com.gabryel.y2022

import java.util.Scanner

private val scanner = Scanner(System.`in`)

fun main() {
    val matches = getAllMatches()
    println("Total Hand Stance Points: ${matches.sumOf { it.handStacePoints }}")
    println("Total Win Stance Points: ${matches.sumOf { it.winStacePoints }}")
}

private fun getAllMatches() = generateSequence(::getNextMatch).toList()

private fun getNextMatch(): Match? {
    val line = (if (scanner.hasNextLine()) scanner.nextLine().ifEmpty { null } else null)
        ?: return null

    val (opponent, you) = line.split(" ")
    return Match(opponent, you)
}

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
