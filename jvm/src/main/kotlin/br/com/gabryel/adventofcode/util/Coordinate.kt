package br.com.gabryel.adventofcode.util

import kotlin.math.absoluteValue

typealias Coordinate = Pair<Int, Int>

inline fun Coordinate.x() = first

inline fun Coordinate.y() = second

inline infix fun Coordinate.getManhattanDistance(other: Coordinate) = (x() - other.x()).absoluteValue + (y() - other.y()).absoluteValue

inline infix fun Coordinate.distanceTo(other: Coordinate) = (x() - other.x()) to (y() - other.y())

inline operator fun Coordinate.plus(other: Coordinate) = (x() + other.x()) to (y() + other.y())

inline operator fun Coordinate.plus(other: Direction) = this + other.vector

inline operator fun Coordinate.times(other: Int) = (x() * other) to (y() * other)

inline operator fun Coordinate.times(other: Long) = (x() * other).toInt() to (y() * other).toInt()

inline operator fun Coordinate.div(other: Int) = (x() / other) to (y() / other)

inline operator fun Coordinate.rem(other: Int) = (x() % other) to (y() % other)

inline operator fun Coordinate.rem(other: Long) = (x() % other).toInt() to (y() % other).toInt()

inline fun Coordinate.getAdjacent() = Direction.entries.map { this + it }

inline infix fun Coordinate.bindTo(dimensions: Coordinate) = (x().mod(dimensions.x())) to (y().mod(dimensions.y()))

inline operator fun Coordinate.contains(other: Coordinate) = other.x() in (0 until x()) && other.y() in (0 until y())

val ZERO = 0 to 0
