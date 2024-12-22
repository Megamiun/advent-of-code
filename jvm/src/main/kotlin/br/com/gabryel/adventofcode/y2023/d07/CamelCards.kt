package br.com.gabryel.adventofcode.y2023.d07

import br.com.gabryel.adventofcode.util.readLines

private val order = (listOf('A', 'K', 'Q', 'J', 'T') + ('9' downTo '2'))
    .mapIndexed { index, card -> card to index }
    .toMap()

private val orderWithJoker = (listOf('A', 'K', 'Q', 'T') + ('9' downTo '2') + 'J')
    .mapIndexed { index, card -> card to index }
    .toMap()

fun main() {
    listOf("sample", "input").forEach { file ->
        val games = readLines(2023, 7, file)
            .map { it.split(" ") }
            .map { (cards, bet) -> cards to bet.toInt() }

        println("[Ranked Bets            ][$file] ${findRankedSum(games, ::findWinLevel, order)}")
        println("[Ranked Bets With Jokers][$file] ${findRankedSum(games, ::findWinLevelWithJokers, orderWithJoker)}")
    }
}

private fun findRankedSum(games: List<Pair<String, Int>>, findWinLevel: String.() -> Int, cardMapper: Map<Char, Int>) =
    games.sortedByDescending { (cards) ->
        val level = cards.findWinLevel()
        val normalizedByStrength = cards.normalizeByStrength(cardMapper)

        "$level $normalizedByStrength"
    }.mapIndexed { index, (_, bet) -> bet * (index + 1) }
        .sum()

private fun findWinLevel(cards: String): Int {
    val sorted = cards.groupingBy { it }.eachCount().values.sortedDescending()
    val first = sorted[0]
    val second = sorted.getOrElse(1) { 0 }

    return defineWinLevel(first, second)
}

private fun findWinLevelWithJokers(cards: String): Int {
    val sorted = cards.groupingBy { it }.eachCount().filter { it.key != 'J' }.values.sortedDescending()
    val jokers = cards.count { it == 'J' }

    val firstWithJokers = sorted.getOrElse(0) { 0 } + jokers
    val second = sorted.getOrElse(1) { 0 }

    return defineWinLevel(firstWithJokers, second)
}

private fun defineWinLevel(first: Int, second: Int) =
    when (first) {
        5 -> 0
        4 -> 1
        3 -> if (second == 2) 2 else 3
        2 -> if (second == 2) 4 else 5
        else -> 6
    }

private fun String.normalizeByStrength(mapper: Map<Char, Int>) = map(mapper::get)
    .joinToString("") { "$it".padStart(2, '0') }
