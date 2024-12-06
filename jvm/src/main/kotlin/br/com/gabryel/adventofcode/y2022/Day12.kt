package br.com.gabryel.adventofcode.y2022

import java.util.*
import java.util.Comparator.comparingInt

fun main() {
    val map = getMap()

    map.getMinimumDistanceFromStart()
    map.getMinimumDistanceFromA()
}

private fun List<List<Char>>.getMinimumDistanceFromA() {
    val result = getMinimumDistanceToTop('a')
    println("Minimum distance from a to E is $result")
}

private fun List<List<Char>>.getMinimumDistanceFromStart() {
    val result = getMinimumDistanceToTop('S')
    println("Minimum distance from S to E is $result")
}

private fun List<List<Char>>.getMinimumDistanceToTop(searched: Char): Int {
    val visited = mutableSetOf<Coordinate>()

    val toVisit = PriorityQueue<Pair<Int, Coordinate>>(comparingInt { it.first })
    toVisit.add(0 to findCharacter('E'))

    while (true) {
        val (distance, current) = toVisit.remove()

        if (current in visited) continue
        visited.add(current)

        val currentCharacter = getAtSafe(current) ?: continue
        val currentAltitude = currentCharacter.getAltitude()
        if (currentCharacter == searched) return distance

        val valid = current.getAdjacent()
            .filter { next -> getAtSafe(next)?.getAltitude()?.let { it + 1 >= currentAltitude } ?: false }
            .map { distance + 1 to it }

        toVisit.addAll(valid)
    }
}

private fun Char.getAltitude() =
    when (this) {
        'E' -> 'z'
        'S' -> 'a'
        else -> this
    }

private fun List<List<Char>>.getAtSafe(current: Coordinate) =
    getOrNull(current.y())?.getOrNull(current.x())

private fun List<List<Char>>.findCharacter(searched: Char) =
    withIndex().flatMap { (y, line) ->
        line.withIndex()
            .filter { (_, character) -> character == searched }
            .map { (x) -> x to y }
    }.first()

private fun getMap() = getLines {
    it.toCharArray().toList()
}.toList()
