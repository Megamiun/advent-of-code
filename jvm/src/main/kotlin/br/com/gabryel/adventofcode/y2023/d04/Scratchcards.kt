package br.com.gabryel.adventofcode.y2023.d04

import br.com.gabryel.adventofcode.readLines
import kotlin.math.pow

typealias Card = Triple<Int, List<Int>, List<Int>>

fun main() {
    listOf("sample1", "input").forEach { file ->
        val cards = readLines(2023, 4, file)

        println("[Card Power    ][$file] ${sumCardsValues(cards)}")
        println("[Card Recursive][$file] ${sumCardsRecursive(cards)}")
    }
}

private val regex = "(\\d+): (.*) \\| (.*)".toRegex()

private fun sumCardsValues(cards: List<String>) = cards
    .map(String::getCard)
    .map { (_, player, winning) -> player.getIntersectionCountWith(winning) }
    .filter { it != 0 }
    .sumOf { 2.0.pow(it.toDouble() - 1) }
    .toInt()

private fun sumCardsRecursive(cards: List<String>): Int {
    val convertedCards = cards.map(String::getCard)
    return sumCardsRecursive(convertedCards)
}

private fun sumCardsRecursive(remaining: List<Card>, cache: MutableMap<Int, Int> = mutableMapOf()): Int {
    val (gameNum, player, winning) = remaining.firstOrNull()
        ?: return 0

    val tail = remaining.drop(1)
    val sum = sumCardsRecursive(tail, cache)

    val cardsBelow = player.getIntersectionCountWith(winning)
    val cardsBelowToTake = minOf(cardsBelow, remaining.size)
    val repeatedCardsValue = tail.take(cardsBelowToTake)
        .sumOf { (gameNum) -> cache.getOrDefault(gameNum, 0) }

    cache[gameNum] = repeatedCardsValue + 1
    return sum + repeatedCardsValue + 1
}

private fun List<Int>.getIntersectionCountWith(winning: List<Int>) =
    count { winning.contains(it) }

private fun String.getCard(): Card {
    val (gameNumber, player, winning) = regex.find(this)?.groupValues?.drop(1)
        ?: throw IllegalStateException("Data not found")

    return Triple(gameNumber.toInt(), player.extractCards(), winning.extractCards())
}

private fun String.extractCards() = split(" ")
    .filter { it.isNotBlank() }
    .map { it.trim().toInt() }
