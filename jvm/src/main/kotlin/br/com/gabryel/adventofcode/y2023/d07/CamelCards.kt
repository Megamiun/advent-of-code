package br.com.gabryel.adventofcode.y2023.d07

private val order = (listOf('A', 'K', 'Q', 'J', 'T') + ('9' downTo '2'))
    .mapIndexed { index, card -> card to index }
    .toMap()

private val orderWithJoker = (listOf('A', 'K', 'Q', 'T') + ('9' downTo '2') + 'J')
    .mapIndexed { index, card -> card to index }
    .toMap()

fun findSimple(lines: List<String>) =
    lines.parse().findRankedSum(::findWinLevel, order)

fun findJokers(lines: List<String>) =
    lines.parse().findRankedSum(::findWinLevelWithJokers, orderWithJoker)

private fun List<Pair<String, Int>>.findRankedSum(findWinLevel: String.() -> Int, cardMapper: Map<Char, Int>) =
    sortedByDescending { (cards) ->
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

private fun List<String>.parse() =
    map { it.split(" ") }.map { (cards, bet) -> cards to bet.toInt() }
