package br.com.gabryel.adventofcode.y2022

import br.com.gabryel.adventofcode.util.*

fun main() {
    val rocks = getRocks()

    rocks.dropUntilAbyss()
    rocks.dropUntilFilled()
}

private fun List<Pair<Int, Int>>.dropUntilAbyss() {
    val map = CaveMap.from(this)
    val dropped = generateSequence { map.dropSand() }
        .takeWhile { it.y() != 1 }
        .count()

    println("Dropped $dropped grains of sand until bottom")
}

private fun List<Pair<Int, Int>>.dropUntilFilled() {
    val map = CaveMap.from(this)
    val dropped = generateSequence { map.dropSand() }
        .takeWhile { it != map.fallingPoint }
        .count() + 1

    println("Dropped $dropped grains of sand until blocked")
}

class CaveMap(val map: Array<BooleanArray>, val fallingPoint: Coordinate) {

    private val fallDirections = listOf(0 to -1, -1 to -1, 1 to -1)

    tailrec fun dropSand(fallFrom: Coordinate = fallingPoint): Coordinate {
        val next = fallDirections.asSequence()
            .map { fallFrom + it }
            .firstOrNull { !map[it.x()][it.y()] }

        if (next == null) {
            map[fallFrom.x()][fallFrom.y()] = true
            return fallFrom
        }

        return dropSand(next)
    }

    companion object {
        fun from(rocks: List<Coordinate>): CaveMap {
            val ySize = rocks.maxOf { it.y() } + 3
            val lastYIndex = ySize - 1

            val minX = minOf(rocks.minOf { it.x() }, 500) - ySize
            val maxX = maxOf(rocks.maxOf { it.x() }, 500) + ySize

            val xSize = maxX - minX

            val map = Array(xSize) {
                BooleanArray(ySize) { false }
            }

            rocks.forEach { (x, y) ->
                map[x - minX][lastYIndex - y] = true
            }

            (0 until xSize).forEach { x ->
                map[x][0] = true
            }

            return CaveMap(map, 500 - minX to lastYIndex)
        }
    }
}

private fun getRocks() = getLinesFromSystemIn { line ->
    line.split(" -> ")
        .map { coords -> coords.split(",").map(String::toInt).let { it[0] to it[1] } }
        .windowed(2) { (left, right) ->
            (minOf(left.x(), right.x())..maxOf(left.x(), right.x())).flatMap { x ->
                (minOf(left.y(), right.y())..maxOf(left.y(), right.y())).map { y ->
                    x to y
                }
            }
        }.flatten()
}.flatten().toList()
