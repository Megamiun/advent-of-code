package br.com.gabryel.adventofcode.util

import kotlin.math.absoluteValue

typealias Coordinate = Pair<Int, Int>

typealias CharMap = Array<CharArray>

fun CharMap.findAdjacent(coordinate: Coordinate) =
    Direction.entries
        .map { dir -> (coordinate + dir.vector) to (this[coordinate + dir.vector] to dir) }

operator fun CharMap.get(coord: Coordinate) = getOrNull(coord.y())?.getOrNull(coord.x())

fun Coordinate.x() = first
fun Coordinate.y() = second

infix fun Coordinate.getManhattanDistance(other: Coordinate) = (x() - other.x()).absoluteValue + (y() - other.y()).absoluteValue

infix fun Coordinate.distanceTo(other: Coordinate) = (x() - other.x()) to (y() - other.y())

operator fun Coordinate.plus(other: Coordinate) = (x() + other.x()) to (y() + other.y())

operator fun Coordinate.plus(other: Direction) = this + other.vector

operator fun Coordinate.times(other: Int) = (x() * other) to (y() * other)

operator fun Coordinate.times(other: Long) = (x() * other).toInt() to (y() * other).toInt()

operator fun Coordinate.rem(other: Int) = (x() % other) to (y() % other)

operator fun Coordinate.rem(other: Long) = (x() % other).toInt() to (y() % other).toInt()

fun Coordinate.getAdjacent() = Direction.entries.map { this + it.vector }

infix fun Coordinate.bindTo(dimensions: Coordinate) = (x().mod(dimensions.x())) to (y().mod(dimensions.y()))

operator fun Coordinate.contains(other: Coordinate) = other.x() in (0 until x()) && other.y() in (0 until y())

val ZERO = 0 to 0
